// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SECURITY_DESCRIPTOR_H_
#define BASE_WIN_SECURITY_DESCRIPTOR_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/files/file_path.h"
#include "base/win/access_control_list.h"
#include "base/win/access_token.h"
#include "base/win/sid.h"
#include "base/win/windows_types.h"

namespace base::win {

// Represents the type of Windows kernel object for reading/writing the security
// descriptor.
enum class SecurityObjectType {
  kFile,
  kRegistry,
  kWindowStation,
  kDesktop,
  kKernel
};

// Results from the access check.
struct AccessCheckResult {
  // The granted access from the check.
  ACCESS_MASK granted_access;
  // The access status. Set to true if the access check was successful.
  bool access_status;
};

// This class is used to hold and modify a Windows security descriptor.
class BASE_EXPORT SecurityDescriptor {
 public:
  class BASE_EXPORT SelfRelative {
   public:
    friend SecurityDescriptor;

    SelfRelative(const SelfRelative&);
    ~SelfRelative();

    size_t size() const { return sd_.size(); }
    PSECURITY_DESCRIPTOR get() const {
      return const_cast<uint8_t*>(sd_.data());
    }

   private:
    explicit SelfRelative(std::vector<uint8_t>&& sd);

    std::vector<uint8_t> sd_;
  };

  // Create from an existing security descriptor pointer.
  // |security_descriptor| The pointer to a self-relative or absolute security
  // descriptor. This method will copy all security descriptor data.
  static std::optional<SecurityDescriptor> FromPointer(
      PSECURITY_DESCRIPTOR security_descriptor);

  // Create from the security descriptor of an existing file.
  // |path| the path to the file.
  // |security_info| indicates what parts to read.
  static std::optional<SecurityDescriptor> FromFile(
      const base::FilePath& path,
      SECURITY_INFORMATION security_info);

  // Create from the security descriptor of a named Windows object.
  // |name| the name of the object using the format specified for the
  // GetNamedSecurityInfo API.
  // |object_type| specifies the type of object the name represents.
  // |security_info| indicates what parts to read.
  static std::optional<SecurityDescriptor> FromName(
      const std::wstring& name,
      SecurityObjectType object_type,
      SECURITY_INFORMATION security_info);

  // Create from the security descriptor of a kernel object.
  // |handle| the object handle. It must have READ_CONTROL access.
  // |object_type| specifies the type of object the handle represents.
  // |security_info| indicates what parts to read.
  static std::optional<SecurityDescriptor> FromHandle(
      HANDLE handle,
      SecurityObjectType object_type,
      SECURITY_INFORMATION security_info);

  // Create from a string representation of a security descriptor.
  // |sddl| the security descriptor in SDDL format.
  static std::optional<SecurityDescriptor> FromSddl(const std::wstring& sddl);

  SecurityDescriptor();
  SecurityDescriptor(const SecurityDescriptor&) = delete;
  SecurityDescriptor& operator=(const SecurityDescriptor&) = delete;
  SecurityDescriptor(SecurityDescriptor&&);
  SecurityDescriptor& operator=(SecurityDescriptor&&);
  ~SecurityDescriptor();

  // Write the security descriptor to a file.
  // |path| specifies the path to the file.
  // |security_info| indicates what parts to write.
  bool WriteToFile(const base::FilePath& path,
                   SECURITY_INFORMATION security_info) const;

  // Write the security descriptor to a named kernel object.
  // |name| the name of the object using the format specified for the
  // SetNamedSecurityInfo API.
  // |object_type| specifies the type of object name represents.
  // |security_info| indicates what parts to write.
  bool WriteToName(const std::wstring& name,
                   SecurityObjectType object_type,
                   SECURITY_INFORMATION security_info) const;

  // Write the SecurityDescriptor to a kernel object.
  // |handle| the handle to the object. Must have WRITE_DAC and/or WRITE_OWNER
  // access depending of the parts specified with |security_info|. |object_type|
  // specifies the type of object the handle represents. Use kKernel for
  // undefined types. |security_info| indicates what parts to write.
  bool WriteToHandle(HANDLE handle,
                     SecurityObjectType object_type,
                     SECURITY_INFORMATION security_info) const;

  // Convert the SecurityDescriptor to an SDDL string.
  // |security_info| determines what parts are included in the string.
  std::optional<std::wstring> ToSddl(SECURITY_INFORMATION security_info) const;

  // Create an reference to the absolute security descriptor of this instance.
  // |sd| the SECURITY_DESCRIPTOR structure to populate. This is is only valid
  // as long as this object is in scope and not modified.
  void ToAbsolute(SECURITY_DESCRIPTOR& sd);

  // Create a self-relative security descriptor in a single buffer.
  std::optional<SelfRelative> ToSelfRelative() const;

  // Make a clone of the current security descriptor object.
  SecurityDescriptor Clone() const;

