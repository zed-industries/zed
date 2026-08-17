// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_PROCESS_REQUIREMENT_H_
#define BASE_MAC_PROCESS_REQUIREMENT_H_

#include <Security/Security.h>
#include <mach/mach.h>

#include <optional>
#include <string>
#include <vector>

#include "base/apple/scoped_cftyperef.h"
#include "base/base_export.h"
#include "base/containers/span.h"

namespace base::mac {

enum class ValidationCategory : unsigned int;

// Represents constraints on the code signing identity of a peer process.
//
// `ProcessRequirement` is typically used to describe which processes are
// permitted to establish IPC connections, and to validate that a connecting
// process fulfills those constraints.
class BASE_EXPORT ProcessRequirement {
 public:
  class BASE_EXPORT Builder {
   public:
    Builder();
    ~Builder();

    Builder(const Builder&) = delete;
    Builder& operator=(const Builder&) = delete;
    Builder(Builder&&);
    Builder& operator=(Builder&&);

    // The identifier in the signature must match `identifier`.
    // Can be called at most once. See `IdentifierIsOneOf` if multiple
    // identifiers can be accepted.
    //
    // The identifier is typically the executable name or bundle identifier of
    // the application.
    Builder Identifier(std::string identifier) &&;

    // The identifier in the signature must match one of the values
    // in`identifiers`.
    // Can be called at most once.
    //
    // The identifier is typically the executable name or bundle identifier of
    // the application.
    Builder IdentifierIsOneOf(std::vector<std::string> identifiers) &&;

    // Equivalent to HasSameTeamIdentifier().HasSameCertificateType()
    Builder SignedWithSameIdentity() &&;

    // The process must be signed with a certificate that uses the same
    // team identifier as this process.
    // Can be called at most once.
    //
    // Note: It is an error to call this without also limiting the certificate
    // type via `HasSameCertificateType`, `DeveloperIdCertificateType`, etc.
    Builder HasSameTeamIdentifier() &&;

    // The process must be signed with the same type of certificate as this
    // process.
    // Can be called at most once.
    Builder HasSameCertificateType() &&;

    // The team identifier in the signing certificate matches `team_identifier`.
    // Can be called at most once.
    //
    // Note: It is an error to call this without also limiting the certificate
    // type via `HasSameCertificateType`, `DeveloperIdCertificateType`, etc.
    Builder TeamIdentifier(std::string team_identifier) &&;

    // The certificate used during signing is an Apple Developer ID certificate.
    // Can be called at most once.
    Builder DeveloperIdCertificateType() &&;

    // The certificate used during signing is an Apple App Store certificate.
    // Can be called at most once.
    Builder AppStoreCertificateType() &&;

    // The certificate used during signing is an Apple Development certificate
    // that cannot be used for distributing applications.
    // Can be called at most once.
    Builder DevelopmentCertificateType() &&;

    // Validate only the dynamic signature of the application without
    // comparing it to the state of the application on disk.
    //
    // Note that when requesting dynamic validation it is necessary to
    // supply the application's Info.plist data when performing
    // code signature validation using the resulting requirement.
    Builder CheckDynamicValidityOnly() &&;

    // Consume the constraints and produce a ProcessRequirement.
    // Returns `std::nullopt` on error.
    std::optional<ProcessRequirement> Build() &&;

   private:
    std::vector<std::string> identifiers_;
    std::string team_identifier_;
    std::optional<ValidationCategory> validation_category_;
    bool dynamic_validity_only_ = false;
    bool failed_ = false;
    bool has_same_team_identifier_called_ = false;
    bool has_same_certificate_type_called_ = false;
  };  // class Builder

  // Use Builder::Build to construct a ProcessRequirement.
  ProcessRequirement() = delete;
  ~ProcessRequirement();

  ProcessRequirement(const ProcessRequirement&);
  ProcessRequirement& operator=(const ProcessRequirement&);
  ProcessRequirement(ProcessRequirement&&);
  ProcessRequirement& operator=(ProcessRequirement&&);

  // Validate the process represented by `audit_token` against this requirement.
  //
  // If this requirement was created with `CheckDynamicValidityOnly()` then
  // the target process's Info.plist data must be provided in `info_plist_data`.
  bool ValidateProcess(audit_token_t audit_token,
                       base::span<const uint8_t> info_plist_data = {}) const;

  // Create a `SecRequirementRef` from the requirement.
  // Will return `nullptr` if the requirement does not place any limits
  // on the process, such as if `SignedWithSameIdentity()` was used
  // from a process with an ad-hoc code signature.
  //
  // Prefer to use `ValidateProcess` when possible.
  apple::ScopedCFTypeRef<SecRequirementRef> AsSecRequirement() const;

  // Returns true if only the dynamic signature of the application
  // should be validated without comparing it to the state of the
  // application on disk.
  bool ShouldCheckDynamicValidityOnly() const { return dynamic_validity_only_; }

  // Gather metrics to validate the reliability of ProcessRequirement.
  // Work is performed asynchronously on a background thread.
  static void MaybeGatherMetrics();

  static ProcessRequirement AlwaysMatchesForTesting();
  static ProcessRequirement NeverMatchesForTesting();

  void SetShouldCheckDynamicValidityOnlyForTesting();

  struct CSOpsSystemCallProvider {
    virtual ~CSOpsSystemCallProvider() = default;
    virtual int csops(pid_t pid,
                      unsigned int ops,
                      void* useraddr,
                      size_t usersize) = 0;
    virtual bool SupportsValidationCategory() const = 0;
  };

  // Use `csops_provider` function in place of using the default provider which
  // uses the `csops` system call for retrieving code signing information.
  // Pass `nullptr` to reset to the default provider.
  static void SetCSOpsSystemCallProviderForTesting(
      CSOpsSystemCallProvider* csops_provider);

 private:
  ProcessRequirement(std::vector<std::string> identifiers,
                     std::string team_identifier,
                     ValidationCategory validation_category,
                     bool dynamic_validity_only);

  // Returns true if the code signature must be validated to enforce this
  // requirement.
  // This will be false for unsigned code and true for all signed code.
  bool RequiresSignatureValidation() const;

  // Do the work of gathering metrics. Called on a background thread.
  static void GatherMetrics();

  enum ForTesting {
    AlwaysMatches,
    NeverMatches,
  };

  explicit ProcessRequirement(ForTesting for_testing);

  static apple::ScopedCFTypeRef<SecRequirementRef> AsSecRequirementForTesting(
      ForTesting for_testing);

  std::vector<std::string> identifiers_;
  std::string team_identifier_;
  std::optional<ForTesting> for_testing_;
  ValidationCategory validation_category_;
  bool dynamic_validity_only_ = false;
};

}  // namespace base::mac

#endif  // BASE_MAC_PROCESS_REQUIREMENT_H_
