// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_ACCESS_TOKEN_H_
#define BASE_WIN_ACCESS_TOKEN_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/win/access_control_list.h"
#include "base/win/scoped_handle.h"
#include "base/win/sid.h"
#include "base/win/windows_types.h"

namespace base::win {

// Impersonation level for the token.
enum class SecurityImpersonationLevel {
  kAnonymous,
  kIdentification,
  kImpersonation,
  kDelegation
};

// This class is used to access the information for a Windows access token.
class BASE_EXPORT AccessToken {
 public:
  // This class represents an access token group.
  class BASE_EXPORT Group {
   public:
    // Get the group SID.
    const Sid& GetSid() const LIFETIME_BOUND { return sid_; }
    // Get the group attribute flags.
    DWORD GetAttributes() const { return attributes_; }
    // Returns true if the group is an integrity level.
    bool IsIntegrity() const;
    // Returns true if the group is enabled.
    bool IsEnabled() const;
    // Returns true if the group is deny only.
    bool IsDenyOnly() const;
    // Returns true if the group is the logon ID.
    bool IsLogonId() const;

    Group(Sid&& sid, DWORD attributes);
    Group(Group&&);
    ~Group();

   private:
    Sid sid_;
    DWORD attributes_;
  };

  // This class represents an access token privilege.
  class BASE_EXPORT Privilege {
   public:
    // Get the privilege LUID.
    CHROME_LUID GetLuid() const { return luid_; }
    // Get the privilege attribute flags.
    DWORD GetAttributes() const { return attributes_; }
    // Get the name of the privilege.
    std::wstring GetName() const;
    // Returns true if the privilege is enabled.
    bool IsEnabled() const;

    Privilege(CHROME_LUID luid, DWORD attributes);

   private:
    CHROME_LUID luid_;
    DWORD attributes_;
  };

  // Creates an AccessToken object from a token handle.
  // |token| the token handle. This handle will be duplicated for TOKEN_QUERY
  // access, therefore the caller must be granted that access to the token
  // object. The AccessToken object owns its own copy of the token handle so
  // the original can be closed.
  // |desired_access| specifies additional access for the token handle,
  // TOKEN_QUERY will always be requested.
  static std::optional<AccessToken> FromToken(HANDLE token,
                                              ACCESS_MASK desired_access = 0);

  // Creates an AccessToken object from an existing token handle.
  // |token| the token handle. The AccessToken object will take ownership of
  // this handle without duplicating it. It must have been opened with at least
  // TOKEN_QUERY access to succeed.
  static std::optional<AccessToken> FromToken(ScopedHandle&& token);

  // Creates an AccessToken object from a process handle.
  // |process| the process handle. The handle needs to have
  // PROCESS_QUERY_LIMITED_INFORMATION access to the handle and TOKEN_QUERY
  // access to the token object.
  // |impersonation| if true then the process token will be duplicated to an
  // impersonation token. This allows you to call the IsMember API which
  // requires an impersonation token. To duplicate TOKEN_DUPLICATE access is
  // required.
  // |desired_access| specifies additional access for the token handle,
  // TOKEN_QUERY will always be requested.
  static std::optional<AccessToken> FromProcess(HANDLE process,
                                                bool impersonation = false,
                                                ACCESS_MASK desired_access = 0);

  // Creates an AccessToken object for the current process.
  // |impersonation| if true then the process token will be duplicated to an
  // impersonation token. This allows you to call the IsMember API which
  // requires an impersonation token. To duplicate TOKEN_DUPLICATE access is
  // required.
  // |desired_access| specifies additional access for the token handle,
  // TOKEN_QUERY will always be requested.
  static std::optional<AccessToken> FromCurrentProcess(
      bool impersonation = false,
      ACCESS_MASK desired_access = 0);

  // Creates an AccessToken object from a thread handle. The thread must be
  // impersonating a token for this to succeed.
  // |thread| the thread handle. The handle needs to have
  // THREAD_QUERY_LIMITED_INFORMATION access and TOKEN_QUERY access to the
  // token object.
  // |open_as_self| open the token using the process token rather than the
  // current thread's impersonated token.
  // If the thread isn't impersonating it will return an empty value and the
  // Win32 last error code will be ERROR_NO_TOKEN.
  // |desired_access| specifies additional access for the token handle,
  // TOKEN_QUERY will always be requested.
  static std::optional<AccessToken> FromThread(HANDLE thread,
                                               bool open_as_self = true,
                                               ACCESS_MASK desired_access = 0);

