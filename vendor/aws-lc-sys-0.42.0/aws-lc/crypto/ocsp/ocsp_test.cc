// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC
#include <gtest/gtest.h>
#include <array>
#include <ctime>

#include "openssl/ocsp.h"
#include "openssl/pem.h"
#include "openssl/bio.h"

#include "../internal.h"
#include "../test/test_util.h"
#include "internal.h"

static const time_t invalid_before_ocsp_update_time = 1621988613;
static const time_t valid_after_ocsp_update_time = 1621988615;
static const time_t valid_before_ocsp_expire_time = 1937348613;
static const time_t invalid_after_ocsp_expire_time = 1937348615;

static const time_t invalid_before_ocsp_update_time_sha256 = 1622145762;
static const time_t valid_after_ocsp_update_time_sha256 = 1622145764;
static const time_t valid_before_ocsp_expire_time_sha256 = 1937505762;
static const time_t invalid_after_ocsp_expire_time_sha256 = 1937505764;

#define OCSP_VERIFYSTATUS_SUCCESS 1
#define OCSP_VERIFYSTATUS_ERROR 0
#define OCSP_VERIFYSTATUS_FATALERROR -1

#define OCSP_RESPFINDSTATUS_SUCCESS 1
#define OCSP_RESPFINDSTATUS_ERROR 0
#define OCSP_RESPFINDSTATUS_UNDEFINED -1

#define OCSP_REQUEST_PARSE_SUCCESS 1
#define OCSP_REQUEST_PARSE_ERROR 0

#define OCSP_HTTP_PARSE_SUCCESS 1
#define OCSP_HTTP_PARSE_ERROR 0

#define OCSP_SIGN_SUCCESS 1
#define OCSP_SIGN_ERROR 0

#define OCSP_URL_PARSE_SUCCESS 1
#define OCSP_URL_PARSE_ERROR 0

std::string GetTestData(const char *path);

static bool DecodeBase64(std::vector<uint8_t> *out, const char *in) {
  int temp_length = 0;
  size_t total_length = 0;
  EVP_ENCODE_CTX ctx;
  EVP_DecodeInit(&ctx);
  out->resize(strlen(in));

  int decode_update_result = EVP_DecodeUpdate(&ctx, out->data(), &temp_length,
                                              (const uint8_t *)in, strlen(in));
  if (decode_update_result != 1 && decode_update_result != 0) {
    fprintf(stderr, "EVP_DecodeUpdate failed\n");
    return false;
  }
  total_length += temp_length;
  temp_length = 0;

  if (1 != EVP_DecodeFinal(&ctx, &out->data()[total_length], &temp_length)) {
    fprintf(stderr, "EVP_DecodeFinal failed\n");
    return false;
  }
  total_length += temp_length;
  out->resize(total_length);
  return true;
}

static bssl::UniquePtr<EC_KEY> ECDSAFromPEM(const char *pem) {
  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(pem, strlen(pem)));
  return bssl::UniquePtr<EC_KEY>(
      PEM_read_bio_ECPrivateKey(bio.get(), nullptr, nullptr, nullptr));
}

static bssl::UniquePtr<STACK_OF(X509)> CertChainFromPEM(const char *pem) {
  bssl::UniquePtr<STACK_OF(X509)> stack(sk_X509_new_null());
  if (!stack) {
    return nullptr;
  }

  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(pem, strlen(pem)));
  for (;;) {
    bssl::UniquePtr<X509> cert = bssl::UniquePtr<X509>(
        PEM_read_bio_X509(bio.get(), nullptr, nullptr, nullptr));
    if (cert == nullptr) {
      break;
    }
    if (!bssl::PushToStack(stack.get(), bssl::UpRef(cert.get()))) {
      return nullptr;
    }
  }
  return stack;
}

static bssl::UniquePtr<OCSP_RESPONSE> LoadOCSP_RESPONSE(
    bssl::Span<const uint8_t> der) {
  const uint8_t *ptr = der.data();
  return bssl::UniquePtr<OCSP_RESPONSE>(
      d2i_OCSP_RESPONSE(nullptr, &ptr, der.size()));
}

static bssl::UniquePtr<OCSP_REQUEST> LoadOCSP_REQUEST(
    bssl::Span<const uint8_t> der) {
  const uint8_t *ptr = der.data();
  return bssl::UniquePtr<OCSP_REQUEST>(
      d2i_OCSP_REQUEST(nullptr, &ptr, der.size()));
}

// Load a generic |OCSP_CERTID| for testing.
static OCSP_CERTID *LoadTestOCSP_CERTID() {
  bssl::UniquePtr<X509> ca_cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/ca_cert.pem").c_str())
          .c_str()));
  bssl::UniquePtr<X509> server_cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/rsa_cert.pem").c_str())
          .c_str()));
  return OCSP_cert_to_id(nullptr, ca_cert.get(), server_cert.get());
}

static void ExtractAndVerifyBasicOCSP(
    bssl::Span<const uint8_t> der, const std::string ca_cert_file,
    const std::string server_cert_file, int expected_ocsp_verify_status,
    bssl::UniquePtr<OCSP_BASICRESP> *basic_response,
    bssl::UniquePtr<STACK_OF(X509)> *server_cert_chain) {
  bssl::UniquePtr<OCSP_RESPONSE> ocsp_response;
  ocsp_response = LoadOCSP_RESPONSE(der);
  ASSERT_TRUE(ocsp_response);

  int ret = OCSP_response_status(ocsp_response.get());
  if (ret != OCSP_RESPONSE_STATUS_SUCCESSFUL) {
    return;
  }

  *basic_response = bssl::UniquePtr<OCSP_BASICRESP>(
      OCSP_response_get1_basic(ocsp_response.get()));
  ASSERT_TRUE(*basic_response);

  // Set up trust store and certificate chain.
  bssl::UniquePtr<X509> ca_cert(CertFromPEM(
      GetTestData(
          std::string("crypto/ocsp/test/aws/" + ca_cert_file + ".pem").c_str())
          .c_str()));
  bssl::UniquePtr<X509> server_cert(
      CertFromPEM(GetTestData(std::string("crypto/ocsp/test/aws/" +
                                          server_cert_file + ".pem")
                                  .c_str())
                      .c_str()));

  bssl::UniquePtr<X509_STORE> trust_store(X509_STORE_new());
  X509_STORE_add_cert(trust_store.get(), ca_cert.get());
  *server_cert_chain = CertsToStack({server_cert.get(), ca_cert.get()});
  ASSERT_TRUE(*server_cert_chain);

  // Verifies the OCSP responder's signature on the OCSP response data.
  const int ocsp_verify_err = OCSP_basic_verify(
      basic_response->get(), server_cert_chain->get(), trust_store.get(), 0);
  ASSERT_EQ(expected_ocsp_verify_status, ocsp_verify_err);
}

static void CheckOCSP_CERTSTATUS(
    bssl::UniquePtr<OCSP_BASICRESP> *basic_response,
    bssl::UniquePtr<STACK_OF(X509)> *server_cert_chain, const EVP_MD *dgst,
    int expected_resp_find_status, int *status, ASN1_GENERALIZEDTIME **thisupd,
    ASN1_GENERALIZEDTIME **nextupd) {
  X509 *subject = sk_X509_value(server_cert_chain->get(), 0);
  X509 *issuer = sk_X509_value(server_cert_chain->get(), 1);
  // Convert issuer certificate to |OCSP_CERTID|.
  bssl::UniquePtr<OCSP_CERTID> cert_id =
      bssl::UniquePtr<OCSP_CERTID>(OCSP_cert_to_id(dgst, subject, issuer));
  ASSERT_TRUE(cert_id);

  int reason = 0;
  ASN1_GENERALIZEDTIME *revtime;
  // Checks revocation status of the response.
  const int ocsp_resp_find_status_res =
      OCSP_resp_find_status(basic_response->get(), cert_id.get(), status,
                            &reason, &revtime, thisupd, nextupd);
  ASSERT_EQ(expected_resp_find_status, ocsp_resp_find_status_res);
}

// Test data below are taken from s2n's ocsp test files:
// https://github.com/aws/s2n-tls/blob/main/tests/pems/ocsp
// OCSP testing methods were taken from s2n's validation tests:
// https://github.com/aws/s2n-tls/blob/main/tests/unit/s2n_x509_validator_test.c
struct OCSPAWSTestVector {
  const char *ocsp_response;
  const char *cafile;
  const char *server_cert;
  const EVP_MD *dgst;
  int expected_ocsp_verify_status;
  int expected_ocsp_resp_find_status;
  int expected_ocsp_cert_status;
  const char *expected_ocsp_cert_status_string;
};

static const OCSPAWSTestVector nTestVectors[] = {
    // === SHA1 OCSP RESPONSES ===
    // Test valid OCSP response signed by an OCSP responder.
    {"ocsp_response", "ca_cert", "rsa_cert", EVP_sha1(),
     OCSP_VERIFYSTATUS_SUCCESS, OCSP_RESPFINDSTATUS_SUCCESS,
     V_OCSP_CERTSTATUS_GOOD, "good"},
    // Test against same good OCSP response, but checking behavior of not
    // specifying hash algorithm used for |OCSP_cert_to_id| this time (should
    // default to sha1). When |*dgst| is set to NULL, the default hash algorithm
    // should automatically be set to sha1. The revocation status check of the
    // response should work if hash algorithm of |cert_id| has been set to sha1
    // successfully.
    {"ocsp_response", "ca_cert", "rsa_cert", nullptr, OCSP_VERIFYSTATUS_SUCCESS,
     OCSP_RESPFINDSTATUS_SUCCESS, V_OCSP_CERTSTATUS_GOOD, "good"},
    // Test valid OCSP response directly signed by the CA certificate.
    {"ocsp_response_ca_signed", "ca_cert", "rsa_cert", EVP_sha1(),
     OCSP_VERIFYSTATUS_SUCCESS, OCSP_RESPFINDSTATUS_SUCCESS,
     V_OCSP_CERTSTATUS_GOOD, "good"},
    // Test OCSP response status is revoked.
    {"ocsp_response_revoked", "ca_cert", "rsa_cert", EVP_sha1(),
     OCSP_VERIFYSTATUS_SUCCESS, OCSP_RESPFINDSTATUS_SUCCESS,
     V_OCSP_CERTSTATUS_REVOKED, "revoked"},
    // Test OCSP response status is unknown.
    {"ocsp_response_unknown", "ca_cert", "rsa_cert", EVP_sha1(),
     OCSP_VERIFYSTATUS_SUCCESS, OCSP_RESPFINDSTATUS_SUCCESS,
     V_OCSP_CERTSTATUS_UNKNOWN, "unknown"},
    // for the requested certificate. (So this would be a completely valid
    // response to a different OCSP request for the other certificate.)
    {"ocsp_response", "ca_cert", "ecdsa_cert", EVP_sha1(),
     OCSP_VERIFYSTATUS_SUCCESS, OCSP_RESPFINDSTATUS_ERROR, -1, nullptr},
    // Test OCSP response where the requested certificate was signed by the OCSP
    // responder, but signed by the wrong requested OCSP responder key
    // certificate.
    // However, this incorrect OCSP responder certificate may be a valid OCSP
    // responder for some other case and also chains to a trusted root.
    {"ocsp_response_wrong_signer", "ca_cert", "rsa_cert", EVP_sha1(),
     OCSP_VERIFYSTATUS_ERROR, OCSP_RESPFINDSTATUS_UNDEFINED, -1, nullptr},
    // Test OCSP response where the requested certificate was signed by an OCSP
    // responder with an expired certificate.
    // However, this incorrect OCSP responder certificate may be a valid OCSP
    // responder for some other case and also chains to a trusted root.
    {"ocsp_response_expired_signer", "ca_cert", "rsa_cert", EVP_sha1(),
     OCSP_VERIFYSTATUS_ERROR, OCSP_RESPFINDSTATUS_UNDEFINED, -1, nullptr},

    // === SHA256 OCSP RESPONSES ===
    // Test valid OCSP response signed by an OCSP responder.
    {"ocsp_response_sha256", "ca_cert", "rsa_cert", EVP_sha256(),
     OCSP_VERIFYSTATUS_SUCCESS, OCSP_RESPFINDSTATUS_SUCCESS,
     V_OCSP_CERTSTATUS_GOOD, "good"},
    // Test a SHA-256 revoked OCSP response status.
    {"ocsp_response_revoked_sha256", "ca_cert", "rsa_cert", EVP_sha256(),
     OCSP_VERIFYSTATUS_SUCCESS, OCSP_RESPFINDSTATUS_SUCCESS,
     V_OCSP_CERTSTATUS_REVOKED, "revoked"},
    // Test a SHA-256 unknown OCSP response status.
    {"ocsp_response_unknown_sha256", "ca_cert", "rsa_cert", EVP_sha256(),
     OCSP_VERIFYSTATUS_SUCCESS, OCSP_RESPFINDSTATUS_SUCCESS,
     V_OCSP_CERTSTATUS_UNKNOWN, "unknown"},
    // Test a SHA-256 OCSP response signed by the correct responder certificate,
    // but not for the requested certificate. (So this would be a completely
    // valid response to a different OCSP request for the other certificate.)
    {"ocsp_response_sha256", "ca_cert", "ecdsa_cert", EVP_sha256(),
     OCSP_VERIFYSTATUS_SUCCESS, OCSP_RESPFINDSTATUS_ERROR, -1, nullptr},
    // Test a SHA-256 OCSP response signed by the wrong responder certificate,
    // but the requested certificate was signed. (however this incorrect OCSP
    // responder certificate is a valid OCSP responder for some other case and
    // chains to a trusted root). Thus, this response is not valid for any
    // request.
    {"ocsp_response_wrong_signer_sha256", "ca_cert", "rsa_cert", EVP_sha256(),
     OCSP_VERIFYSTATUS_ERROR, OCSP_RESPFINDSTATUS_UNDEFINED, -1, nullptr},
};

class OCSPTestAWS : public testing::TestWithParam<OCSPAWSTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPTestAWS, testing::ValuesIn(nTestVectors));

