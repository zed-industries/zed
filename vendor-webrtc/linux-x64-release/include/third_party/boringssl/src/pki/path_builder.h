// Copyright 2016 The Chromium Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef BSSL_PKI_PATH_BUILDER_H_
#define BSSL_PKI_PATH_BUILDER_H_

#include <memory>
#include <vector>

#include <openssl/base.h>
#include <openssl/pki/verify_error.h>

#include "cert_errors.h"
#include "input.h"
#include "parse_values.h"
#include "parsed_certificate.h"
#include "trust_store.h"
#include "verify_certificate_chain.h"

BSSL_NAMESPACE_BEGIN

namespace der {
struct GeneralizedTime;
}

class CertPathBuilder;
class CertPathIter;
class CertIssuerSource;

// Base class for custom data that CertPathBuilderDelegate can attach to paths.
class OPENSSL_EXPORT CertPathBuilderDelegateData {
 public:
  virtual ~CertPathBuilderDelegateData() = default;
};

// Represents a single candidate path that was built or is being processed.
//
// This is used both to represent valid paths, as well as invalid/partial ones.
//
// Consumers must use |IsValid()| to test whether the
// CertPathBuilderResultPath is the result of a successful certificate
// verification.
struct OPENSSL_EXPORT CertPathBuilderResultPath {
  CertPathBuilderResultPath();
  ~CertPathBuilderResultPath();

  // Returns true if the candidate path is valid. A "valid" path is one which
  // chains to a trusted root, and did not have any high severity errors added
  // to it during certificate verification.
  bool IsValid() const;

  // Public verify error result for this candidate path.
  VerifyError GetVerifyError() const;

  // Returns the chain's root certificate or nullptr if the chain doesn't
  // chain to a trust anchor.
  const ParsedCertificate *GetTrustedCert() const;

  // Path in the forward direction:
  //
  //   certs[0] is the target certificate
  //   certs[i] was issued by certs[i+1]
  //   certs.back() is the root certificate (which may or may not be trusted).
  ParsedCertificateList certs;

  // Describes the trustedness of the final certificate in the chain,
  // |certs.back()|
  //
  // For result paths where |IsValid()|, the final certificate is trusted.
  // However for failed or partially constructed paths the final certificate may
  // not be a trust anchor.
  CertificateTrust last_cert_trust;

  // The set of policies that the certificate is valid for (of the
  // subset of policies user requested during verification).
  std::set<der::Input> user_constrained_policy_set;

  // Slot for per-path data that may set by CertPathBuilderDelegate. The
  // specific type is chosen by the delegate. Can be nullptr when unused.
  std::unique_ptr<CertPathBuilderDelegateData> delegate_data;

  // The set of errors and warnings associated with this path (bucketed
  // per-certificate). Note that consumers should always use |IsValid()| to
  // determine validity of the CertPathBuilderResultPath, and not just inspect
  // |errors|.
  CertPathErrors errors;
};

// CertPathBuilderDelegate controls policies for certificate verification and
// path building.
class OPENSSL_EXPORT CertPathBuilderDelegate
    : public VerifyCertificateChainDelegate {
 public:
  // This is called during path building on candidate paths. These are either
  // paths which have already been run through RFC 5280 verification, or
  // partial paths that the path builder cannot continue either due to not
  // finding a matching issuer or reaching a configured pathbuilding limit.
  // |path| may already have errors and warnings set on it. Delegates can
  // "reject" a candidate path from path building by adding high severity
  // errors.
  virtual void CheckPathAfterVerification(const CertPathBuilder &path_builder,
                                          CertPathBuilderResultPath *path) = 0;

  // This is called during path building in between attempts to build candidate
  // paths. Delegates can cause path building to stop and return indicating
  // the deadline was exceeded by returning true from this function.
  virtual bool IsDeadlineExpired() = 0;

  // This is called during path building to decide if debug logs will be
  // sent to the delegate rom the path builder. No calls to DebugLog (below)
  // will be made unless this returns true.
  virtual bool IsDebugLogEnabled() = 0;

  // This is called to send a debug log string |msg| to the delegate. These are
  // only called if IsDebugLogEnabled (above) returns true.
  virtual void DebugLog(std::string_view msg) = 0;
};

// Checks whether a certificate is trusted by building candidate paths to trust
// anchors and verifying those paths according to RFC 5280. Each instance of
// CertPathBuilder is used for a single verification.
//
// WARNING: This implementation is currently experimental.  Consult an OWNER
// before using it.
class OPENSSL_EXPORT CertPathBuilder {
 public:
  // Provides the overall result of path building. This includes the paths that
  // were attempted.
  struct OPENSSL_EXPORT Result {
    Result();
    Result(Result &&);

    Result(const Result &) = delete;
    Result &operator=(const Result &) = delete;

    ~Result();
    Result &operator=(Result &&);

    // Returns true if there was a valid path.
    bool HasValidPath() const;

    // Returns true if any of the attempted paths contain |error_id|.
    bool AnyPathContainsError(CertErrorId error_id) const;