  // Creates an AccessToken object from the current thread. The thread must be
  // impersonating a token for this to succeed.
  // |open_as_self| open the thread handle using the process token rather
  // than the current thread's impersonated token.
  // If the thread isn't impersonating it will return an empty value and the
  // Win32 last error code will be ERROR_NO_TOKEN.
  // |desired_access| specifies additional access for the token handle,
  // TOKEN_QUERY will always be requested.
  static std::optional<AccessToken> FromCurrentThread(
      bool open_as_self = true,
      ACCESS_MASK desired_access = 0);

  // Creates an AccessToken object for the current thread's effective token.
  // If the thread is impersonating then it'll try and open the thread token,
  // otherwise it'll open the process token.
  // |desired_access| specifies additional access for the token handle,
  // TOKEN_QUERY will always be requested.
  static std::optional<AccessToken> FromEffective(
      ACCESS_MASK desired_access = 0);

  AccessToken(const AccessToken&) = delete;
  AccessToken& operator=(const AccessToken&) = delete;
  AccessToken(AccessToken&&);
  AccessToken& operator=(AccessToken&&);
  ~AccessToken();

  // Get the token's user SID.
  Sid User() const;

  // Get the token's user group.
  Group UserGroup() const;

  // Get the token's owner SID. This can be different to the user SID, it's
  // used as the default owner for new secured objects.
  Sid Owner() const;

  // Get the token's primary group SID.
  Sid PrimaryGroup() const;

  // Get the token logon SID. Returns an empty value if the token doesn't have
  // a logon SID. If the logon SID doesn't exist then the Win32 last error code
  // will be ERROR_NOT_FOUND.
  std::optional<Sid> LogonId() const;

  // Get the token's integrity level. Returns MAXDWORD if the token doesn't
  // have an integrity level.
  DWORD IntegrityLevel() const;

  // Set the token's integrity level. Token needs to have been opened with
  // TOKEN_ADJUST_DEFAULT access.
  bool SetIntegrityLevel(DWORD integrity_level);

  // Get the token's session ID. Returns MAXDWORD if the token if the session
  // ID can't be queried.
  DWORD SessionId() const;

  // The token's group list.
  std::vector<Group> Groups() const;

  // Get whether the token is a restricted.
  bool IsRestricted() const;

  // The token's restricted SIDs list. If not a restricted token this will
  // return an empty vector.
  std::vector<Group> RestrictedSids() const;

  // Get whether the token is an appcontainer.
  bool IsAppContainer() const;

  // Get the token's appcontainer SID. If not an appcontainer token this will
  // return an empty value.
  std::optional<Sid> AppContainerSid() const;

  // The token's capabilities. If not an appcontainer token this will return an
  // empty vector.
  std::vector<Group> Capabilities() const;

  // Get the UAC linked token.
  std::optional<AccessToken> LinkedToken() const;

  // Get the default DACL for the token. Returns an empty value on error.
  std::optional<AccessControlList> DefaultDacl() const;

  // Set the default DACL of the token. Token needs to have been opened with
  // TOKEN_ADJUST_DEFAULT access.
  bool SetDefaultDacl(const AccessControlList& default_dacl);

  // Get the token's ID.
  CHROME_LUID Id() const;

  // Get the token's authentication ID.
  CHROME_LUID AuthenticationId() const;

  // Get the token's privileges.
  std::vector<Privilege> Privileges() const;

  // Get whether the token is elevated.
  bool IsElevated() const;

  // Checks if the sid is a member of the token's groups. The token must be
  // an impersonation token rather than a primary token. If the token is not an
  // impersonation token then it returns false and the Win32 last error will be
  // set to ERROR_NO_IMPERSONATION_TOKEN.
  bool IsMember(const Sid& sid) const;