TEST_P(OCSPTestAWS, VerifyOCSPResponse) {
  const OCSPAWSTestVector &t = GetParam();

  std::string data =
      GetTestData(std::string("crypto/ocsp/test/aws/" +
                              std::string(t.ocsp_response) + ".der")
                      .c_str());
  std::vector<uint8_t> ocsp_reponse_data(data.begin(), data.end());

  // OCSP response parsing and verification step.
  bssl::UniquePtr<OCSP_BASICRESP> basic_response;
  bssl::UniquePtr<STACK_OF(X509)> server_cert_chain;
  ExtractAndVerifyBasicOCSP(ocsp_reponse_data, t.cafile, t.server_cert,
                            t.expected_ocsp_verify_status, &basic_response,
                            &server_cert_chain);

  // If OCSP basic verify is successful, we check the OCSP response status.
  if (t.expected_ocsp_verify_status == OCSP_VERIFYSTATUS_SUCCESS) {
    int status = 0;
    ASN1_GENERALIZEDTIME *thisupd, *nextupd;
    CheckOCSP_CERTSTATUS(&basic_response, &server_cert_chain, t.dgst,
                         t.expected_ocsp_resp_find_status, &status, &thisupd,
                         &nextupd);
    // If OCSP response status retrieving is successful, we check if the cert
    // status of the OCSP response is correct.
    if (t.expected_ocsp_resp_find_status == OCSP_RESPFINDSTATUS_SUCCESS) {
      ASSERT_EQ(t.expected_ocsp_cert_status, status);
      ASSERT_EQ(std::string(t.expected_ocsp_cert_status_string),
                std::string(OCSP_cert_status_str(status)));
    }
  }
}

struct OCSPResponseStatusTestVector {
  const char *ocsp_response;
  int expected_ocsp_status;
  const char *expected_ocsp_status_string;
};

static const OCSPResponseStatusTestVector respTestVectors[] = {
    // === Invalid OCSP response requests sent back an OCSP responder ===
    // https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1
    // OCSPResponseStatus: successful
    {"ocsp_response", OCSP_RESPONSE_STATUS_SUCCESSFUL, "successful"},
    // OCSPResponseStatus: malformedRequest
    {"ocsp_response_malformedrequest", OCSP_RESPONSE_STATUS_MALFORMEDREQUEST,
     "malformedrequest"},
    // OCSPResponseStatus: internalError
    {"ocsp_response_internalerror", OCSP_RESPONSE_STATUS_INTERNALERROR,
     "internalerror"},
    // OCSPResponseStatus: tryLater
    {"ocsp_response_trylater", OCSP_RESPONSE_STATUS_TRYLATER, "trylater"},
    // OCSPResponseStatus: sigRequired
    {"ocsp_response_sigrequired", OCSP_RESPONSE_STATUS_SIGREQUIRED,
     "sigrequired"},
    // OCSPResponseStatus: unauthorized
    {"ocsp_response_unauthorized", OCSP_RESPONSE_STATUS_UNAUTHORIZED,
     "unauthorized"},
};

class OCSPResponseStatusTest
    : public testing::TestWithParam<OCSPResponseStatusTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPResponseStatusTest,
                         testing::ValuesIn(respTestVectors));

TEST_P(OCSPResponseStatusTest, OCSPResponseStatus) {
  const OCSPResponseStatusTestVector &t = GetParam();

  std::string data =
      GetTestData(std::string("crypto/ocsp/test/aws/" +
                              std::string(t.ocsp_response) + ".der")
                      .c_str());
  std::vector<uint8_t> ocsp_reponse_data(data.begin(), data.end());

  bssl::UniquePtr<OCSP_RESPONSE> ocsp_response(
      LoadOCSP_RESPONSE(ocsp_reponse_data));
  ASSERT_TRUE(ocsp_response);

  int ret = OCSP_response_status(ocsp_response.get());
  ASSERT_EQ(t.expected_ocsp_status, ret);
  ASSERT_EQ(std::string(t.expected_ocsp_status_string),
            std::string(OCSP_response_status_str(ret)));
}


// === Specific test cases ===

// Test valid OCSP response signed by an OCSP responder along with check for
// correct time field parsing.
TEST(OCSPTest, TestGoodOCSP) {
  std::string data = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response.der").c_str());
  std::vector<uint8_t> ocsp_reponse_data(data.begin(), data.end());

  bssl::UniquePtr<OCSP_BASICRESP> basic_response;
  bssl::UniquePtr<STACK_OF(X509)> server_cert_chain;
  ExtractAndVerifyBasicOCSP(ocsp_reponse_data, "ca_cert", "rsa_cert",
                            OCSP_VERIFYSTATUS_SUCCESS, &basic_response,
                            &server_cert_chain);

  int status = 0;
  ASN1_GENERALIZEDTIME *thisupd, *nextupd;
  CheckOCSP_CERTSTATUS(&basic_response, &server_cert_chain, EVP_sha1(),
                       OCSP_RESPFINDSTATUS_SUCCESS, &status, &thisupd,
                       &nextupd);
  ASSERT_EQ(V_OCSP_CERTSTATUS_GOOD, status);

  // There is 1 |OCSP_SINGLERESP| in the |basic_response|.
  EXPECT_EQ(OCSP_resp_count(basic_response.get()), 1);

  // If OCSP response is verifiable and all good, an OCSP client should check
  // time fields to see if the response is still valid.

  // Check before OCSP was last updated connection timestamp
  time_t connection_time = invalid_before_ocsp_update_time;
  ASSERT_EQ(1, X509_cmp_time(thisupd, &connection_time));
  ASSERT_EQ(1, X509_cmp_time(nextupd, &connection_time));

  // Check valid connection timestamp right after OCSP response was validated.
  connection_time = valid_after_ocsp_update_time;
  ASSERT_EQ(-1, X509_cmp_time(thisupd, &connection_time));
  ASSERT_EQ(1, X509_cmp_time(nextupd, &connection_time));

  // Check valid connection timestamp right before OCSP response expires.
  connection_time = valid_before_ocsp_expire_time;
  ASSERT_EQ(-1, X509_cmp_time(thisupd, &connection_time));
  ASSERT_EQ(1, X509_cmp_time(nextupd, &connection_time));

  // Check expired connection timestamp
  connection_time = invalid_after_ocsp_expire_time;
  ASSERT_EQ(-1, X509_cmp_time(thisupd, &connection_time));
  ASSERT_EQ(-1, X509_cmp_time(nextupd, &connection_time));

  // Check that |OCSP_check_validity| sees that |thisupd| is more than 100
  // seconds in the past, and disallows it.
  EXPECT_FALSE(OCSP_check_validity(thisupd, nextupd, 0, 100));
  uint32_t err = ERR_get_error();
  EXPECT_EQ(OCSP_R_STATUS_TOO_OLD, ERR_GET_REASON(err));

  // We offset "nsec" in |OCSP_check_validity| with a negative timestamp
  // equal to the current time. We offset this larger timestamp on purpose to
  // check if |OCSP_check_validity| can properly reject improper timestamps.
  // This will cause the function to fail in two places, once when checking
  // if "(current_time + nsec) > thisupd [Status Not Yet Valid]", and a second
  // time when checking if "nextupd > (current_time - nsec) [Status Expired]".
  ERR_clear_error();
  EXPECT_FALSE(OCSP_check_validity(thisupd, nextupd, -time(nullptr), -1));
  err = ERR_get_error();
  EXPECT_EQ(OCSP_R_STATUS_NOT_YET_VALID, ERR_GET_REASON(err));
  err = ERR_get_error();
  EXPECT_EQ(OCSP_R_STATUS_EXPIRED, ERR_GET_REASON(err));
  ERR_clear_error();

  // Check that "NEXTUPDATE_BEFORE_THISUPDATE" is properly detected. We have to
  // use |valid_after_ocsp_update_time| instead of |nextupd| to avoid a
  // ticking time-bomb test. |nextupd| will throw an additional
  // "STATUS_NOT_YET_VALID" error until it is a valid timestamp.
  bssl::UniquePtr<ASN1_GENERALIZEDTIME> after_thisupd(
      ASN1_GENERALIZEDTIME_new());
  ASN1_GENERALIZEDTIME_set(after_thisupd.get(), valid_after_ocsp_update_time);
  EXPECT_FALSE(OCSP_check_validity(after_thisupd.get(), thisupd, 0, -1));
  err = ERR_get_error();
  EXPECT_EQ(OCSP_R_STATUS_EXPIRED, ERR_GET_REASON(err));
  err = ERR_get_error();
  EXPECT_EQ(OCSP_R_NEXTUPDATE_BEFORE_THISUPDATE, ERR_GET_REASON(err));

  // We set |nextupd| to NULL on purpose to avoid another ticking time-bomb
  // expiration in our tests. We only check if |thisupd| is a valid timestamp
  // in the past.
  EXPECT_TRUE(OCSP_check_validity(thisupd, nullptr, 0, -1));
}

// Test valid OCSP response, but the data itself is untrusted.
TEST(OCSPTest, TestUntrustedDataOCSP) {
  std::string data = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response.der").c_str());
  std::vector<uint8_t> ocsp_reponse_data(data.begin(), data.end());

  // Mess up a byte right in the middle of the cert.
  ocsp_reponse_data[800] = ocsp_reponse_data[800] + 1;

  bssl::UniquePtr<OCSP_BASICRESP> basic_response;
  bssl::UniquePtr<STACK_OF(X509)> server_cert_chain;
  ExtractAndVerifyBasicOCSP(ocsp_reponse_data, "ca_cert", "rsa_cert",
                            OCSP_VERIFYSTATUS_ERROR, &basic_response,
                            &server_cert_chain);
}


// Test valid OCSP response hashed with sha256 along with check for correct time
// field parsing.
TEST(OCSPTest, TestGoodOCSP_SHA256) {
  std::string data = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response_sha256.der").c_str());
  std::vector<uint8_t> ocsp_reponse_data(data.begin(), data.end());

  bssl::UniquePtr<OCSP_BASICRESP> basic_response;
  bssl::UniquePtr<STACK_OF(X509)> server_cert_chain;
  ExtractAndVerifyBasicOCSP(ocsp_reponse_data, "ca_cert", "rsa_cert",
                            OCSP_VERIFYSTATUS_SUCCESS, &basic_response,
                            &server_cert_chain);

  int status = 0;
  ASN1_GENERALIZEDTIME *thisupd, *nextupd;
  CheckOCSP_CERTSTATUS(&basic_response, &server_cert_chain, EVP_sha256(),
                       OCSP_RESPFINDSTATUS_SUCCESS, &status, &thisupd,
                       &nextupd);
  ASSERT_EQ(V_OCSP_CERTSTATUS_GOOD, status);

  // There is 1 |OCSP_SINGLERESP| in the |basic_response|.
  EXPECT_EQ(OCSP_resp_count(basic_response.get()), 1);

  // If OCSP response is verifiable and all good, an OCSP client should check
  // time fields to see if the response is still valid.

  // Check before OCSP was last updated connection timestamp.
  time_t connection_time = invalid_before_ocsp_update_time_sha256;
  ASSERT_EQ(1, X509_cmp_time(thisupd, &connection_time));
  ASSERT_EQ(1, X509_cmp_time(nextupd, &connection_time));

  // Check valid connection timestamp right after OCSP response was validated.
  connection_time = valid_after_ocsp_update_time_sha256;
  ASSERT_EQ(-1, X509_cmp_time(thisupd, &connection_time));
  ASSERT_EQ(1, X509_cmp_time(nextupd, &connection_time));

  // Check valid connection timestamp right before OCSP response expires.
  connection_time = valid_before_ocsp_expire_time_sha256;
  ASSERT_EQ(-1, X509_cmp_time(thisupd, &connection_time));
  ASSERT_EQ(1, X509_cmp_time(nextupd, &connection_time));

  // Check expired connection timestamp.
  connection_time = invalid_after_ocsp_expire_time_sha256;
  ASSERT_EQ(-1, X509_cmp_time(thisupd, &connection_time));
  ASSERT_EQ(-1, X509_cmp_time(nextupd, &connection_time));
}

struct OCSPResponseVerifyFlagTestVector {
  const char *ocsp_response;
  unsigned long ocsp_verify_flag;
  bool include_expired_signer_cert;
  int expected_ocsp_verify_status;
};

static const OCSPResponseVerifyFlagTestVector
    OCSPResponseVerifyFlagTestVectors[] = {
        {"ocsp_response", OCSP_NOINTERN, false, OCSP_VERIFYSTATUS_ERROR},
        {"ocsp_response", OCSP_NOCHAIN, false, OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_response_expired_signer", OCSP_NOVERIFY, false,
         OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_response_wrong_signer", OCSP_NOVERIFY, false,
         OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_response", OCSP_NOEXPLICIT, false, OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_response", OCSP_TRUSTOTHER, false, OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_response_expired_signer", OCSP_TRUSTOTHER, false,
         OCSP_VERIFYSTATUS_ERROR},
        // |OCSP_TRUSTOTHER| sets |OCSP_NOVERIFY| if the signer cert is included
        // within the parameters.
        {"ocsp_response_expired_signer", OCSP_TRUSTOTHER, true,
         OCSP_VERIFYSTATUS_SUCCESS},
};

class OCSPVerifyFlagTest
    : public testing::TestWithParam<OCSPResponseVerifyFlagTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPVerifyFlagTest,
                         testing::ValuesIn(OCSPResponseVerifyFlagTestVectors));

