// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_TEXT_OPTIONS_H_
#define UPB_TEXT_OPTIONS_H_

enum {
  // When set, prints everything on a single line.
  UPB_TXTENC_SINGLELINE = 1,

  // When set, unknown fields are not printed.
  UPB_TXTENC_SKIPUNKNOWN = 2,

  // When set, maps are *not* sorted (this avoids allocating tmp mem).
  UPB_TXTENC_NOSORT = 4
};

#endif  // UPB_TEXT_OPTIONS_H_
