// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_ACCESS_CONTROL_LIST_H_
#define BASE_WIN_ACCESS_CONTROL_LIST_H_

#include <stdint.h>

#include <memory>
#include <optional>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/heap_array.h"
#include "base/win/sid.h"
#include "base/win/windows_types.h"

namespace base::win {

// Represents the type of access operation to perform on an ACL.
enum class SecurityAccessMode { kGrant, kSet, kDeny, kRevoke };

// Class to represent an entry to modify the ACL.
class BASE_EXPORT ExplicitAccessEntry {
 public:
  ExplicitAccessEntry(const Sid& sid,
                      SecurityAccessMode mode,
                      DWORD access_mask,
                      DWORD inheritance);
  ExplicitAccessEntry(WellKnownSid known_sid,
                      SecurityAccessMode mode,
                      DWORD access_mask,
                      DWORD inheritance);
  ExplicitAccessEntry(const ExplicitAccessEntry&) = delete;
  ExplicitAccessEntry& operator=(const ExplicitAccessEntry&) = delete;
  ExplicitAccessEntry(ExplicitAccessEntry&&);
  ExplicitAccessEntry& operator=(ExplicitAccessEntry&&);
  ~ExplicitAccessEntry();

  const Sid& sid() const LIFETIME_BOUND { return sid_; }
  SecurityAccessMode mode() const { return mode_; }
  DWORD access_mask() const { return access_mask_; }
  DWORD inheritance() const { return inheritance_; }
  // Clones the entry.
  ExplicitAccessEntry Clone() const;

 private:
  Sid sid_;
  SecurityAccessMode mode_;
  DWORD access_mask_;
  DWORD inheritance_;
};

// This class is used to hold and modify Windows ACLs. An AccessControlList
// object can contain a null ACL which grants everyone access to a resource. A
// null ACL is distinct from an empty ACL which grants no-one access. An empty
// ACL is the default when constructing a new instance.
class BASE_EXPORT AccessControlList {
 public:
  // Create from an existing ACL pointer.
  // |acl| The ACL pointer. Passing nullptr will create a null ACL.
  static std::optional<AccessControlList> FromPACL(ACL* acl);

  // Create an AccessControlList from a mandatory label.
  // |integrity_level| is the integrity level for the label.
  // |inheritance| inheritance flags.
  // |mandatory_policy| is the policy, e.g. SYSTEM_MANDATORY_LABEL_NO_WRITE_UP.
  static std::optional<AccessControlList> FromMandatoryLabel(
      DWORD integrity_level,
      DWORD inheritance,
      DWORD mandatory_policy);

  AccessControlList();
  AccessControlList(const AccessControlList&) = delete;
  AccessControlList& operator=(const AccessControlList&) = delete;
  AccessControlList(AccessControlList&&);
  AccessControlList& operator=(AccessControlList&&);
  ~AccessControlList();

  // Set one or more entry in the ACL.
  // |entries| the list of entries to set in the ACL.
  // Returns true if successful, false on error, with the Win32 last error set.
  bool SetEntries(const std::vector<ExplicitAccessEntry>& entries);

  // Set one entry in the ACL.
  // |sid| the SID for the entry.
  // |mode| the operation to perform on the ACL, e.g. grant access.
  // |access_mask| the entries access mask.
  // |inheritance| inheritance flags.
  // Returns true if successful, false on error, with the Win32 last error set.
  bool SetEntry(const Sid& sid,
                SecurityAccessMode mode,
                DWORD access_mask,
                DWORD inheritance);

  // Make a clone of the current AccessControlList object.
  AccessControlList Clone() const;

  // Clear all entries in the AccessControlList.
  void Clear();

  // Returns the AccessControlList as a ACL*. The AccessControlList object
  // retains owenership of the pointer. This can return nullptr if the ACL is
  // null.
  ACL* get() { return reinterpret_cast<ACL*>(acl_.data()); }
  const ACL* get() const { return reinterpret_cast<const ACL*>(acl_.data()); }

  // Returns whether the AccessControlList is considered a null ACL.
  bool is_null() const {
    // Note: there is a distinction between null ACL (ACL that occupies 0 bytes)
    // and an empty ACL (an ACL that occupies sizeof(ACL) bytes and has no ACEs
    // following it). If the underlying storage is empty, it means that we're
    // dealing with a null ACL.
    return acl_.empty();
  }

 private:
  explicit AccessControlList(const ACL* acl);
  base::HeapArray<uint8_t> acl_;
};

}  // namespace base::win

#endif  // BASE_WIN_ACCESS_CONTROL_LIST_H_