TEST_P(OCSPVerifyFlagTest, OCSPVerifyFlagTest) {
  const OCSPResponseVerifyFlagTestVector &t = GetParam();

  std::string respData =
      GetTestData(std::string("crypto/ocsp/test/aws/" +
                              std::string(t.ocsp_response) + ".der")
                      .c_str());
  std::vector<uint8_t> ocsp_response_data(respData.begin(), respData.end());

  bssl::UniquePtr<OCSP_RESPONSE> ocsp_response =
      LoadOCSP_RESPONSE(ocsp_response_data);
  ASSERT_TRUE(ocsp_response);

  int ret = OCSP_response_status(ocsp_response.get());
  ASSERT_EQ(OCSP_RESPONSE_STATUS_SUCCESSFUL, ret);

  bssl::UniquePtr<OCSP_BASICRESP> basic_response =
      bssl::UniquePtr<OCSP_BASICRESP>(
          OCSP_response_get1_basic(ocsp_response.get()));
  ASSERT_TRUE(basic_response);

  // Set up trust store and certificate chain.
  bssl::UniquePtr<X509> ca_cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/ca_cert.pem").c_str())
          .c_str()));
  bssl::UniquePtr<X509> server_cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/rsa_cert.pem").c_str())
          .c_str()));

  bssl::UniquePtr<X509_STORE> trust_store(X509_STORE_new());
  X509_STORE_add_cert(trust_store.get(), ca_cert.get());
  bssl::UniquePtr<STACK_OF(X509)> server_cert_chain(
      CertsToStack({server_cert.get(), ca_cert.get()}));
  ASSERT_TRUE(server_cert_chain);
  if (t.include_expired_signer_cert) {
    bssl::UniquePtr<X509> signing_cert(CertFromPEM(
        GetTestData(
            std::string("crypto/ocsp/test/aws/ocsp_expired_cert.pem").c_str())
            .c_str()));
    ASSERT_TRUE(sk_X509_push(server_cert_chain.get(), signing_cert.get()));
    X509_up_ref(signing_cert.get());
  }

  // Does basic verification on OCSP response.
  const int ocsp_verify_status =
      OCSP_basic_verify(basic_response.get(), server_cert_chain.get(),
                        trust_store.get(), t.ocsp_verify_flag);
  ASSERT_EQ(t.expected_ocsp_verify_status, ocsp_verify_status);
}

struct OCSPRequestVerifyFlagTestVector {
  const char *ocsp_request;
  unsigned long ocsp_verify_flag;
  bool include_expired_signer_cert;
  int expected_ocsp_verify_status;
};

static const OCSPRequestVerifyFlagTestVector
    OCSPRequestVerifyFlagTestVectors[] = {
        // Although signatures for OCSP requests are optional according to the
        // RFC, OCSP requests with absent signatures can't be verified.
        {"ocsp_request", 0, false, OCSP_VERIFYSTATUS_ERROR},
        // ocsp_request_signed's certificate is embedded in the request.
        {"ocsp_request_signed", 0, false, OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_request_signed", OCSP_NOINTERN, false, OCSP_VERIFYSTATUS_ERROR},
        {"ocsp_request_signed", OCSP_NOCHAIN, false, OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_request_wrong_signer", 0, false, OCSP_VERIFYSTATUS_ERROR},
        {"ocsp_request_wrong_signer", OCSP_NOCHAIN, false,
         OCSP_VERIFYSTATUS_ERROR},
        // OCSP request verification will fail if the cert can't be found or
        // when working with an expired cert.
        {"ocsp_request_expired_signer", 0, true, OCSP_VERIFYSTATUS_ERROR},
        {"ocsp_request_expired_signer", 0, false, OCSP_VERIFYSTATUS_ERROR},
        {"ocsp_request_expired_signer_no_certs", 0, true,
         OCSP_VERIFYSTATUS_ERROR},
        {"ocsp_request_expired_signer_no_certs", 0, false,
         OCSP_VERIFYSTATUS_ERROR},
        // |OCSP_NOVERIFY| skips certificate verification, but can't succeed if
        // the signer cert isn't found.
        {"ocsp_request_expired_signer", OCSP_NOVERIFY, true,
         OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_request_expired_signer", OCSP_NOVERIFY, false,
         OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_request_expired_signer_no_certs", OCSP_NOVERIFY, true,
         OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_request_expired_signer_no_certs", OCSP_NOVERIFY, false,
         OCSP_VERIFYSTATUS_ERROR},
        // |OCSP_TRUSTOTHER| sets |OCSP_NOVERIFY| if the signer cert is included
        // within the parameters.
        {"ocsp_request_expired_signer", OCSP_TRUSTOTHER, true,
         OCSP_VERIFYSTATUS_ERROR},
        {"ocsp_request_expired_signer", OCSP_TRUSTOTHER, false,
         OCSP_VERIFYSTATUS_ERROR},
        {"ocsp_request_expired_signer_no_certs", OCSP_TRUSTOTHER, true,
         OCSP_VERIFYSTATUS_SUCCESS},
        {"ocsp_request_expired_signer_no_certs", OCSP_TRUSTOTHER, false,
         OCSP_VERIFYSTATUS_ERROR},
};

class OCSPRequestVerifyFlagTest
    : public testing::TestWithParam<OCSPRequestVerifyFlagTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPRequestVerifyFlagTest,
                         testing::ValuesIn(OCSPRequestVerifyFlagTestVectors));

TEST_P(OCSPRequestVerifyFlagTest, OCSPVerifyFlagTest) {
  const OCSPRequestVerifyFlagTestVector &t = GetParam();

  std::string respData =
      GetTestData(std::string("crypto/ocsp/test/aws/" +
                              std::string(t.ocsp_request) + ".der")
                      .c_str());
  std::vector<uint8_t> ocsp_request_data(respData.begin(), respData.end());
  bssl::UniquePtr<OCSP_REQUEST> ocsp_request =
      LoadOCSP_REQUEST(ocsp_request_data);
  ASSERT_TRUE(ocsp_request);

  // Set up trust store and certificate chain.
  bssl::UniquePtr<X509> ca_cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/ca_cert.pem").c_str())
          .c_str()));

  bssl::UniquePtr<X509_STORE> trust_store(X509_STORE_new());
  X509_STORE_add_cert(trust_store.get(), ca_cert.get());
  bssl::UniquePtr<STACK_OF(X509)> server_cert_chain(sk_X509_new_null());
  ASSERT_TRUE(server_cert_chain);
  if (t.include_expired_signer_cert) {
    bssl::UniquePtr<X509> signing_cert(CertFromPEM(
        GetTestData(
            std::string("crypto/ocsp/test/aws/ocsp_expired_cert.pem").c_str())
            .c_str()));
    ASSERT_TRUE(sk_X509_push(server_cert_chain.get(), signing_cert.get()));
    X509_up_ref(signing_cert.get());
  }

  // Does basic verification on OCSP request.
  const int ocsp_verify_status =
      OCSP_request_verify(ocsp_request.get(), server_cert_chain.get(),
                          trust_store.get(), t.ocsp_verify_flag);
  ASSERT_EQ(t.expected_ocsp_verify_status, ocsp_verify_status);
}

TEST(OCSPTest, GetInfo) {
  // Create a sample |OCSP_CERTID| structure.
  bssl::UniquePtr<OCSP_CERTID> cert_id(LoadTestOCSP_CERTID());
  ASSERT_TRUE(cert_id);

  ASN1_OCTET_STRING *nameHash = nullptr;
  ASN1_OBJECT *algo = nullptr;
  ASN1_OCTET_STRING *keyHash = nullptr;
  ASN1_INTEGER *serial = nullptr;
  EXPECT_TRUE(
      OCSP_id_get0_info(&nameHash, &algo, &keyHash, &serial, cert_id.get()));
  EXPECT_EQ(nameHash, cert_id.get()->issuerNameHash);
  EXPECT_EQ(algo, cert_id.get()->hashAlgorithm->algorithm);
  EXPECT_EQ(keyHash, cert_id.get()->issuerKeyHash);
  EXPECT_EQ(serial, cert_id.get()->serialNumber);
}

TEST(OCSPTest, BasicAddCert) {
  std::string respData = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response.der").c_str());
  std::vector<uint8_t> ocsp_response_data(respData.begin(), respData.end());
  bssl::UniquePtr<OCSP_RESPONSE> ocspResponse =
      LoadOCSP_RESPONSE(ocsp_response_data);
  bssl::UniquePtr<OCSP_BASICRESP> basicResponse(
      OCSP_response_get1_basic(ocspResponse.get()));
  ASSERT_TRUE(basicResponse);

  bssl::UniquePtr<X509> cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/ca_cert.pem").c_str())
          .c_str()));

  EXPECT_TRUE(OCSP_basic_add1_cert(basicResponse.get(), cert.get()));
  // Check that the cert is the same as the one added to the stack.
  EXPECT_EQ(sk_X509_value(basicResponse.get()->certs,
                          sk_X509_num(basicResponse.get()->certs) - 1),
            cert.get());
}

TEST(OCSPTest, BasicAddStatus) {
  bssl::UniquePtr<OCSP_BASICRESP> basicResponse(OCSP_BASICRESP_new());
  ASSERT_TRUE(basicResponse);
  bssl::UniquePtr<OCSP_CERTID> certId(LoadTestOCSP_CERTID());
  ASSERT_TRUE(certId);

  bssl::UniquePtr<ASN1_TIME> revoked_time(ASN1_TIME_new()),
      this_update(ASN1_TIME_new()), next_update(ASN1_TIME_new());
  ASSERT_TRUE(
      ASN1_TIME_set(revoked_time.get(), invalid_before_ocsp_update_time));
  ASSERT_TRUE(ASN1_TIME_set(this_update.get(), valid_after_ocsp_update_time));
  ASSERT_TRUE(ASN1_TIME_set(next_update.get(), valid_before_ocsp_expire_time));

  EXPECT_TRUE(OCSP_basic_add1_status(basicResponse.get(), certId.get(),
                                     V_OCSP_CERTSTATUS_GOOD, 0, nullptr,
                                     this_update.get(), next_update.get()));

  // |revoked_reason| and |revoked_time| are ignored if |status| is
  // |V_OCSP_CERTSTATUS_GOOD|, but will still succeed.
  EXPECT_TRUE(OCSP_basic_add1_status(
      basicResponse.get(), certId.get(), V_OCSP_CERTSTATUS_GOOD,
      OCSP_REVOKED_STATUS_CACOMPROMISE, revoked_time.get(), this_update.get(),
      next_update.get()));

  EXPECT_TRUE(OCSP_basic_add1_status(basicResponse.get(), certId.get(),
                                     V_OCSP_CERTSTATUS_UNKNOWN, 0, nullptr,
                                     this_update.get(), next_update.get()));

  // |revoked_reason| and |revoked_time| are ignored if |status| is
  // |V_OCSP_CERTSTATUS_UNKNOWN|, but will still succeed.
  EXPECT_TRUE(OCSP_basic_add1_status(
      basicResponse.get(), certId.get(), V_OCSP_CERTSTATUS_UNKNOWN,
      OCSP_REVOKED_STATUS_SUPERSEDED, revoked_time.get(), this_update.get(),
      next_update.get()));

  // Try setting a revoked response without an |ASN1_TIME|.
  EXPECT_FALSE(OCSP_basic_add1_status(basicResponse.get(), certId.get(),
                                      V_OCSP_CERTSTATUS_REVOKED, 0, nullptr,
                                      this_update.get(), nullptr));

  // Try setting a revoked response with an invalid revoked reason number.
  EXPECT_FALSE(OCSP_basic_add1_status(
      basicResponse.get(), certId.get(), V_OCSP_CERTSTATUS_REVOKED,
      OCSP_UNASSIGNED_REVOKED_STATUS, revoked_time.get(), this_update.get(),
      nullptr));

  EXPECT_TRUE(OCSP_basic_add1_status(
      basicResponse.get(), certId.get(), V_OCSP_CERTSTATUS_REVOKED,
      OCSP_REVOKED_STATUS_UNSPECIFIED, revoked_time.get(), this_update.get(),
      nullptr));

  // Try setting an invalid status to the response.
  EXPECT_FALSE(OCSP_basic_add1_status(basicResponse.get(), certId.get(), 4, 0,
                                      nullptr, this_update.get(), nullptr));

  // |OCSP_basic_add1_status| has succeeded 5 times at this point, so the
  // |basicResponse| should have 5 |OCSP_SINGLERESP|s in the internal stack.
  EXPECT_EQ((int)sk_OCSP_SINGLERESP_num(
                basicResponse.get()->tbsResponseData->responses),
            5);
}

TEST(OCSPTest, OCSPResponseRecreate) {
  std::string data = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response.der").c_str());
  std::vector<uint8_t> ocsp_response_data(data.begin(), data.end());
  bssl::UniquePtr<OCSP_RESPONSE> ocsp_response(
      LoadOCSP_RESPONSE(ocsp_response_data));
  ASSERT_TRUE(ocsp_response);
  bssl::UniquePtr<OCSP_BASICRESP> basic_response(
      OCSP_response_get1_basic(ocsp_response.get()));
  ASSERT_TRUE(basic_response);

  // Recreate the same |OCSP_RESPONSE| with the same contents.
  bssl::UniquePtr<OCSP_RESPONSE> ocsp_response_recreated(OCSP_response_create(
      OCSP_response_status(ocsp_response.get()), basic_response.get()));
  EXPECT_TRUE(ocsp_response_recreated);

  // Write out the bytes from |ocsp_response_recreated| to compare with the
  // original.
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  EXPECT_TRUE(i2d_OCSP_RESPONSE_bio(bio.get(), ocsp_response_recreated.get()));
  const uint8_t *bio_data;
  size_t bio_len;
  ASSERT_TRUE(BIO_mem_contents(bio.get(), &bio_data, &bio_len));
  EXPECT_EQ(Bytes(bio_data, bio_len),
            Bytes(ocsp_response_data.data(), ocsp_response_data.size()));

  // Disallow creation of an |OCSP_RESPONSE| with an invalid status number.
  EXPECT_FALSE(OCSP_response_create(OCSP_UNASSIGNED_RESPONSE_STATUS,
                                    basic_response.get()));
}

// === Translation of OpenSSL's OCSP tests ===

// https://github.com/openssl/openssl/blob/OpenSSL_1_1_1-stable/test/recipes/80-test_ocsp.t
struct OCSPTestVector {
  const char *ocsp_response;
  const char *cafile;
  const char *untrusted;
  int expected_ocsp_verify_status;
};