  // Checks if the well known sid is a member of the token's groups. The token
  // must be an impersonation token rather than a primary token. If the token
  // is not an impersonation token then it returns false and the Win32 last
  // error will be set to ERROR_NO_IMPERSONATION_TOKEN.
  bool IsMember(WellKnownSid known_sid) const;

  // Checks if the token is an impersonation token. If false then it's a primary
  // token.
  bool IsImpersonation() const;

  // Checks if the token can only be used for identification. This is based on
  // the security impersonation level of the token. If the level is less than
  // or equal to SecurityIdentification this function returns true. Always
  // returns false for a primary token.
  bool IsIdentification() const;

  // Get the current impersonation level. If the token is a primary token
  // the function returns kImpersonation.
  SecurityImpersonationLevel ImpersonationLevel() const;

  // Duplicate the token to a new primary token.
  // |desired_access| specifies additional access for the token handle.
  // TOKEN_QUERY will always be requested.
  // The original token must have TOKEN_DUPLICATE access to successfully
  // duplicate the token.
  std::optional<AccessToken> DuplicatePrimary(
      ACCESS_MASK desired_access = 0) const;

  // Duplicate the token to a new impersonation token.
  // |impersonation_level| specifies the impersonation level for the token.
  // |desired_access| specifies additional access for the token handle.
  // TOKEN_QUERY will always be requested.
  // The original token must have TOKEN_DUPLICATE access to successfully
  // duplicate the token.
  std::optional<AccessToken> DuplicateImpersonation(
      SecurityImpersonationLevel impersonation_level =
          SecurityImpersonationLevel::kImpersonation,
      ACCESS_MASK desired_access = 0) const;

  // Create a new restricted token from this token.
  // |flags| can be set to a combination of DISABLE_MAX_PRIVILEGE,
  // SANDBOX_INERT, LUA_TOKEN and WRITE_RESTRICTED.
  // |sids_to_disable| is the list of SIDs to disable in the token.
  // |privileges_to_delete| is the names of the privileges to delete.
  // |sids_to_restrict| is the list of SIDs to add as restricted SIDs.
  // |desired_access| specifies additional access for the token handle.
  // The token needs to be opened with TOKEN_DUPLICATE access.
  std::optional<AccessToken> CreateRestricted(
      DWORD flags,
      const std::vector<Sid>& sids_to_disable,
      const std::vector<std::wstring>& privileges_to_delete,
      const std::vector<Sid>& sids_to_restrict,
      ACCESS_MASK desired_access = 0) const;

  // Create a new AppContainer primary token from this token.
  // |app_container_sid| the AppContainer package SID.
  // |capabilities| the list of AppContainer capabilities.
  // |desired_access| specifies additional access for the token handle.
  // The token needs to be opened with TOKEN_DUPLICATE access.
  std::optional<AccessToken> CreateAppContainer(
      const Sid& appcontainer_sid,
      const std::vector<Sid>& capabilities,
      ACCESS_MASK desired_access = 0) const;

  // Enable or disable a privilege.
  // |name| the name of the privilege to change.
  // |enable| specify whether to enable or disable the privilege.
  // Returns the previous enable state of the privilege, or nullopt if failed.
  // The token must be opened with TOKEN_ADJUST_PRIVILEGES access.
  std::optional<bool> SetPrivilege(const std::wstring& name, bool enable);

  // Remove a privilege permanently from the token.
  // |name| the name of the privilege to remove.
  // Returns true if successfully removed the privilege.
  // The token must be opened with TOKEN_ADJUST_PRIVILEGES access.
  bool RemovePrivilege(const std::wstring& name);

  // Permanently remove all privileges from the token.
  // Returns true if the operation was successful.
  // The token must be opened with TOKEN_ADJUST_PRIVILEGES access.
  bool RemoveAllPrivileges();

  // Indicates if the AccessToken object is valid.
  bool is_valid() const;

  // Get the underlying token handle.
  HANDLE get() const;

  // Take ownership of the underlying token handle. Once released no other
  // methods on this object should be called.
  ScopedHandle release();

 private:
  explicit AccessToken(HANDLE token);
  ScopedHandle token_;
};

}  // namespace base::win

#endif  // BASE_WIN_ACCESS_TOKEN_H_
