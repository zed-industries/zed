// Copyright 2006-2008 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_BASE_PATHS_MAC_H_
#define BASE_BASE_PATHS_MAC_H_

// This file declares Mac-specific path keys for the base module.
// These can be used with the PathService to access various special
// directories and files.

namespace base {

enum {
  PATH_MAC_START = 200,

  DIR_APP_DATA,  // ~/Library/Application Support
                 // Data for specific applications is stored in subdirectories.

  PATH_MAC_END
};

}  // namespace base

#endif  // BASE_BASE_PATHS_MAC_H_