// Test vectors from OpenSSL OCSP's tests
static const OCSPTestVector kTestVectors[] = {
    // === VALID OCSP RESPONSES ===
    {"ND1", "ND1_Issuer_ICA", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"ND2", "ND2_Issuer_Root", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"ND3", "ND3_Issuer_Root", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"ND1", "ND1_Cross_Root", "ND1_Issuer_ICA-Cross",
     OCSP_VERIFYSTATUS_SUCCESS},
    {"D1", "D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"D2", "D2_Issuer_Root", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"D3", "D3_Issuer_Root", "", OCSP_VERIFYSTATUS_SUCCESS},
    // === INVALID SIGNATURE on the OCSP RESPONSE ===
    {"ISOP_ND1", "ND1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"ISOP_ND2", "ND2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"ISOP_ND3", "ND3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"ISOP_D1", "D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"ISOP_D2", "D2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"ISOP_D3", "D3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    // === WRONG RESPONDERID in the OCSP RESPONSE ===
    {"WRID_ND1", "ND1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"WRID_ND2", "ND2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WRID_ND3", "ND3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WRID_D1", "D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"WRID_D2", "D2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WRID_D3", "D3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    // === WRONG ISSUERNAMEHASH in the OCSP RESPONSE ===
    {"WINH_ND1", "ND1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"WINH_ND2", "ND2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WINH_ND3", "ND3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WINH_D1", "D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"WINH_D2", "D2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WINH_D3", "D3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    // === WRONG ISSUERKEYHASH in the OCSP RESPONSE ===
    {"WIKH_ND1", "ND1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"WIKH_ND2", "ND2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WIKH_ND3", "ND3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WIKH_D1", "D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"WIKH_D2", "D2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WIKH_D3", "D3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    // === WRONG KEY in the DELEGATED OCSP SIGNING CERTIFICATE ===
    {"WKDOSC_D1", "D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"WKDOSC_D2", "D2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"WKDOSC_D3", "D3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    // === INVALID SIGNATURE on the DELEGATED OCSP SIGNING CERTIFICATE ===
    {"ISDOSC_D1", "D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"ISDOSC_D1", "D2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"ISDOSC_D1", "D3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    // === WRONG SUBJECT NAME in the ISSUER CERTIFICATE ===
    {"ND1", "WSNIC_ND1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"ND2", "WSNIC_ND2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"ND3", "WSNIC_ND3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"D1", "WSNIC_D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"D2", "WSNIC_D2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"D3", "WSNIC_D3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    // === WRONG KEY in the ISSUER CERTIFICATE ===
    {"ND1", "WKIC_ND1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"ND2", "WKIC_ND2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"ND3", "WKIC_ND3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"D1", "WKIC_D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_ERROR},
    {"D2", "WKIC_D2_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    {"D3", "WKIC_D3_Issuer_Root", "", OCSP_VERIFYSTATUS_ERROR},
    // === INVALID SIGNATURE on the ISSUER CERTIFICATE ===
    // Expect success, because we're explicitly trusting the issuer certificate.
    // https://datatracker.ietf.org/doc/html/rfc6960#section-2.6
    {"ND1", "ISIC_ND1_Issuer_ICA", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"ND2", "ISIC_ND2_Issuer_Root", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"ND3", "ISIC_ND3_Issuer_Root", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"D1", "ISIC_D1_Issuer_ICA", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"D2", "ISIC_D2_Issuer_Root", "", OCSP_VERIFYSTATUS_SUCCESS},
    {"D3", "ISIC_D3_Issuer_Root", "", OCSP_VERIFYSTATUS_SUCCESS},
};


class OCSPTest : public testing::TestWithParam<OCSPTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPTest, testing::ValuesIn(kTestVectors));

TEST_P(OCSPTest, VerifyOCSPResponse) {
  const OCSPTestVector &t = GetParam();

  bssl::UniquePtr<OCSP_RESPONSE> ocsp_response;
  bssl::UniquePtr<OCSP_BASICRESP> basic_response;

  // Get OCSP response from path.
  std::string data = GetTestData(
      std::string("crypto/ocsp/test/" + std::string(t.ocsp_response) + ".ors")
          .c_str());
  std::vector<uint8_t> input;
  ASSERT_TRUE(DecodeBase64(&input, data.c_str()));

  ocsp_response = LoadOCSP_RESPONSE(input);
  ASSERT_TRUE(ocsp_response);

  int ret = OCSP_response_status(ocsp_response.get());
  ASSERT_EQ(OCSP_RESPONSE_STATUS_SUCCESSFUL, ret);

  basic_response = bssl::UniquePtr<OCSP_BASICRESP>(
      OCSP_response_get1_basic(ocsp_response.get()));
  ASSERT_TRUE(basic_response);

  // Set up OpenSSL OCSP tests trust store parameters and cert chain.
  bssl::UniquePtr<STACK_OF(X509)> server_cert_chain;
  bssl::UniquePtr<X509_STORE> trust_store(X509_STORE_new());
  bssl::UniquePtr<X509_VERIFY_PARAM> vpm(X509_VERIFY_PARAM_new());
  X509_VERIFY_PARAM_set_time(vpm.get(), (time_t)1355875200);
  // Discussion for whether this flag should be on by default or not:
  // https://github.com/openssl/openssl/issues/7871
  ASSERT_EQ(X509_VERIFY_PARAM_set_flags(vpm.get(), X509_V_FLAG_PARTIAL_CHAIN),
            1);

  // Get CA root certificate from path and set up trust store.
  bssl::UniquePtr<X509> ca_certificate(
      CertFromPEM(GetTestData(std::string("crypto/ocsp/test/" +
                                          std::string(t.cafile) + ".pem")
                                  .c_str())
                      .c_str()));
  X509_STORE_add_cert(trust_store.get(), ca_certificate.get());
  ASSERT_EQ(X509_STORE_set1_param(trust_store.get(), vpm.get()), 1);

  // If untrusted cert chain isn't available, we only use CA cert as root
  // cert.
  if (std::string(t.untrusted).empty()) {
    server_cert_chain = CertsToStack({ca_certificate.get()});
  } else {
    server_cert_chain = CertChainFromPEM(
        GetTestData(
            std::string("crypto/ocsp/test/" + std::string(t.untrusted) + ".pem")
                .c_str())
            .c_str());
  }
  ASSERT_TRUE(server_cert_chain);

  // Does basic verification on OCSP response.
  const int ocsp_verify_status = OCSP_basic_verify(
      basic_response.get(), server_cert_chain.get(), trust_store.get(), 0);
  ASSERT_EQ(t.expected_ocsp_verify_status, ocsp_verify_status);
}

struct OCSPRequestTestVector {
  const char *ocsp_request;
  int expected_parse_status;
  int expected_sign_status;
  const char *signer_cert;
  const char *key_type;
  const EVP_MD *dgst;
};

static const OCSPRequestTestVector kRequestTestVectors[] = {
    {"ocsp_request", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_SUCCESS, "rsa_cert",
     "rsa_key", EVP_sha1()},
    {"ocsp_request", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_SUCCESS, "rsa_cert",
     "rsa_key", EVP_sha256()},
    {"ocsp_request", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_SUCCESS, "rsa_cert",
     "rsa_key", nullptr},
    {"ocsp_request_attached_cert", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_ERROR,
     "rsa_cert", "rsa_key", nullptr},
    {"ocsp_request_no_nonce", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_SUCCESS,
     "rsa_cert", "rsa_key", EVP_sha512()},
    {"ocsp_request_signed", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_ERROR,
     "rsa_cert", "rsa_key", nullptr},
    {"ocsp_request_signed_sha256", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_ERROR,
     "rsa_cert", "rsa_key", nullptr},
    {"ocsp_response", OCSP_REQUEST_PARSE_ERROR, OCSP_SIGN_ERROR, "rsa_cert",
     "rsa_key", nullptr},
    // Test signing with ECDSA certs and keys.
    {"ocsp_request", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_SUCCESS,
     "ecdsa_cert", "ecdsa_key", EVP_sha1()},
    {"ocsp_request", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_SUCCESS,
     "ecdsa_cert", "ecdsa_key", EVP_sha256()},
    {"ocsp_request", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_SUCCESS,
     "ecdsa_cert", "ecdsa_key", nullptr},
    {"ocsp_request_no_nonce", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_SUCCESS,
     "rsa_cert", "rsa_key", EVP_sha256()},
    {"ocsp_request_no_nonce", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_SUCCESS,
     "ecdsa_cert", "ecdsa_key", EVP_sha256()},
    // Test certificate type mismatch.
    {"ocsp_request", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_ERROR, "rsa_cert",
     "ecdsa_key", EVP_sha256()},
    {"ocsp_request", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_ERROR, "ecdsa_cert",
     "rsa_key", EVP_sha256()},
    // Test certificate key and cert mismatch.
    {"ocsp_request", OCSP_REQUEST_PARSE_SUCCESS, OCSP_SIGN_ERROR, "ca_cert",
     "rsa_key", EVP_sha256()},
};

class OCSPRequestTest : public testing::TestWithParam<OCSPRequestTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPRequestTest,
                         testing::ValuesIn(kRequestTestVectors));

static const char good_http_request_hdr[] =
    "POST / HTTP/1.0\r\n"
    "Content-Type: application/ocsp-request\r\n"
    "Content-Length: ";

TEST_P(OCSPRequestTest, OCSPRequestParse) {
  const OCSPRequestTestVector &t = GetParam();

  std::string data =
      GetTestData(std::string("crypto/ocsp/test/aws/" +
                              std::string(t.ocsp_request) + ".der")
                      .c_str());
  std::vector<uint8_t> ocsp_request_data(data.begin(), data.end());

  bssl::UniquePtr<OCSP_REQUEST> ocspRequest =
      LoadOCSP_REQUEST(ocsp_request_data);

  if (t.expected_parse_status == OCSP_REQUEST_PARSE_SUCCESS) {
    ASSERT_TRUE(ocspRequest);

    // If request parsing is successful, try setting up a |OCSP_REQ_CTX| with
    // default settings.
    bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
    const uint8_t *out;
    size_t outlen;
    bssl::UniquePtr<OCSP_REQ_CTX> ocspReqCtx(
        OCSP_sendreq_new(bio.get(), nullptr, ocspRequest.get(), 0));
    ASSERT_TRUE(ocspReqCtx);
    // Get internal memory of |OCSP_REQ_CTX| and ensure contents are written.
    ASSERT_TRUE(BIO_mem_contents(OCSP_REQ_CTX_get0_mem_bio(ocspReqCtx.get()),
                                 &out, &outlen));
    ASSERT_GT(outlen, (size_t)0);

    // Set up |OCSP_REQ_CTX| without a |OCSP_REQUEST|. Then finalize the context
    // later.
    bssl::UniquePtr<BIO> bio2(BIO_new(BIO_s_mem()));
    const uint8_t *out2;
    size_t outlen2;
    bssl::UniquePtr<OCSP_REQ_CTX> ocspReqCtxLater(
        OCSP_sendreq_new(bio2.get(), nullptr, nullptr, 0));
    ASSERT_TRUE(ocspReqCtxLater);
    ASSERT_TRUE(
        OCSP_REQ_CTX_set1_req(ocspReqCtxLater.get(), ocspRequest.get()));
    ASSERT_TRUE(BIO_mem_contents(
        OCSP_REQ_CTX_get0_mem_bio(ocspReqCtxLater.get()), &out2, &outlen2));

    // Ensure header contents are written as expected.
    EXPECT_EQ(
        good_http_request_hdr + std::to_string(ocsp_request_data.size()) + "\r",
        std::string(reinterpret_cast<const char *>(out),
                    sizeof(good_http_request_hdr) +
                        std::to_string(ocsp_request_data.size()).size()));
    // Check |OCSP_REQ_CTX| construction methods write the exact same contents.
    ASSERT_EQ(Bytes(out, outlen), Bytes(out2, outlen2));
  } else {
    ASSERT_FALSE(ocspRequest);
  }
}

TEST_P(OCSPRequestTest, OCSPRequestSign) {
  const OCSPRequestTestVector &t = GetParam();

  std::string data =
      GetTestData(std::string("crypto/ocsp/test/aws/" +
                              std::string(t.ocsp_request) + ".der")
                      .c_str());
  std::vector<uint8_t> ocsp_request_data(data.begin(), data.end());
  bssl::UniquePtr<OCSP_REQUEST> ocspRequest =
      LoadOCSP_REQUEST(ocsp_request_data);

  bssl::UniquePtr<X509> signer_cert(
      CertFromPEM(GetTestData(std::string("crypto/ocsp/test/aws/" +
                                          std::string(t.signer_cert) + ".pem")
                                  .c_str())
                      .c_str()));
  ASSERT_TRUE(signer_cert);
  bssl::UniquePtr<X509> ca_cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/ca_cert.pem").c_str())
          .c_str()));
  ASSERT_TRUE(ca_cert);

  if (t.expected_parse_status == OCSP_REQUEST_PARSE_SUCCESS) {
    bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
    if (std::string(t.key_type) == "rsa_key") {
      bssl::UniquePtr<RSA> rsa(
          RSAFromPEM(GetTestData(std::string("crypto/ocsp/test/aws/" +
                                             std::string(t.key_type) + ".pem")
                                     .c_str())
                         .c_str()));
      ASSERT_TRUE(rsa);
      ASSERT_TRUE(EVP_PKEY_set1_RSA(pkey.get(), rsa.get()));
    } else {
      bssl::UniquePtr<EC_KEY> ecdsa(
          ECDSAFromPEM(GetTestData(std::string("crypto/ocsp/test/aws/" +
                                               std::string(t.key_type) + ".pem")
                                       .c_str())
                           .c_str()));
      ASSERT_TRUE(ecdsa);
      ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(pkey.get(), ecdsa.get()));
    }

    int ret =
        OCSP_request_sign(ocspRequest.get(), signer_cert.get(), pkey.get(),
                          t.dgst, CertsToStack({ca_cert.get()}).get(), 0);
    if (t.expected_sign_status == OCSP_SIGN_SUCCESS) {
      ASSERT_TRUE(ret);
      EXPECT_TRUE(OCSP_request_is_signed(ocspRequest.get()));
      bssl::UniquePtr<X509_STORE> trust_store(X509_STORE_new());
      ASSERT_TRUE(X509_STORE_add_cert(trust_store.get(), ca_cert.get()));
      EXPECT_TRUE(OCSP_request_verify(ocspRequest.get(),
                                      CertsToStack({signer_cert.get()}).get(),
                                      trust_store.get(), 0));
    } else {
      ASSERT_FALSE(ret);
      EXPECT_FALSE(OCSP_request_is_signed(ocspRequest.get()));
    }
  }
}

struct OCSPResponseSignTestVector {
  int expected_sign_status;
  const char *signer_cert;
  const char *key_type;
  const EVP_MD *dgst;
};

static const OCSPResponseSignTestVector kOCSPResponseSignTestVectors[] = {
    {OCSP_SIGN_SUCCESS, "rsa_cert", "rsa_key", EVP_sha1()},
    {OCSP_SIGN_SUCCESS, "rsa_cert", "rsa_key", EVP_sha256()},
    {OCSP_SIGN_SUCCESS, "rsa_cert", "rsa_key", EVP_sha512()},
    {OCSP_SIGN_SUCCESS, "rsa_cert", "rsa_key", nullptr},
    // Test signing with ECDSA certs and keys.
    {OCSP_SIGN_SUCCESS, "ecdsa_cert", "ecdsa_key", EVP_sha1()},
    {OCSP_SIGN_SUCCESS, "ecdsa_cert", "ecdsa_key", EVP_sha256()},
    {OCSP_SIGN_SUCCESS, "ecdsa_cert", "ecdsa_key", EVP_sha512()},
    {OCSP_SIGN_SUCCESS, "ecdsa_cert", "ecdsa_key", nullptr},
    // Test certificate type mismatch.
    {OCSP_SIGN_ERROR, "rsa_cert", "ecdsa_key", EVP_sha256()},
    {OCSP_SIGN_ERROR, "ecdsa_cert", "rsa_key", EVP_sha256()},
    // Test certificate key and cert mismatch.
    {OCSP_SIGN_ERROR, "ca_cert", "rsa_key", EVP_sha256()},
};

class OCSPResponseSignTest
    : public testing::TestWithParam<OCSPResponseSignTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPResponseSignTest,
                         testing::ValuesIn(kOCSPResponseSignTestVectors));

TEST_P(OCSPResponseSignTest, OCSPResponseSign) {
  const OCSPResponseSignTestVector &t = GetParam();

  // Set up an empty |OCSP_BASICRESP| for signing.
  bssl::UniquePtr<OCSP_BASICRESP> basic_response(OCSP_BASICRESP_new());
  ASSERT_TRUE(basic_response);

  // Gather certs and key needed for signing.
  bssl::UniquePtr<X509> signer_cert(
      CertFromPEM(GetTestData(std::string("crypto/ocsp/test/aws/" +
                                          std::string(t.signer_cert) + ".pem")
                                  .c_str())
                      .c_str()));
  ASSERT_TRUE(signer_cert);
  bssl::UniquePtr<X509> ca_cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/ca_cert.pem").c_str())
          .c_str()));
  ASSERT_TRUE(ca_cert);
  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  if (std::string(t.key_type) == "rsa_key") {
    bssl::UniquePtr<RSA> rsa(
        RSAFromPEM(GetTestData(std::string("crypto/ocsp/test/aws/" +
                                           std::string(t.key_type) + ".pem")
                                   .c_str())
                       .c_str()));
    ASSERT_TRUE(rsa);
    ASSERT_TRUE(EVP_PKEY_set1_RSA(pkey.get(), rsa.get()));
  } else {
    bssl::UniquePtr<EC_KEY> ecdsa(
        ECDSAFromPEM(GetTestData(std::string("crypto/ocsp/test/aws/" +
                                             std::string(t.key_type) + ".pem")
                                     .c_str())
                         .c_str()));
    ASSERT_TRUE(ecdsa);
    ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(pkey.get(), ecdsa.get()));
  }

  // Do the actual sign.
  EXPECT_EQ(OCSP_basic_sign(basic_response.get(), signer_cert.get(), pkey.get(),
                            t.dgst, CertsToStack({ca_cert.get()}).get(), 0),
            t.expected_sign_status);
  if (t.expected_sign_status == OCSP_SIGN_SUCCESS) {
    bssl::UniquePtr<X509_STORE> trust_store(X509_STORE_new());
    ASSERT_TRUE(X509_STORE_add_cert(trust_store.get(), ca_cert.get()));
    EXPECT_TRUE(OCSP_basic_verify(basic_response.get(),
                                  CertsToStack({signer_cert.get()}).get(),
                                  trust_store.get(), 0));
  }
}