  // Set the mandatory label in the security descriptor. Note that calling
  // this will completely replace the SACL.
  // |integrity_level| is the integrity level for the label.
  // |inheritance| specify the flags for inheritance.
  // |mandatory_policy| is the policy, e.g. SYSTEM_MANDATORY_LABEL_NO_WRITE_UP.
  bool SetMandatoryLabel(DWORD integrity_level,
                         DWORD inheritance,
                         DWORD mandatory_policy);

  // Set one or more entry in the DACL.
  // |entries| the list of entries to set in the ACL.
  // Returns true if successful, false on error, with the Win32 last error set.
  // If DACL is not present a NULL ACL will be added first.
  bool SetDaclEntries(const std::vector<ExplicitAccessEntry>& entries);

  // Set one entry in the DACL.
  // |sid| the SID for the entry.
  // |mode| the operation to perform on the ACL, e.g. grant access.
  // |access_mask| the entries access mask.
  // |inheritance| inheritance flags.
  // Returns true if successful, false on
  // error, with the Win32 last error set.
  // If DACL is not present a NULL ACL will be added first.
  bool SetDaclEntry(const Sid& sid,
                    SecurityAccessMode mode,
                    DWORD access_mask,
                    DWORD inheritance);

  // Set one entry in the DACL.
  // |known_sid| the known SID for the entry.
  // |mode| the operation to perform on the ACL, e.g. grant access.
  // |access_mask| the entries access mask.
  // |inheritance| inheritance flags.
  // Returns true if successful, false on
  // error, with the Win32 last error set.
  // If DACL is not present a NULL ACL will be added first.
  bool SetDaclEntry(WellKnownSid known_sid,
                    SecurityAccessMode mode,
                    DWORD access_mask,
                    DWORD inheritance);

  // Perform an access check for this security descriptor.
  // |token| specify the impersonation token to check against.
  // |desired_access| the access desired for the check.
  // |generic_mapping| the generic mapping for the access check.
  // Returns the result of the access check. If an empty result is returned the
  // call to the AccessCheck API failed.
  std::optional<AccessCheckResult> AccessCheck(
      const AccessToken& token,
      ACCESS_MASK desired_access,
      const GENERIC_MAPPING& generic_mapping);

  // Perform an access check for this security descriptor.
  // |token| specify the impersonation token to check against.
  // |desired_access| the access desired for the check.
  // |object_type| the object type to determine how to map generic rights. Note
  // that you can't use kKernel as that doesn't reflect a specific kernel object
  // type, an empty return will be return if this is used. If you need to access
  // check an unsupported type use the overload which accepts a manually
  // configured GENERIC_MAPPING.
  // Returns the result of the access check. If an empty result is returned the
  // call to the AccessCheck API failed.
  std::optional<AccessCheckResult> AccessCheck(const AccessToken& token,
                                               ACCESS_MASK desired_access,
                                               SecurityObjectType object_type);

  // Get, set and clear owner member.
  const std::optional<Sid>& owner() const { return owner_; }
  std::optional<Sid>& owner() { return owner_; }
  void set_owner(const Sid& owner) { owner_ = owner.Clone(); }
  void clear_owner() { owner_ = std::nullopt; }

  // Get, set and clear group member.
  const std::optional<Sid>& group() const { return group_; }
  std::optional<Sid>& group() { return group_; }
  void set_group(const Sid& group) { group_ = group.Clone(); }
  void clear_group() { group_ = std::nullopt; }

  // Get, set and clear dacl member.
  const std::optional<AccessControlList>& dacl() const { return dacl_; }
  std::optional<AccessControlList>& dacl() { return dacl_; }
  void set_dacl(const AccessControlList& dacl) { dacl_ = dacl.Clone(); }
  void clear_dacl() { dacl_ = std::nullopt; }

  // Get and set dacl_protected member.
  bool dacl_protected() const { return dacl_protected_; }
  void set_dacl_protected(bool dacl_protected) {
    dacl_protected_ = dacl_protected;
  }

  // Get, set and clear sacl member.
  const std::optional<AccessControlList>& sacl() const { return sacl_; }
  std::optional<AccessControlList>& sacl() { return sacl_; }
  void set_sacl(const AccessControlList& sacl) { sacl_ = sacl.Clone(); }
  void clear_sacl() { sacl_ = std::nullopt; }

  // Get and set sacl_protected member.
  bool sacl_protected() const { return sacl_protected_; }
  void set_sacl_protected(bool sacl_protected) {
    sacl_protected_ = sacl_protected;
  }

 private:
  SecurityDescriptor(std::optional<Sid>&& owner,
                     std::optional<Sid>&& group,
                     std::optional<AccessControlList>&& dacl,
                     bool dacl_protected,
                     std::optional<AccessControlList>&& sacl,
                     bool sacl_protected);

  std::optional<Sid> owner_;
  std::optional<Sid> group_;
  std::optional<AccessControlList> dacl_;
  bool dacl_protected_ = false;
  std::optional<AccessControlList> sacl_;
  bool sacl_protected_ = false;
};

}  // namespace base::win

#endif  // BASE_WIN_SECURITY_DESCRIPTOR_H_