    // Returns the best single error from result, using the best path found.
    const VerifyError GetBestPathVerifyError() const;

    // Returns the CertPathBuilderResultPath for the best valid path, or nullptr
    // if there was none.
    const CertPathBuilderResultPath *GetBestValidPath() const;

    // Returns the best CertPathBuilderResultPath or nullptr if there was none.
    const CertPathBuilderResultPath *GetBestPathPossiblyInvalid() const;

    // List of paths that were attempted and the result for each.
    std::vector<std::unique_ptr<CertPathBuilderResultPath>> paths;

    // Index into |paths|. Before use, |paths.empty()| must be checked.
    // NOTE: currently the definition of "best" is fairly limited. Valid is
    // better than invalid, but otherwise nothing is guaranteed.
    size_t best_result_index = 0;

    // The iteration count reached by path building.
    uint32_t iteration_count = 0;

    // The max depth seen while path building.
    uint32_t max_depth_seen = 0;

    // True if the search stopped because it exceeded the iteration limit
    // configured with |SetIterationLimit|.
    bool exceeded_iteration_limit = false;

    // True if the search stopped because delegate->IsDeadlineExpired() returned
    // true.
    bool exceeded_deadline = false;
  };

  // Creates a CertPathBuilder that attempts to find a path from |cert| to a
  // trust anchor in |trust_store| and is valid at |time|.
  //
  // The caller must keep |trust_store| and |delegate| valid for the lifetime
  // of the CertPathBuilder.
  //
  // See VerifyCertificateChain() for a more detailed explanation of the
  // same-named parameters not defined below.
  //
  // * |delegate|: Must be non-null. The delegate is called at various points in
  //               path building to verify specific parts of certificates or the
  //               final chain. See CertPathBuilderDelegate and
  //               VerifyCertificateChainDelegate for more information.
  CertPathBuilder(std::shared_ptr<const ParsedCertificate> cert,
                  TrustStore *trust_store, CertPathBuilderDelegate *delegate,
                  const der::GeneralizedTime &time, KeyPurpose key_purpose,
                  InitialExplicitPolicy initial_explicit_policy,
                  const std::set<der::Input> &user_initial_policy_set,
                  InitialPolicyMappingInhibit initial_policy_mapping_inhibit,
                  InitialAnyPolicyInhibit initial_any_policy_inhibit);

  CertPathBuilder(const CertPathBuilder &) = delete;
  CertPathBuilder &operator=(const CertPathBuilder &) = delete;

  ~CertPathBuilder();

  // Adds a CertIssuerSource to provide intermediates for use in path building.
  // Multiple sources may be added. Must not be called after Run is called.
  // The |*cert_issuer_source| must remain valid for the lifetime of the
  // CertPathBuilder.
  //
  // (If no issuer sources are added, the target certificate will only verify if
  // it is a trust anchor or is directly signed by a trust anchor.)
  void AddCertIssuerSource(CertIssuerSource *cert_issuer_source);

  // Sets a limit to the number of times to repeat the process of considering a
  // new intermediate over all potential paths. Setting |limit| to 0 disables
  // the iteration limit, which is the default.
  void SetIterationLimit(uint32_t limit);

  // Sets a limit to the number of certificates to be added in a path from leaf
  // to root. Setting |limit| to 0 disables this limit, which is the default.
  void SetDepthLimit(uint32_t limit);

  // Set the limit of valid paths returned by the path builder to |limit|.  If
  // |limit| is non zero, path building will stop once |limit| valid paths have
  // been found. Setting |limit| to 0 disables the limit, meaning path building
  // will continue until all possible paths have been exhausted (or iteration
  // limit / deadline is exceeded).  The default limit is 1.
  void SetValidPathLimit(size_t limit);

  // If |explore_all_paths| is false, this is equivalent to calling
  // SetValidPathLimit(1). If |explore_all_paths| is true, this is equivalent to
  // calling SetValidPathLimit(0).
  void SetExploreAllPaths(bool explore_all_paths);

  // Executes verification of the target certificate.
  //
  // Run must not be called more than once on each CertPathBuilder instance.
  Result Run();

 private:
  void AddResultPath(std::unique_ptr<CertPathBuilderResultPath> result_path);

  // |out_result_| may be referenced by other members, so should be initialized
  // first.
  Result out_result_;

  std::unique_ptr<CertPathIter> cert_path_iter_;
  CertPathBuilderDelegate *delegate_;
  const der::GeneralizedTime time_;
  const KeyPurpose key_purpose_;
  const InitialExplicitPolicy initial_explicit_policy_;
  const std::set<der::Input> user_initial_policy_set_;
  const InitialPolicyMappingInhibit initial_policy_mapping_inhibit_;
  const InitialAnyPolicyInhibit initial_any_policy_inhibit_;
  uint32_t max_iteration_count_ = 0;
  uint32_t max_path_building_depth_ = 0;
  size_t valid_path_limit_ = 1;
  size_t valid_path_count_ = 0;
};

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_PATH_BUILDER_H_