// Test against various flags for |OCSP_basic_sign|.
TEST(OCSPResponseSignTestExtended, OCSPResponseSign) {
  bssl::UniquePtr<OCSP_BASICRESP> basic_response(OCSP_BASICRESP_new());
  ASSERT_TRUE(basic_response);

  bssl::UniquePtr<X509> signer_cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/rsa_cert.pem").c_str())
          .c_str()));
  ASSERT_TRUE(signer_cert);
  bssl::UniquePtr<X509> ca_cert(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/ca_cert.pem").c_str())
          .c_str()));
  ASSERT_TRUE(ca_cert);
  bssl::UniquePtr<STACK_OF(X509)> additional_cert =
      CertsToStack({ca_cert.get()});

  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  bssl::UniquePtr<RSA> rsa(RSAFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/rsa_key.pem").c_str())
          .c_str()));
  ASSERT_TRUE(rsa);
  ASSERT_TRUE(EVP_PKEY_set1_RSA(pkey.get(), rsa.get()));

  // Call |OCSP_basic_sign| with no flags and check the expected output.
  EXPECT_TRUE(OCSP_basic_sign(basic_response.get(), signer_cert.get(),
                              pkey.get(), EVP_sha256(), additional_cert.get(),
                              0));
  EXPECT_TRUE(
      ASN1_TIME_check(basic_response.get()->tbsResponseData->producedAt));
  // Allow for time field to be within two hours.
  EXPECT_GT(
      X509_cmp_time_posix(basic_response.get()->tbsResponseData->producedAt,
                          time(nullptr) - 3600),
      0);
  EXPECT_LT(
      X509_cmp_time_posix(basic_response.get()->tbsResponseData->producedAt,
                          time(nullptr) + 3600),
      0);

  EXPECT_EQ(basic_response.get()->tbsResponseData->responderId->type,
            V_OCSP_RESPID_NAME);
  EXPECT_EQ(
      X509_NAME_cmp(
          basic_response.get()->tbsResponseData->responderId->value.byName,
          X509_get_subject_name(signer_cert.get())),
      0);
  EXPECT_EQ((int)sk_X509_num(basic_response.get()->certs), 2);
  EXPECT_EQ(X509_cmp(sk_X509_value(basic_response.get()->certs, 0),
                     signer_cert.get()),
            0);
  EXPECT_EQ(X509_cmp(sk_X509_value(basic_response.get()->certs, 1),
                     sk_X509_value(additional_cert.get(), 0)),
            0);

  // Check expected effects of |OCSP_NOTIME|.
  basic_response.reset(OCSP_BASICRESP_new());
  ASSERT_TRUE(basic_response);
  EXPECT_TRUE(OCSP_basic_sign(basic_response.get(), signer_cert.get(),
                              pkey.get(), EVP_sha256(), additional_cert.get(),
                              OCSP_NOTIME));
  EXPECT_FALSE(
      ASN1_TIME_check(basic_response.get()->tbsResponseData->producedAt));

  // Check expected effects of |OCSP_RESPID_KEY|.
  basic_response.reset(OCSP_BASICRESP_new());
  ASSERT_TRUE(basic_response);
  ASSERT_FALSE(basic_response.get()->tbsResponseData->responderId->value.byKey);
  EXPECT_TRUE(OCSP_basic_sign(basic_response.get(), signer_cert.get(),
                              pkey.get(), EVP_sha256(), additional_cert.get(),
                              OCSP_RESPID_KEY));
  EXPECT_EQ(basic_response.get()->tbsResponseData->responderId->type,
            V_OCSP_RESPID_KEY);
  EXPECT_TRUE(basic_response.get()->tbsResponseData->responderId->value.byKey);


  // Check expected effects of |OCSP_NOCERTS|.
  basic_response.reset(OCSP_BASICRESP_new());
  ASSERT_TRUE(basic_response);
  EXPECT_TRUE(OCSP_basic_sign(basic_response.get(), signer_cert.get(),
                              pkey.get(), EVP_sha256(), additional_cert.get(),
                              OCSP_NOCERTS));
  EXPECT_EQ((int)sk_X509_num(basic_response.get()->certs), 0);

  // Regression: re-signing the same |OCSP_BASICRESP| through any sequence of
  // |OCSP_RESPID_KEY| flag combinations must not leak or free the prior
  // responderId union arm through the wrong destructor. ASAN should flag any
  // misuse.
  basic_response.reset(OCSP_BASICRESP_new());
  ASSERT_TRUE(basic_response);
  EXPECT_TRUE(OCSP_basic_sign(basic_response.get(), signer_cert.get(),
                              pkey.get(), EVP_sha256(), additional_cert.get(),
                              0));
  EXPECT_EQ(basic_response.get()->tbsResponseData->responderId->type,
            V_OCSP_RESPID_NAME);
  EXPECT_TRUE(OCSP_basic_sign(basic_response.get(), signer_cert.get(),
                              pkey.get(), EVP_sha256(), additional_cert.get(),
                              OCSP_RESPID_KEY));
  EXPECT_EQ(basic_response.get()->tbsResponseData->responderId->type,
            V_OCSP_RESPID_KEY);
  EXPECT_TRUE(OCSP_basic_sign(basic_response.get(), signer_cert.get(),
                              pkey.get(), EVP_sha256(), additional_cert.get(),
                              OCSP_RESPID_KEY));
  EXPECT_EQ(basic_response.get()->tbsResponseData->responderId->type,
            V_OCSP_RESPID_KEY);
  EXPECT_TRUE(OCSP_basic_sign(basic_response.get(), signer_cert.get(),
                              pkey.get(), EVP_sha256(), additional_cert.get(),
                              0));
  EXPECT_EQ(basic_response.get()->tbsResponseData->responderId->type,
            V_OCSP_RESPID_NAME);
  EXPECT_TRUE(OCSP_basic_sign(basic_response.get(), signer_cert.get(),
                              pkey.get(), EVP_sha256(), additional_cert.get(),
                              0));
  EXPECT_EQ(basic_response.get()->tbsResponseData->responderId->type,
            V_OCSP_RESPID_NAME);
}

static const char extended_good_http_request_hdr[] =
    "POST / HTTP/1.0\r\n"
    "Accept-Charset: character-set\r\n"
    "Content-Type: application/ocsp-request\r\n"
    "Content-Length: ";

// Check HTTP header can be written with OCSP_REQ_CTX_add1_header().
TEST(OCSPRequestTest, AddHeader) {
  std::string data = GetTestData(std::string("crypto/ocsp/test/aws/"
                                             "ocsp_request.der")
                                     .c_str());
  std::vector<uint8_t> ocsp_request_data(data.begin(), data.end());
  bssl::UniquePtr<OCSP_REQUEST> ocspRequest =
      LoadOCSP_REQUEST(ocsp_request_data);

  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  const uint8_t *out;
  size_t outlen;
  bssl::UniquePtr<OCSP_REQ_CTX> ocspReqCtx(
      OCSP_sendreq_new(bio.get(), nullptr, nullptr, 0));
  ASSERT_TRUE(ocspReqCtx);
  // Set an additional HTTP header.
  ASSERT_TRUE(OCSP_REQ_CTX_add1_header(ocspReqCtx.get(), "Accept-Charset",
                                       "character-set"));
  ASSERT_TRUE(OCSP_REQ_CTX_set1_req(ocspReqCtx.get(), ocspRequest.get()));

  ASSERT_TRUE(BIO_mem_contents(OCSP_REQ_CTX_get0_mem_bio(ocspReqCtx.get()),
                               &out, &outlen));

  // Ensure additional header contents are written as expected.
  EXPECT_EQ(
      extended_good_http_request_hdr +
          std::to_string(ocsp_request_data.size()) + "\r\n",
      std::string(reinterpret_cast<const char *>(out),
                  sizeof(extended_good_http_request_hdr) +
                      std::to_string(ocsp_request_data.size()).size() + 1));
}

// Check a |OCSP_CERTID| can be added to an |OCSP_REQUEST| with
// OCSP_request_add0_id().
TEST(OCSPRequestTest, AddCert) {
  std::string data = GetTestData(std::string("crypto/ocsp/test/aws/"
                                             "ocsp_request.der")
                                     .c_str());
  std::vector<uint8_t> ocsp_request_data(data.begin(), data.end());
  bssl::UniquePtr<OCSP_REQUEST> ocspRequest =
      LoadOCSP_REQUEST(ocsp_request_data);
  ASSERT_TRUE(ocspRequest);

  // Construct |OCSP_CERTID| from certs.
  OCSP_CERTID *certId = LoadTestOCSP_CERTID();
  ASSERT_TRUE(certId);

  OCSP_ONEREQ *oneRequest = OCSP_request_add0_id(ocspRequest.get(), certId);
  ASSERT_TRUE(oneRequest);
  // OCSP_request_add0_id() allocates a new |OCSP_ONEREQ| and assigns the
  // pointer to |OCSP_CERTID| to it, then adds it to the stack within
  // |OCSP_REQUEST|.
  // |OCSP_REQUEST| now takes ownership of the pointer to |OCSP_CERTID| and
  // still maintains ownership of the pointer |OCSP_ONEREQ|, so we have to set
  // the references to NULL to avoid freeing the same pointers twice.
  oneRequest = nullptr;
  certId = nullptr;
}

static const char good_http_response_hdr[] =
    "HTTP/1.1 200 OK\r\n"
    "Content-Type: application/ocsp-response\r\n"
    "Content-Length: ";

