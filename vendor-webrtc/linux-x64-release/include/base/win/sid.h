// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SID_H_
#define BASE_WIN_SID_H_

#include <optional>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/win/windows_types.h"

namespace base::win {

// Known capabilities defined in Windows 8.
enum class WellKnownCapability {
  kInternetClient,
  kInternetClientServer,
  kPrivateNetworkClientServer,
  kPicturesLibrary,
  kVideosLibrary,
  kMusicLibrary,
  kDocumentsLibrary,
  kEnterpriseAuthentication,
  kSharedUserCertificates,
  kRemovableStorage,
  kAppointments,
  kContacts
};

// A subset of well known SIDs to create.
enum class WellKnownSid {
  kNull,
  kWorld,
  kCreatorOwner,
  kNetwork,
  kBatch,
  kInteractive,
  kService,
  kAnonymous,
  kSelf,
  kAuthenticatedUser,
  kRestricted,
  kLocalSystem,
  kLocalService,
  kNetworkService,
  kBuiltinAdministrators,
  kBuiltinUsers,
  kBuiltinGuests,
  kUntrustedLabel,
  kLowLabel,
  kMediumLabel,
  kHighLabel,
  kSystemLabel,
  kWriteRestricted,
  kCreatorOwnerRights,
  kAllApplicationPackages,
  kAllRestrictedApplicationPackages
};

// This class is used to hold and generate SIDs.
class BASE_EXPORT Sid {
 public:
  // Create a Sid from an AppContainer capability name. The name can be
  // completely arbitrary.
  static Sid FromNamedCapability(const std::wstring& capability_name);

  // Create a Sid from a known capability enumeration value. The Sids
  // match with the list defined in Windows 8.
  static Sid FromKnownCapability(WellKnownCapability capability);

  // Create a SID from a well-known type.
  static Sid FromKnownSid(WellKnownSid type);

  // Create a Sid from a SDDL format string, such as S-1-1-0.
  static std::optional<Sid> FromSddlString(const std::wstring& sddl_sid);

  // Create a Sid from a PSID pointer.
  static std::optional<Sid> FromPSID(const PSID sid);

  // Generate a random SID value.
  static Sid GenerateRandomSid();

  // Create a SID for an integrity level RID.
  static Sid FromIntegrityLevel(DWORD integrity_level);

  // Create a vector of SIDs from a vector of SDDL format strings.
  static std::optional<std::vector<Sid>> FromSddlStringVector(
      const std::vector<std::wstring>& sddl_sids);

  // Create a vector of SIDs from a vector of capability names.
  static std::vector<Sid> FromNamedCapabilityVector(
      const std::vector<std::wstring>& capability_names);

  // Create a vector of SIDs from a vector of well-known capability.
  static std::vector<Sid> FromKnownCapabilityVector(
      const std::vector<WellKnownCapability>& capabilities);

  // Create a vector of SIDs from a vector of well-known sids.
  static std::vector<Sid> FromKnownSidVector(
      const std::vector<WellKnownSid>& known_sids);

  // Create a known SID.
  explicit Sid(WellKnownSid known_sid);
  // Create a known capability SID.
  explicit Sid(WellKnownCapability known_capability);
  Sid(const Sid&) = delete;
  Sid& operator=(const Sid&) = delete;
  Sid(Sid&& sid);
  Sid& operator=(Sid&&);
  ~Sid();

  // Returns sid as a PSID. This should only be used temporarily while the Sid
  // is still within scope.
  PSID GetPSID() const;

  // Converts the SID to a SDDL format string.
  std::optional<std::wstring> ToSddlString() const;

  // Make a clone of the current Sid object.
  Sid Clone() const;

  // Is this Sid equal to another raw PSID?
  bool Equal(PSID sid) const;

  // Is this Sid equal to another Sid?
  bool operator==(const Sid& sid) const;

  // Is this Sid not equal to another Sid?
  bool operator!=(const Sid& sid) const;

 private:
  Sid(const void* sid, size_t length);
  std::vector<char> sid_;
};

}  // namespace base::win

#endif  // BASE_WIN_SID_H_
