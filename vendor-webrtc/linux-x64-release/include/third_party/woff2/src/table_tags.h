/* Copyright 2014 Google Inc. All Rights Reserved.

   Distributed under MIT license.
   See file LICENSE for detail or copy at https://opensource.org/licenses/MIT
*/

/* Font table tags */

#ifndef WOFF2_TABLE_TAGS_H_
#define WOFF2_TABLE_TAGS_H_

#include <inttypes.h>

namespace woff2 {

// Tags of popular tables.
static const uint32_t kGlyfTableTag = 0x676c7966;
static const uint32_t kHeadTableTag = 0x68656164;
static const uint32_t kLocaTableTag = 0x6c6f6361;
static const uint32_t kDsigTableTag = 0x44534947;
static const uint32_t kCffTableTag = 0x43464620;
static const uint32_t kHmtxTableTag = 0x686d7478;
static const uint32_t kHheaTableTag = 0x68686561;
static const uint32_t kMaxpTableTag = 0x6d617870;

extern const uint32_t kKnownTags[];

} // namespace woff2

#endif  // WOFF2_TABLE_TAGS_H_