// Test that |OCSP_set_max_response_length| correctly sets the maximum response
// length.
TEST(OCSPRequestTest, SetResponseLength) {
  std::string data = GetTestData(std::string("crypto/ocsp/test/aws/"
                                             "ocsp_request.der")
                                     .c_str());
  std::vector<uint8_t> ocsp_request_data(data.begin(), data.end());
  bssl::UniquePtr<OCSP_REQUEST> ocspRequest =
      LoadOCSP_REQUEST(ocsp_request_data);
  ASSERT_TRUE(ocspRequest);

  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  bssl::UniquePtr<OCSP_REQ_CTX> ocspReqCtx(
      OCSP_sendreq_new(bio.get(), nullptr, nullptr, 0));
  ASSERT_TRUE(ocspReqCtx);

  // Check custom length is set correctly.
  int new_len = 40000;
  OCSP_set_max_response_length(ocspReqCtx.get(), new_len);
  EXPECT_EQ((int)ocspReqCtx.get()->max_resp_len, new_len);

  // Check that default length is set correctly.
  OCSP_set_max_response_length(ocspReqCtx.get(), 0);
  EXPECT_EQ((int)ocspReqCtx.get()->max_resp_len, OCSP_MAX_RESP_LENGTH);

  // Write an HTTP OCSP response into the BIO.
  std::string respData = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response.der").c_str());
  std::vector<uint8_t> ocsp_response_data(respData.begin(), respData.end());
  const int respLen = (int)ocsp_response_data.size();
  ASSERT_GT(BIO_printf(bio.get(), "%s", good_http_response_hdr), 0);
  ASSERT_GT(BIO_printf(bio.get(), "%d\r\n\r\n", respLen), 0);
  ASSERT_EQ(BIO_write(bio.get(), ocsp_response_data.data(), respLen), respLen);

  // Set max response length to a length that is too short on purpose.
  OCSP_set_max_response_length(ocspReqCtx.get(), 1);

  // Sends out an OCSP request and expects an OCSP response in |BIO| with
  // |OCSP_REQ_CTX_nbio|. This should fail if the expected length is too short.
  EXPECT_FALSE(OCSP_REQ_CTX_nbio(ocspReqCtx.get()));
}

static const char malformed_http_response_hdr[] =
    "HTTP/1.200 OK\r\n"
    "Content-Type: application/ocsp-response\r\n"
    "Content-Length: ";

// This parses in OpenSSL, but fails in AWS-LC because we check for the HTTP
// protocol characters at the front.
static const char non_http_response_hdr[] =
    "HTPT/1.1 200 OK\r\n"
    "Content-Type: application/ocsp-response\r\n"
    "Content-Length: ";

// Only status code 200 should be accepted.
static const char not_ok_http_response_hdr[] =
    "HTTP/1.1 404 OK\r\n"
    "Content-Type: application/ocsp-response\r\n"
    "Content-Length: ";

// This should parse. Only the status code is mandatory, subsequent lines are
// optional.
static const char no_type_http_response_hdr[] =
    "HTTP/1.1 200 OK\r\n"
    "Content-Length: ";

// This should parse. Only the status code is mandatory, additional information
// is optional.
static const char no_info_http_response_hdr[] =
    "HTTP/1.0 200\r\n"
    "Content-Length: ";

// OpenSSL uses isspace() to test for white-space characters, which includes
// the following. ``\t''``\n''``\v''``\f''``\r''`` '
// This should parse. "\n" is not included since it will skip the header to the
// next line and fail the http parsing.
static const char additional_space_http_response_hdr[] =
    "HTTP/1.1 \t\v\r\f\t 200  \t\v\f\r\t   OK   \r\n"
    "Content-Type: application/ocsp-response\r\n"
    "Content-Length: ";

// This should fail, since the status code is expected on the first line.
static const char next_line_http_response_hdr[] =
    "HTTP/1.1   \n   200    \f   OK\r\n"
    "Content-Type: application/ocsp-response\r\n"
    "Content-Length: ";

// This should fail. Protocol info is expected at the front.
static const char only_code_http_response_hdr[] =
    "200\r\n"
    "Content-Length: ";

// This should fail. We're appending a negative content length.
static const char negative_length_http_response_hdr[] =
    "HTTP/1.0 200\r\n"
    "Content-Length: -10";

// This should fail. We're appending a non-numeric content length.
static const char nonnumeric_length_http_response_hdr[] =
    "HTTP/1.0 200\r\n"
    "Content-Length: abcd";

// This should fail. We're appending a non-numeric content length.
static const char format_string_http_response_hdr[] =
    "HTTP/1.0 200\r\n"
    "Content-Length: %s%s%s%s%s";

// This should fail. We're appending a value that will be beyond the default max
// response length.
static const char max_length_http_response_hdr[] =
    "HTTP/1.0 200\r\n"
    "Content-Length: 999999";

struct OCSPHTTPTestVector {
  const char *http_header;
  bool response_attached;
  int expected_status;
};

static const OCSPHTTPTestVector kResponseHTTPVectors[] = {
    {good_http_response_hdr, true, OCSP_HTTP_PARSE_SUCCESS},
    {malformed_http_response_hdr, true, OCSP_HTTP_PARSE_ERROR},
    {non_http_response_hdr, true, OCSP_HTTP_PARSE_ERROR},
    {not_ok_http_response_hdr, true, OCSP_HTTP_PARSE_ERROR},
    {no_type_http_response_hdr, true, OCSP_REQUEST_PARSE_SUCCESS},
    {no_info_http_response_hdr, true, OCSP_REQUEST_PARSE_SUCCESS},
    {additional_space_http_response_hdr, true, OCSP_REQUEST_PARSE_SUCCESS},
    {next_line_http_response_hdr, true, OCSP_HTTP_PARSE_ERROR},
    {only_code_http_response_hdr, true, OCSP_HTTP_PARSE_ERROR},
    // HTTP Header is OK, but no actual response is attached.
    {good_http_response_hdr, false, OCSP_HTTP_PARSE_ERROR},
    {no_type_http_response_hdr, false, OCSP_HTTP_PARSE_ERROR},
    {no_info_http_response_hdr, false, OCSP_HTTP_PARSE_ERROR},
    {negative_length_http_response_hdr, false, OCSP_HTTP_PARSE_ERROR},
    {nonnumeric_length_http_response_hdr, false, OCSP_HTTP_PARSE_ERROR},
    {format_string_http_response_hdr, false, OCSP_HTTP_PARSE_ERROR},
    {max_length_http_response_hdr, false, OCSP_HTTP_PARSE_ERROR}};

class OCSPHTTPTest : public testing::TestWithParam<OCSPHTTPTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPHTTPTest,
                         testing::ValuesIn(kResponseHTTPVectors));

// Test if OCSP_sendreq_bio() can properly send out an HTTP request with
// a |OCSP_REQUEST|, and expect an HTTP response with an OCSP response back.
// Always sends the same OCSP request and replies with the same OCSP response.
// This test focuses parsing the OCSP response with HTTP. We have other tests
// to test if the actual OCSP response is parsable and verifiable.
TEST_P(OCSPHTTPTest, OCSPRequestHTTP) {
  const OCSPHTTPTestVector &t = GetParam();

  std::string data =
      GetTestData(std::string("crypto/ocsp/test/aws/ocsp_request.der").c_str());
  std::vector<uint8_t> ocsp_request_data(data.begin(), data.end());
  std::string respData = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response.der").c_str());
  std::vector<uint8_t> ocsp_response_data(respData.begin(), respData.end());

  // Write an HTTP OCSP response into the BIO.
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  const int respLen = (int)ocsp_response_data.size();
  ASSERT_GT(BIO_printf(bio.get(), "%s", t.http_header), 0);
  ASSERT_GT(BIO_printf(bio.get(), "%d\r\n\r\n", respLen), 0);
  if (t.response_attached) {
    ASSERT_EQ(BIO_write(bio.get(), ocsp_response_data.data(), respLen),
              respLen);
  }

  // Sends out an OCSP request and expects an OCSP response in |BIO|.
  bssl::UniquePtr<OCSP_REQUEST> ocspRequest =
      LoadOCSP_REQUEST(ocsp_request_data);
  ASSERT_TRUE(ocspRequest);
  bssl::UniquePtr<OCSP_RESPONSE> ocspResponse(
      OCSP_sendreq_bio(bio.get(), nullptr, ocspRequest.get()));
  if (t.expected_status == OCSP_HTTP_PARSE_SUCCESS && t.response_attached) {
    ASSERT_TRUE(ocspResponse);
  } else {
    ASSERT_FALSE(ocspResponse);
  }
}

struct OCSPURLTestVector {
  const char *ocsp_url;
  const char *expected_host;
  const char *expected_port;
  const char *expected_path;
  int expected_ssl;
  int expected_status;
};

static const OCSPURLTestVector kOCSPURLVectors[] = {
    // === VALID URLs ===
    {"http://ocsp.example.com/", "ocsp.example.com", "80", "/", 0,
     OCSP_URL_PARSE_SUCCESS},
    // Non-standard port.
    {"http://ocsp.example.com:8080/", "ocsp.example.com", "8080", "/", 0,
     OCSP_URL_PARSE_SUCCESS},
    {"http://ocsp.example.com/ocsp", "ocsp.example.com", "80", "/ocsp", 0,
     OCSP_URL_PARSE_SUCCESS},
    // Port for HTTPS is 443.
    {"https://ocsp.example.com/", "ocsp.example.com", "443", "/", 1,
     OCSP_URL_PARSE_SUCCESS},
    // Empty path, the default should be "/".
    {"https://ocsp.example.com", "ocsp.example.com", "443", "/", 1,
     OCSP_URL_PARSE_SUCCESS},
    // Parsing an IPv6 host address.
    {"http://[2001:db8::1]/ocsp", "2001:db8::1", "80", "/ocsp", 0,
     OCSP_URL_PARSE_SUCCESS},
    {"https://[2001:db8::1]/ocsp", "2001:db8::1", "443", "/ocsp", 1,
     OCSP_URL_PARSE_SUCCESS},
    // Parsing a URL with an invalid port, but OpenSSL still parses the string
    // where the port is expected.
    {"http://ocsp.example.com:invalid/", "ocsp.example.com", "invalid", "/", 0,
     OCSP_URL_PARSE_SUCCESS},
    // Empty host component.
    {"http:///ocsp", "", "80", "/ocsp", 0, OCSP_URL_PARSE_SUCCESS},
    // Potential Format String attacks.
    {"http://%s%s%s%s%s/ocsp", "%s%s%s%s%s", "80", "/ocsp", 0,
     OCSP_URL_PARSE_SUCCESS},
    {"http://%s%s%s%s%s, argv[1]/ocsp", "%s%s%s%s%s, argv[1]", "80", "/ocsp", 0,
     OCSP_URL_PARSE_SUCCESS},
    {"http://ocsp/%s%s%s%s%s", "ocsp", "80", "/%s%s%s%s%s", 0,
     OCSP_URL_PARSE_SUCCESS},

    // === INVALID URLs ===
    // Not http or https in front.
    {"htp://ocsp.example.com/", nullptr, nullptr, nullptr, 0,
     OCSP_URL_PARSE_ERROR},
    {"%s%s%s%s://%s%s%s%s%s/ocsp", nullptr, nullptr, nullptr, 0,
     OCSP_URL_PARSE_ERROR},
    // Double slash is mandatory.
    {"http:/ocsp.example.com/", nullptr, nullptr, nullptr, 0,
     OCSP_URL_PARSE_ERROR},
    // Malformed.
    {"httocsp.example.com/", nullptr, nullptr, nullptr, 0,
     OCSP_URL_PARSE_ERROR},
    // No closing bracket for ipv6.
    {"http://[2001:db8::1/", nullptr, nullptr, nullptr, 0,
     OCSP_URL_PARSE_ERROR},
    // Protocol must match exactly, not just as a prefix.
    {"https1://ocsp.example.com/", nullptr, nullptr, nullptr, 0,
     OCSP_URL_PARSE_ERROR},
    {"httpss://ocsp.example.com/", nullptr, nullptr, nullptr, 0,
     OCSP_URL_PARSE_ERROR},
    {"httpe://ocsp.example.com/path", nullptr, nullptr, nullptr, 0,
     OCSP_URL_PARSE_ERROR},
};

class OCSPURLTest : public testing::TestWithParam<OCSPURLTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPURLTest, testing::ValuesIn(kOCSPURLVectors));

TEST_P(OCSPURLTest, OCSPParseURL) {
  const OCSPURLTestVector &t = GetParam();

  char *host = nullptr;
  char *path = nullptr;
  char *port = nullptr;
  int is_ssl;

  int ret = OCSP_parse_url(t.ocsp_url, &host, &port, &path, &is_ssl);
  if (t.expected_status == OCSP_URL_PARSE_SUCCESS) {
    EXPECT_EQ(ret, OCSP_URL_PARSE_SUCCESS);
    EXPECT_EQ(std::string(host), std::string(t.expected_host));
    EXPECT_EQ(std::string(port), std::string(t.expected_port));
    EXPECT_EQ(std::string(path), std::string(t.expected_path));
    OPENSSL_free(host);
    OPENSSL_free(path);
    OPENSSL_free(port);
  } else {
    EXPECT_EQ(ret, OCSP_URL_PARSE_ERROR);
    EXPECT_FALSE(host);
    EXPECT_FALSE(port);
    EXPECT_FALSE(path);
    EXPECT_EQ(is_ssl, 0);
  }
}

// Nonce value used in "crypto/ocsp/test/aws/ocsp_response.der".
static unsigned char ocsp_response_nonce[] = {
    0xaf, 0xab, 0xd4, 0xec, 0x6a, 0x17, 0x2c, 0x4a,
    0x98, 0xfb, 0x1a, 0x6d, 0x22, 0xff, 0x29, 0x28};

struct OCSPNonceTestVector {
  const char *ocsp_request;
  const char *ocsp_response;
  bool add_nonce;
  unsigned char *nonce;
  int nonce_len;
  int nonce_check_status;
};

static const OCSPNonceTestVector kNonceTestVectors[] = {
    // Add a nonce to an OCSP request without a nonce.
    {"ocsp_request_no_nonce", "ocsp_response", true, ocsp_response_nonce,
     sizeof(ocsp_response_nonce), OCSP_NONCE_EQUAL},
    // Add a nonce to an OCSP request with a nonce. The original nonce should be
    // overwritten.
    {"ocsp_request", "ocsp_response", true, ocsp_response_nonce,
     sizeof(ocsp_response_nonce), OCSP_NONCE_EQUAL},
    // Generate a random nonce, which should be different from the nonce
    // available in the OCSP response (unless we have a collision).
    {"ocsp_request", "ocsp_response", true, nullptr, 0, OCSP_NONCE_NOT_EQUAL},
    // OCSP request with no nonce, but an OCSP response that does have one.
    {"ocsp_request_no_nonce", "ocsp_response", false, nullptr, 0,
     OCSP_NONCE_RESPONSE_ONLY},
    // OCSP request with a nonce, but an OCSP response that does not have one.
    {"ocsp_request", "ocsp_response_no_nonce", false, nullptr, 0,
     OCSP_NONCE_REQUEST_ONLY},
    // An OCSP request and an OCSP response that don't have a nonce to compare.
    {"ocsp_request_no_nonce", "ocsp_response_no_nonce", false, nullptr, 0,
     OCSP_NONCE_BOTH_ABSENT},
};

