#ifndef BSSL_VERIFY_H_
#define BSSL_VERIFY_H_

#include <chrono>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include <openssl/pki/signature_verify_cache.h>
#include <openssl/pki/verify_error.h>

BSSL_NAMESPACE_BEGIN

class CertIssuerSourceStatic;
class TrustStoreInMemory;
class CertificateVerifyOptions;
class CertificateVerifyStatus;

class OPENSSL_EXPORT VerifyTrustStore {
 public:
  std::unique_ptr<TrustStoreInMemory> trust_store;

  ~VerifyTrustStore();

  // FromDER returns a |TrustStore| derived from interpreting the |der_certs| as
  // a bunch of DER-encoded certs, concatenated. In the event of a failure nullptr
  // e is returned and a diagnostic string is placed in |out_diagnostic|
  static std::unique_ptr<VerifyTrustStore> FromDER(
      std::string_view der_certs, std::string *out_diagnostic);

  // FromDER returns a |TrustStore| consisting of the supplied DER-encoded
  // certs in |der_certs|. In the event of a failure nullptr is returned and a
  // diagnostic string is placed in |out_diagnostic|
  static std::unique_ptr<VerifyTrustStore> FromDER(
      const std::vector<std::string_view> &der_certs,
      std::string *out_diagnostic);
};

class OPENSSL_EXPORT CertPool {
 public:
  CertPool();
  CertPool(const CertPool &) = delete;
  CertPool &operator=(const CertPool &) = delete;
  virtual ~CertPool();

  // FromCerts returns a |CertPool| consisting of the supplied DER-encoded
  // certs in |der_certs|. In the event of a failure nullptr is returned and a
  // diagnostic string is placed in |out_diagnostic|
  static std::unique_ptr<CertPool> FromCerts(
      const std::vector<std::string_view> &der_certs,
      std::string *out_diagnostic);

 private:
  friend std::optional<std::vector<std::vector<std::string>>>
  CertificateVerifyInternal(const CertificateVerifyOptions &opts,
                            VerifyError *out_error,
                            CertificateVerifyStatus *out_status,
                            bool all_paths);
  std::unique_ptr<CertIssuerSourceStatic> impl_;
};

// CertificateVerifyOptions contains all the options for a certificate verification.
class OPENSSL_EXPORT CertificateVerifyOptions {
 public:
  // The key purpose (extended key usage) to check for during verification.
  enum class KeyPurpose {
    ANY_EKU,
    SERVER_AUTH,
    CLIENT_AUTH,
    SERVER_AUTH_STRICT,
    CLIENT_AUTH_STRICT,
    SERVER_AUTH_STRICT_LEAF,
    CLIENT_AUTH_STRICT_LEAF,
    RCS_MLS_CLIENT_AUTH,
    C2PA_TIMESTAMPING,
    C2PA_MANIFEST
  };

  CertificateVerifyOptions();
  CertificateVerifyOptions(const CertificateVerifyOptions &) = delete;
  CertificateVerifyOptions &operator=(const CertificateVerifyOptions &) =
      delete;

  KeyPurpose key_purpose = KeyPurpose::SERVER_AUTH;
  std::string_view leaf_cert;
  std::vector<std::string_view> intermediates;

  // extra_intermediates optionally points to a pool of common intermediates.
  const CertPool *extra_intermediates = nullptr;
  // trust_store points to the set of root certificates to trust.
  const VerifyTrustStore *trust_store = nullptr;
  // min_rsa_modulus_length is the minimum acceptable RSA key size in a chain.
  size_t min_rsa_modulus_length = 1024;
  // time is the time in POSIX seconds since the POSIX epoch at which to
  // validate the chain. It defaults to the current time if not set.
  std::optional<int64_t> time;
  // insecurely_allow_sha1 allows verification of signatures that use SHA-1
  // message digests.  This option is insecure and should not be used.
  bool insecurely_allow_sha1 = false;

  // max_iteration_count, if not zero, limits the number of times path building
  // will try to append an intermediate to a potential path. This bounds the
  // amount of time that a verification attempt can take, at the risk of
  // rejecting cases that would be solved if only more effort were used.
  uint32_t max_iteration_count = 0;

  // Sets an optional deadline for completing path building. It defaults
  // to std::chrono::time_point::max() if it not set. If |deadline| has a
  // value that has passed based on comparison to
  // std::chrono::steady_clock::now(), and path building has not completed,
  // path building will stop. Note that this is not a hard limit, there is no
  // guarantee how far past |deadline| time will be when path building is
  // aborted.
  std::optional<std::chrono::time_point<std::chrono::steady_clock>> deadline;

  // max_path_building_depth, if not zero, limits the depth of the path that the
  // path building algorithm attempts to build between leafs and roots. Using
  // this comes at the risk of rejecting cases that would be solved if only one
  // more certificate is added to the path.
  uint32_t max_path_building_depth = 0;

  // signature_verify_cache, if not nullptr, points to an object implementing a
  // signature verification cache derived from
  // <openssl/pki/signature_verify_cache.h>
  SignatureVerifyCache *signature_verify_cache = nullptr;
};

// CertificateVerifyStatus describes the status of a certificate verification
// attempt.
class OPENSSL_EXPORT CertificateVerifyStatus {
 public:
  CertificateVerifyStatus();

  // IterationCount returns the total number of attempted certificate additions
  // to any potential path while performing path building for verification. It
  // is the same value which may be bound by max_iteration_count in
  // CertificateVerifyOptions.
  size_t IterationCount() const;

  // MaxDepthSeen returns the maximum path depth seen during path building.
  size_t MaxDepthSeen() const;

 private:
  friend std::optional<std::vector<std::vector<std::string>>>
  CertificateVerifyInternal(const CertificateVerifyOptions &opts,
                            VerifyError *out_error,
                            CertificateVerifyStatus *out_status,
                            bool all_paths);
  size_t iteration_count_ = 0;
  size_t max_depth_seen_ = 0;
};

// Verify verifies |opts.leaf_cert| using the other values in |opts|. It
// returns either an error, or else a validated chain from leaf to root.
//
// In the event of an error return, |out_error| will be updated with information
// about the error.  It may be |nullptr|.
//
// Status information about the verification will be returned in |out_status|.
// It may be |nullptr|.
OPENSSL_EXPORT std::optional<std::vector<std::string>> CertificateVerify(
    const CertificateVerifyOptions &opts, VerifyError *out_error = nullptr,
    CertificateVerifyStatus *out_status = nullptr);

// VerifyAllPaths verifies |opts.leaf_cert| using the other values in |opts|,
// and returns all possible valid chains from the leaf to a root. If no chains
// exist, it returns an error.
OPENSSL_EXPORT std::optional<std::vector<std::vector<std::string>>>
CertificateVerifyAllPaths(const CertificateVerifyOptions &opts);

BSSL_NAMESPACE_END

#endif  // BSSL_VERIFY_H_
