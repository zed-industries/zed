// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SECURITY_UTIL_H_
#define BASE_WIN_SECURITY_UTIL_H_

#include <optional>
#include <vector>

#include "base/base_export.h"
#include "base/win/sid.h"
#include "base/win/windows_types.h"

namespace base {

class FilePath;

namespace win {

// Adds allowed ACE entries to a file or directory |path| from a list of SIDs
// with allowed |access_mask| and |inheritance| flags. If |path| is a directory
// and |recursive| is true then any inheritable ACEs granted will be propagated
// to its children.
BASE_EXPORT bool GrantAccessToPath(const FilePath& path,
                                   const std::vector<Sid>& sids,
                                   DWORD access_mask,
                                   DWORD inheritance,
                                   bool recursive = true);

// Adds deny ACE entries to a file or directory |path| from a list of SIDs with
// allowed |access_mask| and |inheritance| flags. If |path| is a directory and
// |recursive| is true then any inheritable ACEs granted will be propagated to
// its children.
BASE_EXPORT bool DenyAccessToPath(const FilePath& path,
                                  const std::vector<Sid>& sids,
                                  DWORD access_mask,
                                  DWORD inheritance,
                                  bool recursive = true);

// Checks if a list of SIDs has the specified |access_mask| and
// |inheritance| flags for a file or directory |path|.
BASE_EXPORT bool HasAccessToPath(const FilePath& path,
                                 const std::vector<Sid>& sids,
                                 DWORD access_mask,
                                 DWORD inheritance);

// Clone a vector of Sids.
BASE_EXPORT std::vector<Sid> CloneSidVector(const std::vector<Sid>& sids);

// Append a vector of Sids to an existing vector.
BASE_EXPORT void AppendSidVector(std::vector<Sid>& base_sids,
                                 const std::vector<Sid>& append_sids);

// Gets the granted access for an open handle.
// |handle| specifies any kernel object handle to query.
BASE_EXPORT std::optional<ACCESS_MASK> GetGrantedAccess(HANDLE handle);

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SECURITY_UTIL_H_