class OCSPNonceTest : public testing::TestWithParam<OCSPNonceTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, OCSPNonceTest,
                         testing::ValuesIn(kNonceTestVectors));

TEST_P(OCSPNonceTest, OCSPNonce) {
  const OCSPNonceTestVector &t = GetParam();

  std::string data =
      GetTestData(std::string("crypto/ocsp/test/aws/" +
                              std::string(t.ocsp_request) + ".der")
                      .c_str());
  std::vector<uint8_t> ocsp_request_data(data.begin(), data.end());
  bssl::UniquePtr<OCSP_REQUEST> ocspRequest =
      LoadOCSP_REQUEST(ocsp_request_data);
  std::string respData =
      GetTestData(std::string("crypto/ocsp/test/aws/" +
                              std::string(t.ocsp_response) + ".der")
                      .c_str());
  std::vector<uint8_t> ocsp_response_data(respData.begin(), respData.end());
  bssl::UniquePtr<OCSP_RESPONSE> ocspResponse =
      LoadOCSP_RESPONSE(ocsp_response_data);
  bssl::UniquePtr<OCSP_BASICRESP> basicResponse(
      OCSP_response_get1_basic(ocspResponse.get()));
  ASSERT_TRUE(basicResponse);

  if (t.add_nonce) {
    // Adding a nonce should succeed. The original nonce will get overwritten
    // if one already exists.
    EXPECT_TRUE(
        OCSP_request_add1_nonce(ocspRequest.get(), t.nonce, t.nonce_len));

    // Check if hard coded nonce value has been written correctly.
    if (t.nonce) {
      int req_idx = OCSP_REQUEST_get_ext_by_NID(ocspRequest.get(),
                                                NID_id_pkix_OCSP_Nonce, -1);
      const ASN1_OCTET_STRING *nonce_data = X509_EXTENSION_get_data(
          OCSP_REQUEST_get_ext(ocspRequest.get(), req_idx));

      // The ASN.1 Octet String data type encoding begins with a Tag byte of
      // 0x04. The second byte is the length of the encoded value, so we compare
      // the rest of the bytes to ensure the nonce value was written correctly.
      // https://www.ietf.org/rfc/rfc6025.html#section-2.1.2
      EXPECT_EQ(Bytes(&ASN1_STRING_get0_data(nonce_data)[2],
                      ASN1_STRING_length(nonce_data) - 2),
                Bytes(t.nonce, t.nonce_len));
    }
  }
  EXPECT_EQ(OCSP_check_nonce(ocspRequest.get(), basicResponse.get()),
            t.nonce_check_status);

  // Check that nonce copying from |req| to |bs| also works as expected.
  if (t.nonce_check_status == OCSP_NONCE_RESPONSE_ONLY ||
      t.nonce_check_status == OCSP_NONCE_BOTH_ABSENT) {
    EXPECT_EQ(OCSP_copy_nonce(basicResponse.get(), ocspRequest.get()), 2);
    EXPECT_EQ(OCSP_check_nonce(ocspRequest.get(), basicResponse.get()),
              t.nonce_check_status);
  } else {
    // OpenSSL's implementation of |OCSP_copy_nonce| keeps the original nonce in
    // |resp| at the start of the list. We have to remove the old nonce prior to
    // copying the new one over.
    int resp_idx = OCSP_BASICRESP_get_ext_by_NID(basicResponse.get(),
                                                 NID_id_pkix_OCSP_Nonce, -1);
    if (resp_idx >= 0) {
      bssl::UniquePtr<X509_EXTENSION> old_resp_ext(
          OCSP_BASICRESP_delete_ext(basicResponse.get(), resp_idx));
    }
    EXPECT_EQ(OCSP_copy_nonce(basicResponse.get(), ocspRequest.get()), 1);
    EXPECT_EQ(OCSP_check_nonce(ocspRequest.get(), basicResponse.get()),
              OCSP_NONCE_EQUAL);
  }
}

TEST(OCSPTest, OCSPNonce) {
  std::string data = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_request_no_nonce.der").c_str());
  std::vector<uint8_t> ocsp_request_data(data.begin(), data.end());
  bssl::UniquePtr<OCSP_REQUEST> ocspRequest =
      LoadOCSP_REQUEST(ocsp_request_data);

  // Adding a nonce but using 0 to trigger the default specified length
  // should fail. A length must be specified when adding a specified nonce.
  EXPECT_FALSE(
      OCSP_request_add1_nonce(ocspRequest.get(), ocsp_response_nonce, 0));

  // Adding a random nonce with the default length should succeed.
  // |OCSP_REQUEST_get_ext_by_NID| returns a negative number if a nonce does
  // not exist.
  EXPECT_LT(OCSP_REQUEST_get_ext_by_NID(ocspRequest.get(),
                                        NID_id_pkix_OCSP_Nonce, -1),
            0);
  EXPECT_TRUE(OCSP_request_add1_nonce(ocspRequest.get(), nullptr, 0));
  EXPECT_GE(OCSP_REQUEST_get_ext_by_NID(ocspRequest.get(),
                                        NID_id_pkix_OCSP_Nonce, -1),
            0);

  // Same tests as above, but against an |OCSP_BASICRESP|.
  data = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response_no_nonce.der").c_str());
  std::vector<uint8_t> ocsp_response_data(data.begin(), data.end());
  bssl::UniquePtr<OCSP_RESPONSE> ocspResponse =
      LoadOCSP_RESPONSE(ocsp_response_data);
  ASSERT_TRUE(ocspResponse);
  bssl::UniquePtr<OCSP_BASICRESP> basicResponse(
      OCSP_response_get1_basic(ocspResponse.get()));
  ASSERT_TRUE(basicResponse);

  EXPECT_FALSE(
      OCSP_basic_add1_nonce(basicResponse.get(), ocsp_response_nonce, 0));

  // Adding a random nonce with the default length should succeed.
  // |OCSP_BASICRESP_get_ext_by_NID| returns a negative number if a nonce does
  // not exist.
  // Add a random nonce with a specified length this time around.
  EXPECT_LT(OCSP_BASICRESP_get_ext_by_NID(basicResponse.get(),
                                          NID_id_pkix_OCSP_Nonce, -1),
            0);
  EXPECT_TRUE(OCSP_basic_add1_nonce(basicResponse.get(), nullptr, 10));
  EXPECT_GE(OCSP_BASICRESP_get_ext_by_NID(basicResponse.get(),
                                          NID_id_pkix_OCSP_Nonce, -1),
            0);
}

TEST(OCSPTest, OCSPCRLString) {
  for (int reason_code = 0; reason_code < 11; reason_code++) {
    if (reason_code == 7) {
      // Reason Code 7 is not used.
      EXPECT_EQ("(UNKNOWN)", std::string(OCSP_crl_reason_str(7)));
      continue;
    }
    EXPECT_NE("(UNKNOWN)", std::string(OCSP_crl_reason_str(reason_code)));
  }
  // More unexpected cases.
  EXPECT_EQ("(UNKNOWN)", std::string(OCSP_crl_reason_str(100)));
  EXPECT_EQ("(UNKNOWN)", std::string(OCSP_crl_reason_str(-1)));
  EXPECT_EQ("(UNKNOWN)", std::string(OCSP_crl_reason_str(-100)));
}

TEST(OCSPTest, OCSPBIOTest) {
  std::string reqData =
      GetTestData(std::string("crypto/ocsp/test/aws/ocsp_request.der").c_str());
  std::vector<uint8_t> ocsp_request_data(reqData.begin(), reqData.end());
  std::string respData = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response.der").c_str());
  std::vector<uint8_t> ocsp_response_data(respData.begin(), respData.end());

  // Write an OCSP response into the BIO.
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  int respLen = (int)ocsp_response_data.size();
  ASSERT_EQ(BIO_write(bio.get(), ocsp_response_data.data(), respLen), respLen);
  bssl::UniquePtr<OCSP_RESPONSE> ocspResponse(
      d2i_OCSP_RESPONSE_bio(bio.get(), nullptr));
  EXPECT_TRUE(ocspResponse);

  // Reserialize the OCSP response.
  EXPECT_TRUE(i2d_OCSP_RESPONSE_bio(bio.get(), ocspResponse.get()));
  const uint8_t *bio_data;
  size_t bio_len;
  ASSERT_TRUE(BIO_mem_contents(bio.get(), &bio_data, &bio_len));
  EXPECT_EQ(Bytes(bio_data, bio_len),
            Bytes(ocsp_response_data.data(), respLen));

  // Write an OCSP request into the BIO, but it shouldn't successfully parse
  // with |d2i_OCSP_RESPONSE_bio|.
  ASSERT_TRUE(BIO_reset(bio.get()));
  const int reqLen = (int)ocsp_request_data.size();
  ASSERT_EQ(BIO_write(bio.get(), ocsp_request_data.data(), reqLen), reqLen);
  ocspResponse.reset(d2i_OCSP_RESPONSE_bio(bio.get(), nullptr));
  EXPECT_FALSE(ocspResponse);

  // Write an OCSP request into the BIO.
  bio.reset(BIO_new(BIO_s_mem()));
  ASSERT_EQ(BIO_write(bio.get(), ocsp_request_data.data(), reqLen), reqLen);
  bssl::UniquePtr<OCSP_REQUEST> ocspRequest(
      d2i_OCSP_REQUEST_bio(bio.get(), nullptr));
  EXPECT_TRUE(ocspRequest);

  // Reserialize the OCSP request.
  EXPECT_TRUE(i2d_OCSP_REQUEST_bio(bio.get(), ocspRequest.get()));
  ASSERT_TRUE(BIO_mem_contents(bio.get(), &bio_data, &bio_len));
  EXPECT_EQ(Bytes(bio_data, bio_len), Bytes(ocsp_request_data.data(), reqLen));

  // Write an OCSP response into the BIO, but it shouldn't successfully parse
  // with |d2i_OCSP_REQUEST_bio|.
  ASSERT_TRUE(BIO_reset(bio.get()));
  ASSERT_EQ(BIO_write(bio.get(), ocsp_response_data.data(), respLen), respLen);
  ocspRequest.reset(d2i_OCSP_REQUEST_bio(bio.get(), nullptr));
  EXPECT_FALSE(ocspRequest);
}

TEST(OCSPTest, CertIDDup) {
  bssl::UniquePtr<X509> issuer(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/ca_cert.pem").c_str())
          .c_str()));
  bssl::UniquePtr<X509> subject(CertFromPEM(
      GetTestData(std::string("crypto/ocsp/test/aws/rsa_cert.pem").c_str())
          .c_str()));

  // Create a sample OCSP_CERTID structure
  bssl::UniquePtr<OCSP_CERTID> orig_id(
      OCSP_cert_to_id(EVP_sha256(), subject.get(), issuer.get()));
  ASSERT_TRUE(orig_id);

  // Duplicate the OCSP_CERTID structure
  bssl::UniquePtr<OCSP_CERTID> dup_id(OCSP_CERTID_dup(orig_id.get()));
  ASSERT_TRUE(dup_id);

  // Check that the duplicated structure has the same values as the original.
  EXPECT_EQ(
      ASN1_INTEGER_cmp(orig_id.get()->serialNumber, dup_id.get()->serialNumber),
      0);
  EXPECT_EQ(ASN1_STRING_cmp(orig_id.get()->issuerNameHash,
                            dup_id.get()->issuerNameHash),
            0);
  EXPECT_EQ(ASN1_STRING_cmp(orig_id.get()->issuerKeyHash,
                            dup_id.get()->issuerKeyHash),
            0);
  EXPECT_EQ(
      X509_ALGOR_cmp(orig_id.get()->hashAlgorithm, dup_id.get()->hashAlgorithm),
      0);
  // Check that the duplicated structure is not just a replicated pointer.
  EXPECT_NE(orig_id.get(), dup_id.get());
}

TEST(OCSPTest, OCSPResponsePrint) {
  static const std::array<std::string, 19> kExpected{
      {"OCSP Response Data:", "    OCSP Response Status: successful (0x0)",
       "    Response Type: Basic OCSP Response", "    Version: 1 (0x0)",
       "    Responder Id: C = US, ST = WA, O = s2n, OU = s2n Test OCSP, CN = "
       "ocsp.s2ntest.com",
       "    Produced At: May 26 00:23:34 2021 GMT",
       "    Responses:", "    Certificate ID:", "      Hash Algorithm: sha1",
       "      Issuer Name Hash: DE7932B3217E48FB4E47AE0B9007A55376AE44CA",
       "      Issuer Key Hash: 12DF817571CA92D3CE1B2C2B773B9E3377F3F76F",
       "      Serial Number: 7778", "    Cert Status: good",
       "    This Update: May 26 00:23:34 2021 GMT",
       "    Next Update: May 24 00:23:34 2031 GMT", "",
       "    Response Extensions:", "        OCSP Nonce: ",
       "            0410AFABD4EC6A172C4A98FB1A6D22FF2928"}};

  std::string respData = GetTestData(
      std::string("crypto/ocsp/test/aws/ocsp_response.der").c_str());
  std::vector<uint8_t> ocsp_response_data(respData.begin(), respData.end());
  bssl::UniquePtr<OCSP_RESPONSE> ocspResponse =
      LoadOCSP_RESPONSE(ocsp_response_data);

  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  EXPECT_TRUE(OCSP_RESPONSE_print(bio.get(), ocspResponse.get(), 0));
  const uint8_t *out;
  size_t outlen;
  ASSERT_TRUE(BIO_mem_contents(bio.get(), &out, &outlen));

  // Iterate through |kExpected| and verify that |OCSP_RESPONSE_print| has
  // the expected format.
  std::istringstream iss((std::string((char *)out, outlen)));
  std::string line;
  for (const auto &expected : kExpected) {
    std::getline(iss, line);
    EXPECT_EQ(line, expected);
  }
}

// Verify that malformed OCSP responder IDs cannot be encoded into an
// |OCSP_RESPONSE|.
TEST(OCSPTest, OCSPResponsePrintMalformedResponderId) {
  // A default-initialized |OCSP_RESPID| (ASN1_CHOICE) has type -1, indicating
  // no alternative has been selected.
  bssl::UniquePtr<OCSP_BASICRESP> br(OCSP_BASICRESP_new());
  ASSERT_TRUE(br);
  OCSP_RESPID *rid = br->tbsResponseData->responderId;
  EXPECT_EQ(rid->type, -1);
  EXPECT_EQ(rid->value.byName, nullptr);

  // The ASN.1 encoder rejects an |OCSP_BASICRESP| with an uninitialized
  // responderId choice.
  EXPECT_FALSE(
      OCSP_response_create(OCSP_RESPONSE_STATUS_SUCCESSFUL, br.get()));

  // Set type to |V_OCSP_RESPID_KEY| without providing a key. The encoder
  // rejects this.
  br.reset(OCSP_BASICRESP_new());
  ASSERT_TRUE(br);
  rid = br->tbsResponseData->responderId;
  rid->type = V_OCSP_RESPID_KEY;
  EXPECT_EQ(rid->value.byKey, nullptr);
  EXPECT_FALSE(
      OCSP_response_create(OCSP_RESPONSE_STATUS_SUCCESSFUL, br.get()));

  // Set type to |V_OCSP_RESPID_NAME| without providing a name. The encoder
  // rejects this.
  br.reset(OCSP_BASICRESP_new());
  ASSERT_TRUE(br);
  rid = br->tbsResponseData->responderId;
  rid->type = V_OCSP_RESPID_NAME;
  EXPECT_EQ(rid->value.byName, nullptr);
  EXPECT_FALSE(
      OCSP_response_create(OCSP_RESPONSE_STATUS_SUCCESSFUL, br.get()));

  // Set an unrecognized type. The encoder rejects this.
  br.reset(OCSP_BASICRESP_new());
  ASSERT_TRUE(br);
  rid = br->tbsResponseData->responderId;
  rid->type = 42;
  EXPECT_FALSE(
      OCSP_response_create(OCSP_RESPONSE_STATUS_SUCCESSFUL, br.get()));
}

TEST(OCSPTest, OCSPRequestPrint) {
  static const std::array<std::string, 11> kExpected{
      {"OCSP Request Data:", "    Version: 1 (0x0)", "    Requestor List:",
       "        Certificate ID:", "          Hash Algorithm: sha1",
       "          Issuer Name Hash: DE7932B3217E48FB4E47AE0B9007A55376AE44CA",
       "          Issuer Key Hash: 12DF817571CA92D3CE1B2C2B773B9E3377F3F76F",
       "          Serial Number: 7778",
       "    Request Extensions:", "        OCSP Nonce: ",
       "            0410303F128CD824A2B465F4C846882B3E1F"}};

  std::string data =
      GetTestData(std::string("crypto/ocsp/test/aws/ocsp_request.der").c_str());
  std::vector<uint8_t> ocsp_request_data(data.begin(), data.end());
  bssl::UniquePtr<OCSP_REQUEST> ocspRequest =
      LoadOCSP_REQUEST(ocsp_request_data);

  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  EXPECT_TRUE(OCSP_REQUEST_print(bio.get(), ocspRequest.get(), 0));
  const uint8_t *out;
  size_t outlen;
  ASSERT_TRUE(BIO_mem_contents(bio.get(), &out, &outlen));

  // Iterate through |kExpected| and verify that |OCSP_REQUEST_print| has
  // the expected format.
  std::istringstream iss((std::string((char *)out, outlen)));
  std::string line;
  for (const auto &expected : kExpected) {
    std::getline(iss, line);
    EXPECT_EQ(line, expected);
  }
}

TEST(OCSPTest, OCSPUtilityFunctions) {
  // Create new OCSP_CERTID
  OCSP_CERTID *cert_id = OCSP_CERTID_new();
  ASSERT_TRUE(cert_id);

  bssl::UniquePtr<OCSP_REQUEST> request(OCSP_REQUEST_new());
  ASSERT_TRUE(request);

  // Test that an |OCSP_ONEREQ| does not exist yet.
  EXPECT_EQ(OCSP_request_onereq_count(request.get()), 0);
  EXPECT_FALSE(OCSP_request_onereq_get0(request.get(), 0));

  OCSP_ONEREQ *one = OCSP_request_add0_id(request.get(), cert_id);
  ASSERT_TRUE(one);
  EXPECT_EQ(OCSP_request_onereq_count(request.get()), 1);
  EXPECT_TRUE(OCSP_request_onereq_get0(request.get(), 0));

  // Call function to get OCSP_CERTID
  OCSP_CERTID *returned_id = OCSP_onereq_get0_id(one);

  // Verify the returned OCSP_CERTID is same as the one set
  ASSERT_EQ(returned_id, cert_id);
}

TEST(OCSPTest, OCSP_SINGLERESP) {
  bssl::UniquePtr<OCSP_SINGLERESP> single_resp(OCSP_SINGLERESP_new());
  ASSERT_TRUE(single_resp);

  // Initialize an |X509_EXTENSION| for testing.
  bssl::UniquePtr<ASN1_OCTET_STRING> ext_oct(ASN1_OCTET_STRING_new());
  const uint8_t data[] = {0x01, 0x02, 0x03, 0x04, 0x05};
  ASSERT_TRUE(ASN1_OCTET_STRING_set(ext_oct.get(), data, sizeof(data)));
  bssl::UniquePtr<X509_EXTENSION> ext(X509_EXTENSION_create_by_NID(
      nullptr, NID_id_pkix_OCSP_CrlID, 0, ext_oct.get()));
  ASSERT_TRUE(ext);

  // Test |X509_EXTENSION|s work with |OCSP_SINGLERESP|.
  EXPECT_TRUE(OCSP_SINGLERESP_add_ext(single_resp.get(), ext.get(), -1));
  EXPECT_EQ(OCSP_SINGLERESP_get_ext_count(single_resp.get()), 1);
  X509_EXTENSION *retrieved_ext = OCSP_SINGLERESP_get_ext(single_resp.get(), 0);
  ASSERT_EQ(ASN1_OCTET_STRING_cmp(X509_EXTENSION_get_data(retrieved_ext),
                                  X509_EXTENSION_get_data(ext.get())),
            0);
}

// Helper to create an OCSP responder cert issued by |ca_cert|/|ca_key|, with
// EKU=OCSP Signing. If |add_nocheck| is true, the id-pkix-ocsp-nocheck
// extension is added.
static bssl::UniquePtr<X509> MakeOCSPResponderCert(EVP_PKEY *responder_key,
                                                   X509 *ca_cert,
                                                   EVP_PKEY *ca_key,
                                                   bool add_nocheck) {
  bssl::UniquePtr<X509> cert(X509_new());
  if (!cert ||
      !X509_set_version(cert.get(), X509_VERSION_3) ||
      !ASN1_INTEGER_set(X509_get_serialNumber(cert.get()), 1) ||
      !X509_set_issuer_name(cert.get(), X509_get_subject_name(ca_cert)) ||
      !X509_NAME_add_entry_by_txt(
          X509_get_subject_name(cert.get()), "CN", MBSTRING_UTF8,
          reinterpret_cast<const uint8_t *>("OCSP Responder"), -1, -1, 0) ||
      !X509_set_pubkey(cert.get(), responder_key) ||
      !ASN1_TIME_adj(X509_getm_notBefore(cert.get()), kReferenceTime, -1, 0) ||
      !ASN1_TIME_adj(X509_getm_notAfter(cert.get()), kReferenceTime, 1, 0)) {
    return nullptr;
  }

  // Add EKU: OCSP Signing (required for delegated responder).
  bssl::UniquePtr<STACK_OF(ASN1_OBJECT)> eku(sk_ASN1_OBJECT_new_null());
  if (!eku || !sk_ASN1_OBJECT_push(eku.get(), OBJ_nid2obj(NID_OCSP_sign)) ||
      !X509_add1_ext_i2d(cert.get(), NID_ext_key_usage, eku.get(),
                         /*crit=*/1, /*flags=*/0)) {
    return nullptr;
  }

  if (add_nocheck) {
    ASN1_NULL *null_val = ASN1_NULL_new();
    if (null_val == NULL ||
        !X509_add1_ext_i2d(cert.get(), NID_id_pkix_OCSP_noCheck, null_val,
                           /*crit=*/0, /*flags=*/0)) {
      ASN1_NULL_free(null_val);
      return nullptr;
    }
    ASN1_NULL_free(null_val);
  }

  if (!X509_sign(cert.get(), ca_key, EVP_sha256())) {
    return nullptr;
  }
  return cert;
}

// Test that the id-pkix-ocsp-nocheck extension on a delegated OCSP responder
// certificate causes CRL checking to be skipped during verification. Without
// the extension, verification should fail when CRL checking is enabled because
// no CRL is available.
TEST(OCSPTest, OCSPNoCheckExtension) {
  // Generate CA key and self-signed CA cert.
  bssl::UniquePtr<EVP_PKEY> ca_key(EVP_PKEY_new());
  ASSERT_TRUE(ca_key);
  bssl::UniquePtr<RSA> ca_rsa(RSA_new());
  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));
  ASSERT_TRUE(RSA_generate_key_ex(ca_rsa.get(), 2048, e.get(), nullptr));
  ASSERT_TRUE(EVP_PKEY_set1_RSA(ca_key.get(), ca_rsa.get()));

  bssl::UniquePtr<X509> ca_cert =
      MakeTestCert("Test CA", "Test CA", ca_key.get(), /*is_ca=*/true);
  ASSERT_TRUE(ca_cert);
  ASSERT_TRUE(X509_sign(ca_cert.get(), ca_key.get(), EVP_sha256()));

  // Generate responder key.
  bssl::UniquePtr<EVP_PKEY> resp_key(EVP_PKEY_new());
  ASSERT_TRUE(resp_key);
  bssl::UniquePtr<RSA> resp_rsa(RSA_new());
  ASSERT_TRUE(RSA_generate_key_ex(resp_rsa.get(), 2048, e.get(), nullptr));
  ASSERT_TRUE(EVP_PKEY_set1_RSA(resp_key.get(), resp_rsa.get()));

  // Create trust store with CRL checking enabled and verification time set
  // to kReferenceTime (certs from MakeTestCert are only valid around that time).
  bssl::UniquePtr<X509_STORE> trust_store(X509_STORE_new());
  ASSERT_TRUE(trust_store);
  ASSERT_TRUE(X509_STORE_add_cert(trust_store.get(), ca_cert.get()));
  X509_STORE_set_flags(trust_store.get(), X509_V_FLAG_CRL_CHECK);
  X509_VERIFY_PARAM *store_param = X509_STORE_get0_param(trust_store.get());
  ASSERT_TRUE(store_param);
  X509_VERIFY_PARAM_set_time_posix(store_param, kReferenceTime);

  // Create an OCSP_CERTID for use in the OCSP response.
  bssl::UniquePtr<OCSP_CERTID> cert_id(
      OCSP_cert_to_id(EVP_sha1(), ca_cert.get(), ca_cert.get()));
  ASSERT_TRUE(cert_id);

  // Test with nocheck extension: verification should succeed despite CRL
  // checking being enabled, because the extension causes the flag to be
  // cleared for the responder cert.
  {
    bssl::UniquePtr<X509> responder_cert = MakeOCSPResponderCert(
        resp_key.get(), ca_cert.get(), ca_key.get(), /*add_nocheck=*/true);
    ASSERT_TRUE(responder_cert);

    bssl::UniquePtr<OCSP_BASICRESP> bs(OCSP_BASICRESP_new());
    ASSERT_TRUE(bs);

    bssl::UniquePtr<ASN1_TIME> this_update(ASN1_TIME_adj(nullptr, kReferenceTime, -1, 0));
    bssl::UniquePtr<ASN1_TIME> next_update(ASN1_TIME_adj(nullptr, kReferenceTime, 1, 0));
    ASSERT_TRUE(OCSP_basic_add1_status(bs.get(), cert_id.get(),
                                       V_OCSP_CERTSTATUS_GOOD, 0, nullptr,
                                       this_update.get(), next_update.get()));

    ASSERT_TRUE(OCSP_basic_sign(bs.get(), responder_cert.get(), resp_key.get(),
                                EVP_sha256(),
                                CertsToStack({ca_cert.get()}).get(), 0));

    int ret = OCSP_basic_verify(
        bs.get(), CertsToStack({responder_cert.get()}).get(), trust_store.get(),
        0);
    EXPECT_EQ(ret, OCSP_VERIFYSTATUS_SUCCESS)
        << "Verification should succeed with nocheck extension";
  }

  ERR_clear_error();

  // Test without nocheck extension: verification should fail because CRL
  // checking is enabled but no CRL is available.
  {
    bssl::UniquePtr<X509> responder_cert = MakeOCSPResponderCert(
        resp_key.get(), ca_cert.get(), ca_key.get(), /*add_nocheck=*/false);
    ASSERT_TRUE(responder_cert);

    bssl::UniquePtr<OCSP_BASICRESP> bs(OCSP_BASICRESP_new());
    ASSERT_TRUE(bs);

    bssl::UniquePtr<ASN1_TIME> this_update(ASN1_TIME_adj(nullptr, kReferenceTime, -1, 0));
    bssl::UniquePtr<ASN1_TIME> next_update(ASN1_TIME_adj(nullptr, kReferenceTime, 1, 0));
    ASSERT_TRUE(OCSP_basic_add1_status(bs.get(), cert_id.get(),
                                       V_OCSP_CERTSTATUS_GOOD, 0, nullptr,
                                       this_update.get(), next_update.get()));

    ASSERT_TRUE(OCSP_basic_sign(bs.get(), responder_cert.get(), resp_key.get(),
                                EVP_sha256(),
                                CertsToStack({ca_cert.get()}).get(), 0));

    int ret = OCSP_basic_verify(
        bs.get(), CertsToStack({responder_cert.get()}).get(), trust_store.get(),
        0);
    EXPECT_NE(ret, OCSP_VERIFYSTATUS_SUCCESS)
        << "Verification should fail without nocheck extension when CRL "
           "checking is enabled";
  }
}
