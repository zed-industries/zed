// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include "test_config.h"

#include <assert.h>
#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <algorithm>
#include <functional>
#include <limits>
#include <memory>
#include <type_traits>

#include <openssl/base64.h>
#include <openssl/hmac.h>
#include <openssl/hpke.h>
#include <openssl/rand.h>
#include <openssl/span.h>
#include <openssl/ssl.h>

#include "../../crypto/internal.h"
#include "../internal.h"
#include "handshake_util.h"
#include "mock_quic_transport.h"
#include "test_state.h"

namespace {

struct Flag {
  const char *name;
  bool has_param;
  // skip_handshaker, if true, causes this flag to be skipped when
  // forwarding flags to the handshaker. This should be used with flags
  // that only impact connecting to the runner.
  bool skip_handshaker;
  // If |has_param| is false, |param| will be nullptr.
  std::function<bool(TestConfig *config, const char *param)> set_param;
};

Flag BoolFlag(const char *name, bool TestConfig::*field,
              bool skip_handshaker = false) {
  return Flag{name, false, skip_handshaker,
              [=](TestConfig *config, const char *) -> bool {
                config->*field = true;
                return true;
              }};
}

template <typename T>
bool StringToInt(T *out, const char *str) {
  static_assert(std::is_integral<T>::value, "not an integral type");

  // |strtoull| allows leading '-' with wraparound. Additionally, both
  // functions accept empty strings and leading whitespace.
  if (!OPENSSL_isdigit(static_cast<unsigned char>(*str)) &&
      (!std::is_signed<T>::value || *str != '-')) {
    return false;
  }

  errno = 0;
  char *end;
  if (std::is_signed<T>::value) {
    static_assert(sizeof(T) <= sizeof(long long),
                  "type too large for long long");
    long long value = strtoll(str, &end, 10);
    if (value < static_cast<long long>(std::numeric_limits<T>::min()) ||
        value > static_cast<long long>(std::numeric_limits<T>::max())) {
      return false;
    }
    *out = static_cast<T>(value);
  } else {
    static_assert(sizeof(T) <= sizeof(unsigned long long),
                  "type too large for unsigned long long");
    unsigned long long value = strtoull(str, &end, 10);
    if (value >
        static_cast<unsigned long long>(std::numeric_limits<T>::max())) {
      return false;
    }
    *out = static_cast<T>(value);
  }

  // Check for overflow and that the whole input was consumed.
  return errno != ERANGE && *end == '\0';
}

template <typename T>
Flag IntFlag(const char *name, T TestConfig::*field,
             bool skip_handshaker = false) {
  return Flag{name, true, skip_handshaker,
              [=](TestConfig *config, const char *param) -> bool {
                return StringToInt(&(config->*field), param);
              }};
}

template <typename T>
Flag IntVectorFlag(const char *name, std::vector<T> TestConfig::*field,
                   bool skip_handshaker = false) {
  return Flag{name, true, skip_handshaker,
              [=](TestConfig *config, const char *param) -> bool {
                T value;
                if (!StringToInt(&value, param)) {
                  return false;
                }
                (config->*field).push_back(value);
                return true;
              }};
}

Flag StringFlag(const char *name, std::string TestConfig::*field,
                bool skip_handshaker = false) {
  return Flag{name, true, skip_handshaker,
              [=](TestConfig *config, const char *param) -> bool {
                config->*field = param;
                return true;
              }};
}

// TODO(davidben): When we can depend on C++17 or Abseil, switch this to
// std::optional or absl::optional.
Flag OptionalStringFlag(const char *name,
                        std::unique_ptr<std::string> TestConfig::*field,
                        bool skip_handshaker = false) {
  return Flag{name, true, skip_handshaker,
              [=](TestConfig *config, const char *param) -> bool {
                (config->*field).reset(new std::string(param));
                return true;
              }};
}

bool DecodeBase64(std::string *out, const std::string &in) {
  size_t len;
  if (!EVP_DecodedLength(&len, in.size())) {
    fprintf(stderr, "Invalid base64: %s.\n", in.c_str());
    return false;
  }
  std::vector<uint8_t> buf(len);
  if (!EVP_DecodeBase64(buf.data(), &len, buf.size(),
                        reinterpret_cast<const uint8_t *>(in.data()),
                        in.size())) {
    fprintf(stderr, "Invalid base64: %s.\n", in.c_str());
    return false;
  }
  out->assign(reinterpret_cast<const char *>(buf.data()), len);
  return true;
}

Flag Base64Flag(const char *name, std::string TestConfig::*field,
                bool skip_handshaker = false) {
  return Flag{name, true, skip_handshaker,
              [=](TestConfig *config, const char *param) -> bool {
                return DecodeBase64(&(config->*field), param);
              }};
}

Flag Base64VectorFlag(const char *name,
                      std::vector<std::string> TestConfig::*field,
                      bool skip_handshaker = false) {
  return Flag{name, true, skip_handshaker,
              [=](TestConfig *config, const char *param) -> bool {
                std::string value;
                if (!DecodeBase64(&value, param)) {
                  return false;
                }
                (config->*field).push_back(std::move(value));
                return true;
              }};
}

Flag StringPairVectorFlag(
    const char *name,
    std::vector<std::pair<std::string, std::string>> TestConfig::*field,
    bool skip_handshaker = false) {
  return Flag{name, true, skip_handshaker,
              [=](TestConfig *config, const char *param) -> bool {
                const char *comma = strchr(param, ',');
                if (!comma) {
                  return false;
                }
                (config->*field)
                    .push_back(std::make_pair(std::string(param, comma - param),
                                              std::string(comma + 1)));
                return true;
              }};
}

std::vector<Flag> SortedFlags() {
  std::vector<Flag> flags = {
      IntFlag("-port", &TestConfig::port, /*skip_handshaker=*/true),
      BoolFlag("-ipv6", &TestConfig::ipv6, /*skip_handshaker=*/true),
      IntFlag("-shim-id", &TestConfig::shim_id, /*skip_handshaker=*/true),
      BoolFlag("-server", &TestConfig::is_server),
      BoolFlag("-dtls", &TestConfig::is_dtls),
      BoolFlag("-quic", &TestConfig::is_quic),
      IntFlag("-resume-count", &TestConfig::resume_count),
      StringFlag("-write-settings", &TestConfig::write_settings),
      BoolFlag("-fallback-scsv", &TestConfig::fallback_scsv),
      IntVectorFlag("-signing-prefs", &TestConfig::signing_prefs),
      IntVectorFlag("-verify-prefs", &TestConfig::verify_prefs),
      IntVectorFlag("-expect-peer-verify-pref",
                    &TestConfig::expect_peer_verify_prefs),
      IntVectorFlag("-curves", &TestConfig::curves),
      StringFlag("-key-file", &TestConfig::key_file),
      StringFlag("-cert-file", &TestConfig::cert_file),
      StringFlag("-trust-cert", &TestConfig::trust_cert),
      StringFlag("-expect-server-name", &TestConfig::expect_server_name),
      BoolFlag("-enable-ech-grease", &TestConfig::enable_ech_grease),
      Base64VectorFlag("-ech-server-config", &TestConfig::ech_server_configs),
      Base64VectorFlag("-ech-server-key", &TestConfig::ech_server_keys),
      IntVectorFlag("-ech-is-retry-config", &TestConfig::ech_is_retry_config),
      BoolFlag("-expect-ech-accept", &TestConfig::expect_ech_accept),
      StringFlag("-expect-ech-name-override",
                 &TestConfig::expect_ech_name_override),
      BoolFlag("-expect-no-ech-name-override",
               &TestConfig::expect_no_ech_name_override),
      Base64Flag("-expect-ech-retry-configs",
                 &TestConfig::expect_ech_retry_configs),
      BoolFlag("-expect-no-ech-retry-configs",
               &TestConfig::expect_no_ech_retry_configs),
      Base64Flag("-ech-config-list", &TestConfig::ech_config_list),
      Base64Flag("-expect-certificate-types",
                 &TestConfig::expect_certificate_types),
      BoolFlag("-require-any-client-certificate",
               &TestConfig::require_any_client_certificate),
      StringFlag("-advertise-npn", &TestConfig::advertise_npn),
      StringFlag("-expect-next-proto", &TestConfig::expect_next_proto),
      BoolFlag("-false-start", &TestConfig::false_start),
      StringFlag("-select-next-proto", &TestConfig::select_next_proto),
      BoolFlag("-async", &TestConfig::async),
      BoolFlag("-write-different-record-sizes",
               &TestConfig::write_different_record_sizes),
      BoolFlag("-cbc-record-splitting", &TestConfig::cbc_record_splitting),
      BoolFlag("-partial-write", &TestConfig::partial_write),
      BoolFlag("-no-tls13", &TestConfig::no_tls13),
      BoolFlag("-no-tls12", &TestConfig::no_tls12),
      BoolFlag("-no-tls11", &TestConfig::no_tls11),
      BoolFlag("-no-tls1", &TestConfig::no_tls1),
      BoolFlag("-no-ticket", &TestConfig::no_ticket),
      Base64Flag("-expect-channel-id", &TestConfig::expect_channel_id),
      BoolFlag("-enable-channel-id", &TestConfig::enable_channel_id),
      StringFlag("-send-channel-id", &TestConfig::send_channel_id),
      BoolFlag("-shim-writes-first", &TestConfig::shim_writes_first),
      StringFlag("-host-name", &TestConfig::host_name),
      StringFlag("-advertise-alpn", &TestConfig::advertise_alpn),
      StringFlag("-expect-alpn", &TestConfig::expect_alpn),
      StringFlag("-expect-advertised-alpn",
                 &TestConfig::expect_advertised_alpn),
      StringFlag("-select-alpn", &TestConfig::select_alpn),
      BoolFlag("-decline-alpn", &TestConfig::decline_alpn),
      BoolFlag("-reject-alpn", &TestConfig::reject_alpn),
      BoolFlag("-select-empty-alpn", &TestConfig::select_empty_alpn),
      BoolFlag("-defer-alps", &TestConfig::defer_alps),
      StringPairVectorFlag("-application-settings",
                           &TestConfig::application_settings),
      OptionalStringFlag("-expect-peer-application-settings",
                         &TestConfig::expect_peer_application_settings),
      BoolFlag("-alps-use-new-codepoint", &TestConfig::alps_use_new_codepoint),
      Base64Flag("-quic-transport-params", &TestConfig::quic_transport_params),
      Base64Flag("-expect-quic-transport-params",
                 &TestConfig::expect_quic_transport_params),
      IntFlag("-quic-use-legacy-codepoint",
              &TestConfig::quic_use_legacy_codepoint),
      BoolFlag("-expect-session-miss", &TestConfig::expect_session_miss),
      BoolFlag("-expect-extended-master-secret",
               &TestConfig::expect_extended_master_secret),
      StringFlag("-psk", &TestConfig::psk),
      StringFlag("-psk-identity", &TestConfig::psk_identity),
      StringFlag("-srtp-profiles", &TestConfig::srtp_profiles),
      BoolFlag("-enable-ocsp-stapling", &TestConfig::enable_ocsp_stapling),
      BoolFlag("-enable-signed-cert-timestamps",
               &TestConfig::enable_signed_cert_timestamps),
      Base64Flag("-expect-signed-cert-timestamps",
                 &TestConfig::expect_signed_cert_timestamps),
      IntFlag("-min-version", &TestConfig::min_version),
      IntFlag("-max-version", &TestConfig::max_version),
      IntFlag("-expect-version", &TestConfig::expect_version),
      IntFlag("-mtu", &TestConfig::mtu),
      BoolFlag("-implicit-handshake", &TestConfig::implicit_handshake),
      BoolFlag("-use-early-callback", &TestConfig::use_early_callback),
      BoolFlag("-fail-early-callback", &TestConfig::fail_early_callback),
      BoolFlag("-install-ddos-callback", &TestConfig::install_ddos_callback),
      BoolFlag("-fail-ddos-callback", &TestConfig::fail_ddos_callback),
      BoolFlag("-fail-cert-callback", &TestConfig::fail_cert_callback),
      StringFlag("-cipher", &TestConfig::cipher),
      BoolFlag("-handshake-never-done", &TestConfig::handshake_never_done),
      IntFlag("-export-keying-material", &TestConfig::export_keying_material),
      StringFlag("-export-label", &TestConfig::export_label),
      StringFlag("-export-context", &TestConfig::export_context),
      BoolFlag("-use-export-context", &TestConfig::use_export_context),
      BoolFlag("-tls-unique", &TestConfig::tls_unique),
      BoolFlag("-expect-ticket-renewal", &TestConfig::expect_ticket_renewal),
      BoolFlag("-expect-no-session", &TestConfig::expect_no_session),
      BoolFlag("-expect-ticket-supports-early-data",
               &TestConfig::expect_ticket_supports_early_data),
      BoolFlag("-expect-accept-early-data",
               &TestConfig::expect_accept_early_data),
      BoolFlag("-expect-reject-early-data",
               &TestConfig::expect_reject_early_data),
      BoolFlag("-expect-no-offer-early-data",
               &TestConfig::expect_no_offer_early_data),
      BoolFlag("-use-ticket-callback", &TestConfig::use_ticket_callback),
      BoolFlag("-renew-ticket", &TestConfig::renew_ticket),
      BoolFlag("-enable-early-data", &TestConfig::enable_early_data),
      BoolFlag("-enable-client-custom-extension", &TestConfig::enable_client_custom_extension),
      BoolFlag("-enable-server-custom-extension", &TestConfig::enable_server_custom_extension),
      BoolFlag("-custom-extension-skip", &TestConfig::custom_extension_skip),
      BoolFlag("-custom-extension-fail-add", &TestConfig::custom_extension_fail_add),
      Base64Flag("-ocsp-response", &TestConfig::ocsp_response),
      Base64Flag("-expect-ocsp-response", &TestConfig::expect_ocsp_response),
      BoolFlag("-check-close-notify", &TestConfig::check_close_notify),
      BoolFlag("-shim-shuts-down", &TestConfig::shim_shuts_down),
      BoolFlag("-verify-fail", &TestConfig::verify_fail),
      BoolFlag("-verify-peer", &TestConfig::verify_peer),
      BoolFlag("-verify-peer-if-no-obc", &TestConfig::verify_peer_if_no_obc),
      BoolFlag("-expect-verify-result", &TestConfig::expect_verify_result),
      Base64Flag("-signed-cert-timestamps",
                 &TestConfig::signed_cert_timestamps),
      IntFlag("-expect-total-renegotiations",
              &TestConfig::expect_total_renegotiations),
      BoolFlag("-renegotiate-once", &TestConfig::renegotiate_once),
      BoolFlag("-renegotiate-freely", &TestConfig::renegotiate_freely),
      BoolFlag("-renegotiate-ignore", &TestConfig::renegotiate_ignore),
      BoolFlag("-renegotiate-explicit", &TestConfig::renegotiate_explicit),
      BoolFlag("-forbid-renegotiation-after-handshake",
               &TestConfig::forbid_renegotiation_after_handshake),
      IntFlag("-expect-peer-signature-algorithm",
              &TestConfig::expect_peer_signature_algorithm),
      IntFlag("-expect-curve-id", &TestConfig::expect_curve_id),
      BoolFlag("-use-old-client-cert-callback",
               &TestConfig::use_old_client_cert_callback),
      IntFlag("-initial-timeout-duration-ms",
              &TestConfig::initial_timeout_duration_ms),
      StringFlag("-use-client-ca-list", &TestConfig::use_client_ca_list),
      StringFlag("-expect-client-ca-list", &TestConfig::expect_client_ca_list),
      BoolFlag("-send-alert", &TestConfig::send_alert),
      BoolFlag("-peek-then-read", &TestConfig::peek_then_read),
      BoolFlag("-enable-grease", &TestConfig::enable_grease),
      BoolFlag("-permute-extensions", &TestConfig::permute_extensions),
      IntFlag("-max-cert-list", &TestConfig::max_cert_list),
      Base64Flag("-ticket-key", &TestConfig::ticket_key),
      BoolFlag("-use-exporter-between-reads",
               &TestConfig::use_exporter_between_reads),
      IntFlag("-expect-cipher-aes", &TestConfig::expect_cipher_aes),
      IntFlag("-expect-cipher-no-aes", &TestConfig::expect_cipher_no_aes),
      IntFlag("-expect-cipher", &TestConfig::expect_cipher),
      StringFlag("-expect-peer-cert-file", &TestConfig::expect_peer_cert_file),
      IntFlag("-resumption-delay", &TestConfig::resumption_delay),
      BoolFlag("-retain-only-sha256-client-cert",
               &TestConfig::retain_only_sha256_client_cert),
      BoolFlag("-expect-sha256-client-cert",
               &TestConfig::expect_sha256_client_cert),
      BoolFlag("-read-with-unfinished-write",
               &TestConfig::read_with_unfinished_write),
      BoolFlag("-expect-secure-renegotiation",
               &TestConfig::expect_secure_renegotiation),
      BoolFlag("-expect-no-secure-renegotiation",
               &TestConfig::expect_no_secure_renegotiation),
      IntFlag("-max-send-fragment", &TestConfig::max_send_fragment),
      IntFlag("-read-size", &TestConfig::read_size),
      BoolFlag("-expect-session-id", &TestConfig::expect_session_id),
      BoolFlag("-expect-no-session-id", &TestConfig::expect_no_session_id),
      IntFlag("-expect-ticket-age-skew", &TestConfig::expect_ticket_age_skew),
      BoolFlag("-no-op-extra-handshake", &TestConfig::no_op_extra_handshake),
      BoolFlag("-handshake-twice", &TestConfig::handshake_twice),
      BoolFlag("-allow-unknown-alpn-protos",
               &TestConfig::allow_unknown_alpn_protos),
      BoolFlag("-use-custom-verify-callback",
               &TestConfig::use_custom_verify_callback),
      StringFlag("-expect-msg-callback", &TestConfig::expect_msg_callback),
      BoolFlag("-allow-false-start-without-alpn",
               &TestConfig::allow_false_start_without_alpn),
      BoolFlag("-handoff", &TestConfig::handoff),
      BoolFlag("-handshake-hints", &TestConfig::handshake_hints),
      BoolFlag("-allow-hint-mismatch", &TestConfig::allow_hint_mismatch),
      BoolFlag("-use-ocsp-callback", &TestConfig::use_ocsp_callback),
      BoolFlag("-set-ocsp-in-callback", &TestConfig::set_ocsp_in_callback),
      BoolFlag("-decline-ocsp-callback", &TestConfig::decline_ocsp_callback),
      BoolFlag("-fail-ocsp-callback", &TestConfig::fail_ocsp_callback),
      BoolFlag("-install-cert-compression-algs",
               &TestConfig::install_cert_compression_algs),
      IntFlag("-install-one-cert-compression-alg",
              &TestConfig::install_one_cert_compression_alg),
      BoolFlag("-reverify-on-resume", &TestConfig::reverify_on_resume),
      BoolFlag("-enforce-rsa-key-usage", &TestConfig::enforce_rsa_key_usage),
      BoolFlag("-expect-key-usage-invalid",
               &TestConfig::expect_key_usage_invalid),
      BoolFlag("-is-handshaker-supported",
               &TestConfig::is_handshaker_supported),
      BoolFlag("-handshaker-resume", &TestConfig::handshaker_resume),
      StringFlag("-handshaker-path", &TestConfig::handshaker_path),
      BoolFlag("-jdk11-workaround", &TestConfig::jdk11_workaround),
      BoolFlag("-server-preference", &TestConfig::server_preference),
      BoolFlag("-export-traffic-secrets", &TestConfig::export_traffic_secrets),
      BoolFlag("-key-update", &TestConfig::key_update),
      BoolFlag("-expect-delegated-credential-used",
               &TestConfig::expect_delegated_credential_used),
      StringFlag("-delegated-credential", &TestConfig::delegated_credential),
      StringFlag("-expect-early-data-reason",
                 &TestConfig::expect_early_data_reason),
      BoolFlag("-expect-hrr", &TestConfig::expect_hrr),
      BoolFlag("-expect-no-hrr", &TestConfig::expect_no_hrr),
      BoolFlag("-wait-for-debugger", &TestConfig::wait_for_debugger),
      StringFlag("-quic-early-data-context",
                 &TestConfig::quic_early_data_context),
      IntFlag("-early-write-after-message",
              &TestConfig::early_write_after_message),
      BoolFlag("-check-ssl-transfer", &TestConfig::check_ssl_transfer),
      BoolFlag("-do-ssl-transfer", &TestConfig::do_ssl_transfer),
      IntFlag("-read-ahead-buffer-size",
          &TestConfig::read_ahead_buffer_size),
      StringFlag("-ssl-fuzz-seed-path-prefix", &TestConfig::ssl_fuzz_seed_path_prefix),
      StringFlag("-tls13-ciphersuites", &TestConfig::tls13_ciphersuites),
      StringPairVectorFlag("-multiple-certs-slot", &TestConfig::multiple_certs_slot),
      BoolFlag("-no-check-client-certificate-type",
               &TestConfig::no_check_client_certificate_type),
  };
  std::sort(flags.begin(), flags.end(), [](const Flag &a, const Flag &b) {
    return strcmp(a.name, b.name) < 0;
  });
  return flags;
}

const Flag *FindFlag(const char *name) {
  static const std::vector<Flag> kSortedFlags = SortedFlags();
  auto iter = std::lower_bound(kSortedFlags.begin(), kSortedFlags.end(), name,
                               [](const Flag &flag, const char *key) {
                                 return strcmp(flag.name, key) < 0;
                               });
  if (iter == kSortedFlags.end() || strcmp(iter->name, name) != 0) {
    return nullptr;
  }
  return &*iter;
}

// RemovePrefix checks if |*str| begins with |prefix| + "-". If so, it advances
// |*str| past |prefix| (but not past the "-") and returns true. Otherwise, it
// returns false and leaves |*str| unmodified.
bool RemovePrefix(const char **str, const char *prefix) {
  size_t prefix_len = strlen(prefix);
  if (strncmp(*str, prefix, strlen(prefix)) == 0 && (*str)[prefix_len] == '-') {
    *str += strlen(prefix);
    return true;
  }
  return false;
}

}  // namespace

bool ParseConfig(int argc, char **argv, bool is_shim,
                 TestConfig *out_initial,
                 TestConfig *out_resume,
                 TestConfig *out_retry) {
  for (int i = 0; i < argc; i++) {
    bool skip = false;
    const char *arg = argv[i];
    const char *name = arg;

    // -on-shim and -on-handshaker prefixes enable flags only on the shim or
    // handshaker.
    if (RemovePrefix(&name, "-on-shim")) {
      if (!is_shim) {
        skip = true;
      }
    } else if (RemovePrefix(&name, "-on-handshaker")) {
      if (is_shim) {
        skip = true;
      }
    }

    // The following prefixes allow different configurations for each of the
    // initial, resumption, and 0-RTT retry handshakes.
    TestConfig *out = nullptr;
    if (RemovePrefix(&name, "-on-initial")) {
      out = out_initial;
    } else if (RemovePrefix(&name, "-on-resume")) {
      out = out_resume;
    } else if (RemovePrefix(&name, "-on-retry")) {
      out = out_retry;
    }

    const Flag *flag = FindFlag(name);
    if (flag == nullptr) {
      fprintf(stderr, "Unrecognized flag: %s\n", name);
      return false;
    }

    const char *param = nullptr;
    if (flag->has_param) {
      if (i >= argc) {
        fprintf(stderr, "Missing parameter for %s\n", name);
        return false;
      }
      i++;
      param = argv[i];
    }

    if (!flag->skip_handshaker) {
      out_initial->handshaker_args.push_back(arg);
      if (flag->has_param) {
        out_initial->handshaker_args.push_back(param);
      }
    }

    if (!skip) {
      if (out != nullptr) {
        if (!flag->set_param(out, param)) {
          fprintf(stderr, "Invalid parameter for %s: %s\n", name, param);
          return false;
        }
      } else {
        // Unprefixed flags apply to all three.
        if (!flag->set_param(out_initial, param) ||
            !flag->set_param(out_resume, param) ||
            !flag->set_param(out_retry, param)) {
          fprintf(stderr, "Invalid parameter for %s: %s\n", name, param);
          return false;
        }
      }
    }
  }

  out_resume->handshaker_args = std::vector<const char*>(out_initial->handshaker_args);
  out_retry->handshaker_args = std::vector<const char*>(out_initial->handshaker_args);
  return true;
}

static CRYPTO_once_t once = CRYPTO_ONCE_INIT;
static int g_config_index = 0;
static CRYPTO_BUFFER_POOL *g_pool = nullptr;

static void init_once() {
  g_config_index = SSL_get_ex_new_index(0, NULL, NULL, NULL, NULL);
  if (g_config_index < 0) {
    abort();
  }
  g_pool = CRYPTO_BUFFER_POOL_new();
  if (!g_pool) {
    abort();
  }
}

bool SetTestConfig(SSL *ssl, const TestConfig *config) {
  CRYPTO_once(&once, init_once);
  return SSL_set_ex_data(ssl, g_config_index, (void *)config) == 1;
}

const TestConfig *GetTestConfig(const SSL *ssl) {
  CRYPTO_once(&once, init_once);
  return (const TestConfig *)SSL_get_ex_data(ssl, g_config_index);
}

static int LegacyOCSPCallback(SSL *ssl, void *arg) {
  const TestConfig *config = GetTestConfig(ssl);
  if (!SSL_is_server(ssl)) {
    return !config->fail_ocsp_callback;
  }

  if (!config->ocsp_response.empty() && config->set_ocsp_in_callback &&
      !SSL_set_ocsp_response(ssl, (const uint8_t *)config->ocsp_response.data(),
                             config->ocsp_response.size())) {
    return SSL_TLSEXT_ERR_ALERT_FATAL;
  }
  if (config->fail_ocsp_callback) {
    return SSL_TLSEXT_ERR_ALERT_FATAL;
  }
  if (config->decline_ocsp_callback) {
    return SSL_TLSEXT_ERR_NOACK;
  }
  return SSL_TLSEXT_ERR_OK;
}

// kCustomExtensionValue is the extension value that the custom extension
// callbacks will add.
static const uint16_t kCustomExtensionValue = 1234;
static void *const kCustomExtensionAddArg =
    reinterpret_cast<void *>(kCustomExtensionValue);
static void *const kCustomExtensionParseArg =
    reinterpret_cast<void *>(kCustomExtensionValue + 1);
static const char kCustomExtensionContents[] = "custom extension";

static int CustomExtensionAddCallback(SSL *ssl, unsigned extension_value,
                                      const uint8_t **out, size_t *out_len,
                                      int *out_alert_value, void *add_arg) {
  if (extension_value != kCustomExtensionValue ||
      add_arg != kCustomExtensionAddArg) {
    abort();
  }

  if (GetTestConfig(ssl)->custom_extension_skip) {
    return 0;
  }
  if (GetTestConfig(ssl)->custom_extension_fail_add) {
    return -1;
  }

  *out = reinterpret_cast<const uint8_t *>(kCustomExtensionContents);
  *out_len = sizeof(kCustomExtensionContents) - 1;

  return 1;
}

static void CustomExtensionFreeCallback(SSL *ssl, unsigned extension_value,
                                        const uint8_t *out, void *add_arg) {
  if (extension_value != kCustomExtensionValue ||
      add_arg != kCustomExtensionAddArg ||
      out != reinterpret_cast<const uint8_t *>(kCustomExtensionContents)) {
    abort();
  }
}

static int CustomExtensionParseCallback(SSL *ssl, unsigned extension_value,
                                        const uint8_t *contents,
                                        size_t contents_len,
                                        int *out_alert_value, void *parse_arg) {
  if (extension_value != kCustomExtensionValue ||
      parse_arg != kCustomExtensionParseArg) {
    abort();
  }

  if (contents_len != sizeof(kCustomExtensionContents) - 1 ||
      OPENSSL_memcmp(contents, kCustomExtensionContents, contents_len) != 0) {
    *out_alert_value = SSL_AD_DECODE_ERROR;
    return 0;
  }

  return 1;
}

static int ServerNameCallback(SSL *ssl, int *out_alert, void *arg) {
  // SNI must be accessible from the SNI callback.
  const TestConfig *config = GetTestConfig(ssl);
  const char *server_name = SSL_get_servername(ssl, TLSEXT_NAMETYPE_host_name);
  if (server_name == nullptr ||
      std::string(server_name) != config->expect_server_name) {
    fprintf(stderr, "servername mismatch (got %s; want %s).\n", server_name,
            config->expect_server_name.c_str());
    return SSL_TLSEXT_ERR_ALERT_FATAL;
  }

  return SSL_TLSEXT_ERR_OK;
}

static int NextProtoSelectCallback(SSL *ssl, uint8_t **out, uint8_t *outlen,
                                   const uint8_t *in, unsigned inlen,
                                   void *arg) {
  const TestConfig *config = GetTestConfig(ssl);
  if (config->select_next_proto.empty()) {
    return SSL_TLSEXT_ERR_NOACK;
  }

  *out = (uint8_t *)config->select_next_proto.data();
  *outlen = config->select_next_proto.size();
  return SSL_TLSEXT_ERR_OK;
}

static int NextProtosAdvertisedCallback(SSL *ssl, const uint8_t **out,
                                        unsigned int *out_len, void *arg) {
  const TestConfig *config = GetTestConfig(ssl);
  if (config->advertise_npn.empty()) {
    return SSL_TLSEXT_ERR_NOACK;
  }

  if (config->advertise_npn.size() > UINT_MAX) {
    fprintf(stderr, "NPN value too large.\n");
    return SSL_TLSEXT_ERR_ALERT_FATAL;
  }

  *out = reinterpret_cast<const uint8_t *>(config->advertise_npn.data());
  *out_len = static_cast<unsigned>(config->advertise_npn.size());
  return SSL_TLSEXT_ERR_OK;
}

static void MessageCallback(int is_write, int version, int content_type,
                            const void *buf, size_t len, SSL *ssl, void *arg) {
  const uint8_t *buf_u8 = reinterpret_cast<const uint8_t *>(buf);
  const TestConfig *config = GetTestConfig(ssl);
  TestState *state = GetTestState(ssl);
  if (!state->msg_callback_ok) {
    return;
  }

  if (content_type == SSL3_RT_HEADER) {
    if (len !=
        (config->is_dtls ? DTLS1_RT_HEADER_LENGTH : SSL3_RT_HEADER_LENGTH)) {
      fprintf(stderr, "Incorrect length for record header: %zu.\n", len);
      state->msg_callback_ok = false;
    }
    return;
  }

  state->msg_callback_text += is_write ? "write " : "read ";
  switch (content_type) {
    case 0:
      if (version != SSL2_VERSION) {
        fprintf(stderr, "Incorrect version for V2ClientHello: %x.\n",
                static_cast<unsigned>(version));
        state->msg_callback_ok = false;
        return;
      }
      state->msg_callback_text += "v2clienthello\n";
      return;

    case SSL3_RT_CLIENT_HELLO_INNER:
    case SSL3_RT_HANDSHAKE: {
      CBS cbs;
      CBS_init(&cbs, buf_u8, len);
      uint8_t type;
      uint32_t msg_len;
      if (!CBS_get_u8(&cbs, &type) ||
          // TODO(davidben): Reporting on entire messages would be more
          // consistent than fragments.
          (config->is_dtls &&
           !CBS_skip(&cbs, 3 /* total */ + 2 /* seq */ + 3 /* frag_off */)) ||
          !CBS_get_u24(&cbs, &msg_len) || !CBS_skip(&cbs, msg_len) ||
          CBS_len(&cbs) != 0) {
        fprintf(stderr, "Could not parse handshake message.\n");
        state->msg_callback_ok = false;
        return;
      }
      char text[16];
      if (content_type == SSL3_RT_CLIENT_HELLO_INNER) {
        if (type != SSL3_MT_CLIENT_HELLO) {
          fprintf(stderr, "Invalid header for ClientHelloInner.\n");
          state->msg_callback_ok = false;
          return;
        }
        state->msg_callback_text += "clienthelloinner\n";
      } else {
        snprintf(text, sizeof(text), "hs %d\n", type);
        state->msg_callback_text += text;
        if (!is_write) {
          state->last_message_received = type;
        }
      }
      return;
    }

    case SSL3_RT_CHANGE_CIPHER_SPEC:
      if (len != 1 || buf_u8[0] != 1) {
        fprintf(stderr, "Invalid ChangeCipherSpec.\n");
        state->msg_callback_ok = false;
        return;
      }
      state->msg_callback_text += "ccs\n";
      return;

    case SSL3_RT_ALERT:
      if (len != 2) {
        fprintf(stderr, "Invalid alert.\n");
        state->msg_callback_ok = false;
        return;
      }
      char text[16];
      snprintf(text, sizeof(text), "alert %d %d\n", buf_u8[0], buf_u8[1]);
      state->msg_callback_text += text;
      return;

    default:
      fprintf(stderr, "Invalid content_type: %d.\n", content_type);
      state->msg_callback_ok = false;
  }
}

static int TicketKeyCallback(SSL *ssl, uint8_t *key_name, uint8_t *iv,
                             EVP_CIPHER_CTX *ctx, HMAC_CTX *hmac_ctx,
                             int encrypt) {
  if (!encrypt) {
    if (GetTestState(ssl)->ticket_decrypt_done) {
      fprintf(stderr, "TicketKeyCallback called after completion.\n");
      return -1;
    }

    GetTestState(ssl)->ticket_decrypt_done = true;
  }

  // This is just test code, so use the all-zeros key.
  static const uint8_t kZeros[16] = {0};

  if (encrypt) {
    OPENSSL_memcpy(key_name, kZeros, sizeof(kZeros));
    RAND_bytes(iv, 16);
  } else if (OPENSSL_memcmp(key_name, kZeros, 16) != 0) {
    return 0;
  }

  if (!HMAC_Init_ex(hmac_ctx, kZeros, sizeof(kZeros), EVP_sha256(), NULL) ||
      !EVP_CipherInit_ex(ctx, EVP_aes_128_cbc(), NULL, kZeros, iv, encrypt)) {
    return -1;
  }

  if (!encrypt) {
    return GetTestConfig(ssl)->renew_ticket ? 2 : 1;
  }
  return 1;
}

static int NewSessionCallback(SSL *ssl, SSL_SESSION *session) {
  // This callback is called as the handshake completes. |SSL_get_session|
  // must continue to work and, historically, |SSL_in_init| returned false at
  // this point.
  if (SSL_in_init(ssl) || SSL_get_session(ssl) == nullptr) {
    fprintf(stderr, "Invalid state for NewSessionCallback.\n");
    abort();
  }

  GetTestState(ssl)->got_new_session = true;
  GetTestState(ssl)->new_session.reset(session);
  return 1;
}

static void InfoCallback(const SSL *ssl, int type, int val) {
  if (type == SSL_CB_HANDSHAKE_DONE) {
    if (GetTestConfig(ssl)->handshake_never_done) {
      fprintf(stderr, "Handshake unexpectedly completed.\n");
      // Abort before any expected error code is printed, to ensure the overall
      // test fails.
      abort();
    }
    // This callback is called when the handshake completes. |SSL_get_session|
    // must continue to work and |SSL_in_init| must return false.
    if (SSL_in_init(ssl) || SSL_get_session(ssl) == nullptr) {
      fprintf(stderr, "Invalid state for SSL_CB_HANDSHAKE_DONE.\n");
      abort();
    }
    GetTestState(ssl)->handshake_done = true;
  }
}

static SSL_SESSION *GetSessionCallback(SSL *ssl, const uint8_t *data, int len,
                                       int *copy) {
  TestState *async_state = GetTestState(ssl);
  if (async_state->session) {
    *copy = 0;
    return async_state->session.release();
  } else if (async_state->pending_session) {
    return SSL_magic_pending_session_ptr();
  } else {
    return NULL;
  }
}

static void CurrentTimeCallback(const SSL *ssl, timeval *out_clock) {
  *out_clock = *GetClock();
}

static int AlpnSelectCallback(SSL *ssl, const uint8_t **out, uint8_t *outlen,
                              const uint8_t *in, unsigned inlen, void *arg) {
  if (GetTestState(ssl)->alpn_select_done) {
    fprintf(stderr, "AlpnSelectCallback called after completion.\n");
    exit(1);
  }

  GetTestState(ssl)->alpn_select_done = true;

  const TestConfig *config = GetTestConfig(ssl);
  if (config->decline_alpn) {
    return SSL_TLSEXT_ERR_NOACK;
  }
  if (config->reject_alpn) {
    return SSL_TLSEXT_ERR_ALERT_FATAL;
  }

  if (!config->expect_advertised_alpn.empty() &&
      (config->expect_advertised_alpn.size() != inlen ||
       OPENSSL_memcmp(config->expect_advertised_alpn.data(), in, inlen) !=
           0)) {
    fprintf(stderr, "bad ALPN select callback inputs.\n");
    exit(1);
  }

  if (config->defer_alps) {
    for (const auto &pair : config->application_settings) {
      if (!SSL_add_application_settings(
              ssl, reinterpret_cast<const uint8_t *>(pair.first.data()),
              pair.first.size(),
              reinterpret_cast<const uint8_t *>(pair.second.data()),
              pair.second.size())) {
        fprintf(stderr, "error configuring ALPS.\n");
        exit(1);
      }
    }
  }

  assert(config->select_alpn.empty() || !config->select_empty_alpn);
  *out = (const uint8_t *)config->select_alpn.data();
  *outlen = config->select_alpn.size();
  return SSL_TLSEXT_ERR_OK;
}

static bool CheckVerifyCallback(SSL *ssl) {
  const TestConfig *config = GetTestConfig(ssl);
  if (!config->expect_ocsp_response.empty()) {
    const uint8_t *data;
    size_t len;
    SSL_get0_ocsp_response(ssl, &data, &len);
    if (len == 0) {
      fprintf(stderr, "OCSP response not available in verify callback.\n");
      return false;
    }
  }

  const char *name_override;
  size_t name_override_len;
  SSL_get0_ech_name_override(ssl, &name_override, &name_override_len);
  if (config->expect_no_ech_name_override && name_override_len != 0) {
    fprintf(stderr, "Unexpected ECH name override.\n");
    return false;
  }
  if (!config->expect_ech_name_override.empty() &&
      config->expect_ech_name_override !=
          std::string(name_override, name_override_len)) {
    fprintf(stderr, "ECH name did not match expected value.\n");
    return false;
  }

  if (GetTestState(ssl)->cert_verified) {
    fprintf(stderr, "Certificate verified twice.\n");
    return false;
  }

  return true;
}

static int CertVerifyCallback(X509_STORE_CTX *store_ctx, void *arg) {
  SSL *ssl = (SSL *)X509_STORE_CTX_get_ex_data(
      store_ctx, SSL_get_ex_data_X509_STORE_CTX_idx());
  const TestConfig *config = GetTestConfig(ssl);
  if (!CheckVerifyCallback(ssl)) {
    return 0;
  }

  GetTestState(ssl)->cert_verified = true;
  if (config->verify_fail) {
    X509_STORE_CTX_set_error(store_ctx, X509_V_ERR_APPLICATION_VERIFICATION);
    return 0;
  }

  return 1;
}

bool LoadCertificate(bssl::UniquePtr<X509> *out_x509,
                     bssl::UniquePtr<STACK_OF(X509)> *out_chain,
                     const std::string &file) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_file()));
  if (!bio || !BIO_read_filename(bio.get(), file.c_str())) {
    return false;
  }

  out_x509->reset(PEM_read_bio_X509(bio.get(), nullptr, nullptr, nullptr));
  if (!*out_x509) {
    return false;
  }

  out_chain->reset(sk_X509_new_null());
  if (!*out_chain) {
    return false;
  }

  // Keep reading the certificate chain.
  for (;;) {
    bssl::UniquePtr<X509> cert(
        PEM_read_bio_X509(bio.get(), nullptr, nullptr, nullptr));
    if (!cert) {
      break;
    }

    if (!bssl::PushToStack(out_chain->get(), std::move(cert))) {
      return false;
    }
  }

  uint32_t err = ERR_peek_last_error();
  if (ERR_GET_LIB(err) != ERR_LIB_PEM ||
      ERR_GET_REASON(err) != PEM_R_NO_START_LINE) {
    return false;
  }

  ERR_clear_error();
  return true;
}

bssl::UniquePtr<EVP_PKEY> LoadPrivateKey(const std::string &file) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_file()));
  if (!bio || !BIO_read_filename(bio.get(), file.c_str())) {
    return nullptr;
  }
  return bssl::UniquePtr<EVP_PKEY>(
      PEM_read_bio_PrivateKey(bio.get(), NULL, NULL, NULL));
}

static bool GetCertificate(SSL *ssl, bssl::UniquePtr<X509> *out_x509,
                           bssl::UniquePtr<STACK_OF(X509)> *out_chain,
                           bssl::UniquePtr<EVP_PKEY> *out_pkey) {
  const TestConfig *config = GetTestConfig(ssl);

  if (!config->signing_prefs.empty()) {
    if (!SSL_set_signing_algorithm_prefs(ssl, config->signing_prefs.data(),
                                         config->signing_prefs.size())) {
      return false;
    }
  }

  if (!config->key_file.empty()) {
    *out_pkey = LoadPrivateKey(config->key_file.c_str());
    if (!*out_pkey) {
      return false;
    }
  }
  if (!config->cert_file.empty() &&
      !LoadCertificate(out_x509, out_chain, config->cert_file.c_str())) {
    return false;
  }
  if (!config->ocsp_response.empty() && !config->set_ocsp_in_callback &&
      !SSL_set_ocsp_response(ssl, (const uint8_t *)config->ocsp_response.data(),
                             config->ocsp_response.size())) {
    return false;
  }
  return true;
}

static bool HexDecode(std::string *out, const std::string &in) {
  if ((in.size() & 1) != 0) {
    return false;
  }

  std::unique_ptr<uint8_t[]> buf(new uint8_t[in.size() / 2]);
  for (size_t i = 0; i < in.size() / 2; i++) {
    uint8_t high, low;
    if (!OPENSSL_fromxdigit(&high, in[i * 2]) ||
        !OPENSSL_fromxdigit(&low, in[i * 2 + 1])) {
      return false;
    }
    buf[i] = (high << 4) | low;
  }

  out->assign(reinterpret_cast<const char *>(buf.get()), in.size() / 2);
  return true;
}

static std::vector<std::string> SplitParts(const std::string &in,
                                           const char delim) {
  std::vector<std::string> ret;
  size_t start = 0;

  for (size_t i = 0; i < in.size(); i++) {
    if (in[i] == delim) {
      ret.push_back(in.substr(start, i - start));
      start = i + 1;
    }
  }

  ret.push_back(in.substr(start, std::string::npos));
  return ret;
}

static std::vector<std::string> DecodeHexStrings(
    const std::string &hex_strings) {
  std::vector<std::string> ret;
  const std::vector<std::string> parts = SplitParts(hex_strings, ',');

  for (const auto &part : parts) {
    std::string binary;
    if (!HexDecode(&binary, part)) {
      fprintf(stderr, "Bad hex string: %s.\n", part.c_str());
      return ret;
    }

    ret.push_back(std::move(binary));
  }

  return ret;
}

static bssl::UniquePtr<STACK_OF(X509_NAME)> DecodeHexX509Names(
    const std::string &hex_names) {
  const std::vector<std::string> der_names = DecodeHexStrings(hex_names);
  bssl::UniquePtr<STACK_OF(X509_NAME)> ret(sk_X509_NAME_new_null());
  if (!ret) {
    return nullptr;
  }

  for (const auto &der_name : der_names) {
    const uint8_t *const data =
        reinterpret_cast<const uint8_t *>(der_name.data());
    const uint8_t *derp = data;
    bssl::UniquePtr<X509_NAME> name(
        d2i_X509_NAME(nullptr, &derp, der_name.size()));
    if (!name || derp != data + der_name.size()) {
      fprintf(stderr, "Failed to parse X509_NAME.\n");
      return nullptr;
    }

    if (!bssl::PushToStack(ret.get(), std::move(name))) {
      return nullptr;
    }
  }

  return ret;
}

static bool CheckPeerVerifyPrefs(SSL *ssl) {
  const TestConfig *config = GetTestConfig(ssl);
  if (!config->expect_peer_verify_prefs.empty()) {
    const uint16_t *peer_sigalgs;
    size_t num_peer_sigalgs =
        SSL_get0_peer_verify_algorithms(ssl, &peer_sigalgs);
    if (config->expect_peer_verify_prefs.size() != num_peer_sigalgs) {
      fprintf(stderr,
              "peer verify preferences length mismatch (got %zu, wanted %zu)\n",
              num_peer_sigalgs, config->expect_peer_verify_prefs.size());
      return false;
    }
    for (size_t i = 0; i < num_peer_sigalgs; i++) {
      if (peer_sigalgs[i] != config->expect_peer_verify_prefs[i]) {
        fprintf(stderr,
                "peer verify preference %zu mismatch (got %04x, wanted %04x\n",
                i, peer_sigalgs[i], config->expect_peer_verify_prefs[i]);
        return false;
      }
    }
  }
  return true;
}

static bool CheckCertificateRequest(SSL *ssl) {
  const TestConfig *config = GetTestConfig(ssl);

  if (!CheckPeerVerifyPrefs(ssl)) {
    return false;
  }

  if (!config->expect_certificate_types.empty()) {
    const uint8_t *certificate_types;
    size_t certificate_types_len =
        SSL_get0_certificate_types(ssl, &certificate_types);
    if (certificate_types_len != config->expect_certificate_types.size() ||
        OPENSSL_memcmp(certificate_types,
                       config->expect_certificate_types.data(),
                       certificate_types_len) != 0) {
      fprintf(stderr, "certificate types mismatch.\n");
      return false;
    }
  }

  if (!config->expect_client_ca_list.empty()) {
    bssl::UniquePtr<STACK_OF(X509_NAME)> expected =
        DecodeHexX509Names(config->expect_client_ca_list);
    const size_t num_expected = sk_X509_NAME_num(expected.get());

    const STACK_OF(X509_NAME) *received = SSL_get_client_CA_list(ssl);
    const size_t num_received = sk_X509_NAME_num(received);

    if (num_received != num_expected) {
      fprintf(stderr, "expected %zu names in CertificateRequest but got %zu.\n",
              num_expected, num_received);
      return false;
    }

    for (size_t i = 0; i < num_received; i++) {
      if (X509_NAME_cmp(sk_X509_NAME_value(received, i),
                        sk_X509_NAME_value(expected.get(), i)) != 0) {
        fprintf(stderr, "names in CertificateRequest differ at index #%zu.\n",
                i);
        return false;
      }
    }

    const STACK_OF(CRYPTO_BUFFER) *buffers = SSL_get0_server_requested_CAs(ssl);
    if (sk_CRYPTO_BUFFER_num(buffers) != num_received) {
      fprintf(stderr,
              "Mismatch between SSL_get_server_requested_CAs and "
              "SSL_get_client_CA_list.\n");
      return false;
    }
  }

  return true;
}

static int ClientCertCallback(SSL *ssl, X509 **out_x509, EVP_PKEY **out_pkey) {
  if (!CheckCertificateRequest(ssl)) {
    return -1;
  }

  if (GetTestConfig(ssl)->async && !GetTestState(ssl)->cert_ready) {
    return -1;
  }

  bssl::UniquePtr<X509> x509;
  bssl::UniquePtr<STACK_OF(X509)> chain;
  bssl::UniquePtr<EVP_PKEY> pkey;
  if (!GetCertificate(ssl, &x509, &chain, &pkey)) {
    return -1;
  }

  // Return zero for no certificate.
  if (!x509) {
    return 0;
  }

  // Chains and asynchronous private keys are not supported with client_cert_cb.
  *out_x509 = x509.release();
  *out_pkey = pkey.release();
  return 1;
}

static ssl_private_key_result_t AsyncPrivateKeyComplete(SSL *ssl, uint8_t *out,
                                                        size_t *out_len,
                                                        size_t max_out);

static ssl_private_key_result_t AsyncPrivateKeySign(
    SSL *ssl, uint8_t *out, size_t *out_len, size_t max_out,
    uint16_t signature_algorithm, const uint8_t *in, size_t in_len) {
  TestState *test_state = GetTestState(ssl);
  test_state->used_private_key = true;
  if (!test_state->private_key_result.empty()) {
    fprintf(stderr, "AsyncPrivateKeySign called with operation pending.\n");
    abort();
  }

  if (EVP_PKEY_id(test_state->private_key.get()) !=
      SSL_get_signature_algorithm_key_type(signature_algorithm)) {
    fprintf(stderr, "Key type does not match signature algorithm.\n");
    abort();
  }

  // Determine the hash.
  const EVP_MD *md = SSL_get_signature_algorithm_digest(signature_algorithm);
  bssl::ScopedEVP_MD_CTX ctx;
  EVP_PKEY_CTX *pctx;
  if (!EVP_DigestSignInit(ctx.get(), &pctx, md, nullptr,
                          test_state->private_key.get())) {
    return ssl_private_key_failure;
  }

  // Configure additional signature parameters.
  if (SSL_is_signature_algorithm_rsa_pss(signature_algorithm)) {
    if (!EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING) ||
        !EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, -1 /* salt len = hash len */)) {
      return ssl_private_key_failure;
    }
  }

  // Write the signature into |test_state|.
  size_t len = 0;
  if (!EVP_DigestSign(ctx.get(), nullptr, &len, in, in_len)) {
    return ssl_private_key_failure;
  }
  test_state->private_key_result.resize(len);
  if (!EVP_DigestSign(ctx.get(), test_state->private_key_result.data(), &len,
                      in, in_len)) {
    return ssl_private_key_failure;
  }
  test_state->private_key_result.resize(len);

  return AsyncPrivateKeyComplete(ssl, out, out_len, max_out);
}

static ssl_private_key_result_t AsyncPrivateKeyDecrypt(SSL *ssl, uint8_t *out,
                                                       size_t *out_len,
                                                       size_t max_out,
                                                       const uint8_t *in,
                                                       size_t in_len) {
  TestState *test_state = GetTestState(ssl);
  test_state->used_private_key = true;
  if (!test_state->private_key_result.empty()) {
    fprintf(stderr, "AsyncPrivateKeyDecrypt called with operation pending.\n");
    abort();
  }

  RSA *rsa = EVP_PKEY_get0_RSA(test_state->private_key.get());
  if (rsa == NULL) {
    fprintf(stderr, "AsyncPrivateKeyDecrypt called with incorrect key type.\n");
    abort();
  }
  test_state->private_key_result.resize(RSA_size(rsa));
  if (!RSA_decrypt(rsa, out_len, test_state->private_key_result.data(),
                   RSA_size(rsa), in, in_len, RSA_NO_PADDING)) {
    return ssl_private_key_failure;
  }

  test_state->private_key_result.resize(*out_len);

  return AsyncPrivateKeyComplete(ssl, out, out_len, max_out);
}

static ssl_private_key_result_t AsyncPrivateKeyComplete(SSL *ssl, uint8_t *out,
                                                        size_t *out_len,
                                                        size_t max_out) {
  TestState *test_state = GetTestState(ssl);
  if (test_state->private_key_result.empty()) {
    fprintf(stderr,
            "AsyncPrivateKeyComplete called without operation pending.\n");
    abort();
  }

  if (GetTestConfig(ssl)->async && test_state->private_key_retries < 2) {
    // Only return the decryption on the second attempt, to test both incomplete
    // |sign|/|decrypt| and |complete|.
    return ssl_private_key_retry;
  }

  if (max_out < test_state->private_key_result.size()) {
    fprintf(stderr, "Output buffer too small.\n");
    return ssl_private_key_failure;
  }
  OPENSSL_memcpy(out, test_state->private_key_result.data(),
                 test_state->private_key_result.size());
  *out_len = test_state->private_key_result.size();

  test_state->private_key_result.clear();
  test_state->private_key_retries = 0;
  return ssl_private_key_success;
}

static const SSL_PRIVATE_KEY_METHOD g_async_private_key_method = {
    AsyncPrivateKeySign,
    AsyncPrivateKeyDecrypt,
    AsyncPrivateKeyComplete,
};

static bool InstallCertificate(SSL *ssl) {
  bssl::UniquePtr<X509> x509;
  bssl::UniquePtr<STACK_OF(X509)> chain;
  bssl::UniquePtr<EVP_PKEY> pkey;
  if (!GetCertificate(ssl, &x509, &chain, &pkey)) {
    return false;
  }

  if (pkey) {
    TestState *test_state = GetTestState(ssl);
    const TestConfig *config = GetTestConfig(ssl);
    if (config->async || config->handshake_hints) {
      // Install a custom private key if testing asynchronous callbacks, or if
      // testing handshake hints. In the handshake hints case, we wish to check
      // that hints only mismatch when allowed.
      test_state->private_key = std::move(pkey);
      SSL_set_private_key_method(ssl, &g_async_private_key_method);
    } else if (!SSL_use_PrivateKey(ssl, pkey.get())) {
      return false;
    }
  }

  if (x509 && !SSL_use_certificate(ssl, x509.get())) {
    return false;
  }

  if (sk_X509_num(chain.get()) > 0 && !SSL_set1_chain(ssl, chain.get())) {
    return false;
  }

  return true;
}

static bool InstallMultipleCertificates(SSL *ssl) {
  const TestConfig *config = GetTestConfig(ssl);
  if (config->multiple_certs_slot.empty()) {
    return false;
  }

  if (!config->signing_prefs.empty()) {
    if (!SSL_set_signing_algorithm_prefs(ssl, config->signing_prefs.data(),
                                         config->signing_prefs.size())) {
      return false;
    }
  }

  for (const auto &cert_key_pair : config->multiple_certs_slot) {
    bssl::UniquePtr<X509> x509;
    bssl::UniquePtr<STACK_OF(X509)> chain;
    bssl::UniquePtr<EVP_PKEY> pkey;

    if (!LoadCertificate(&x509, &chain, cert_key_pair.first)) {
      return false;
    }
    pkey = LoadPrivateKey(cert_key_pair.second);
    if (pkey && !SSL_use_PrivateKey(ssl, pkey.get())) {
      return false;
    }
    if (x509 && !SSL_use_certificate(ssl, x509.get())) {
      return false;
    }
    if (sk_X509_num(chain.get()) > 0 && !SSL_set1_chain(ssl, chain.get())) {
      return false;
    }
  }
  return true;
}


static enum ssl_select_cert_result_t SelectCertificateCallback(
    const SSL_CLIENT_HELLO *client_hello) {
  SSL *ssl = client_hello->ssl;
  const TestConfig *config = GetTestConfig(ssl);
  TestState *test_state = GetTestState(ssl);
  test_state->early_callback_called = true;

  if (!config->expect_server_name.empty()) {
    const char *server_name =
        SSL_get_servername(ssl, TLSEXT_NAMETYPE_host_name);
    if (server_name == nullptr ||
        std::string(server_name) != config->expect_server_name) {
      fprintf(stderr,
              "Server name mismatch in early callback (got %s; want %s).\n",
              server_name, config->expect_server_name.c_str());
      return ssl_select_cert_error;
    }
  }

  if (config->fail_early_callback) {
    return ssl_select_cert_error;
  }

  // Simulate some asynchronous work in the early callback.
  if ((config->use_early_callback || test_state->get_handshake_hints_cb) &&
      config->async && !test_state->early_callback_ready) {
    return ssl_select_cert_retry;
  }

  if (test_state->get_handshake_hints_cb &&
      !test_state->get_handshake_hints_cb(client_hello)) {
    return ssl_select_cert_error;
  }

  if (config->use_early_callback &&
      !InstallMultipleCertificates(ssl) &&
      !InstallCertificate(ssl)) {
    return ssl_select_cert_error;
  }

  return ssl_select_cert_success;
}

static int SetQuicReadSecret(SSL *ssl, enum ssl_encryption_level_t level,
                             const SSL_CIPHER *cipher, const uint8_t *secret,
                             size_t secret_len) {
  MockQuicTransport *quic_transport = GetTestState(ssl)->quic_transport.get();
  if (quic_transport == nullptr) {
    fprintf(stderr, "No QUIC transport.\n");
    return 0;
  }
  return quic_transport->SetReadSecret(level, cipher, secret, secret_len);
}

static int SetQuicWriteSecret(SSL *ssl, enum ssl_encryption_level_t level,
                              const SSL_CIPHER *cipher, const uint8_t *secret,
                              size_t secret_len) {
  MockQuicTransport *quic_transport = GetTestState(ssl)->quic_transport.get();
  if (quic_transport == nullptr) {
    fprintf(stderr, "No QUIC transport.\n");
    return 0;
  }
  return quic_transport->SetWriteSecret(level, cipher, secret, secret_len);
}

static int AddQuicHandshakeData(SSL *ssl, enum ssl_encryption_level_t level,
                                const uint8_t *data, size_t len) {
  MockQuicTransport *quic_transport = GetTestState(ssl)->quic_transport.get();
  if (quic_transport == nullptr) {
    fprintf(stderr, "No QUIC transport.\n");
    return 0;
  }
  return quic_transport->WriteHandshakeData(level, data, len);
}

static int FlushQuicFlight(SSL *ssl) {
  MockQuicTransport *quic_transport = GetTestState(ssl)->quic_transport.get();
  if (quic_transport == nullptr) {
    fprintf(stderr, "No QUIC transport.\n");
    return 0;
  }
  return quic_transport->Flush();
}

static int SendQuicAlert(SSL *ssl, enum ssl_encryption_level_t level,
                         uint8_t alert) {
  MockQuicTransport *quic_transport = GetTestState(ssl)->quic_transport.get();
  if (quic_transport == nullptr) {
    fprintf(stderr, "No QUIC transport.\n");
    return 0;
  }
  return quic_transport->SendAlert(level, alert);
}

static const SSL_QUIC_METHOD g_quic_method = {
    SetQuicReadSecret,
    SetQuicWriteSecret,
    AddQuicHandshakeData,
    FlushQuicFlight,
    SendQuicAlert,
};

static bool MaybeInstallCertCompressionAlg(
    const TestConfig *config, SSL_CTX *ssl_ctx, uint16_t alg,
    ssl_cert_compression_func_t compress,
    ssl_cert_decompression_func_t decompress) {
  if (!config->install_cert_compression_algs &&
      config->install_one_cert_compression_alg != alg) {
    return true;
  }
  return SSL_CTX_add_cert_compression_alg(ssl_ctx, alg, compress, decompress);
}

bssl::UniquePtr<SSL_CTX> TestConfig::SetupCtx(SSL_CTX *old_ctx) const {
  bssl::UniquePtr<SSL_CTX> ssl_ctx(
      SSL_CTX_new(is_dtls ? DTLS_method() : TLS_method()));
  if (!ssl_ctx) {
    return nullptr;
  }

  CRYPTO_once(&once, init_once);
  SSL_CTX_set0_buffer_pool(ssl_ctx.get(), g_pool);

  std::string cipher_list = "ALL";
  if (!cipher.empty()) {
    cipher_list = cipher;
    SSL_CTX_set_options(ssl_ctx.get(), SSL_OP_CIPHER_SERVER_PREFERENCE);
  }
  if (!SSL_CTX_set_strict_cipher_list(ssl_ctx.get(), cipher_list.c_str())) {
    return nullptr;
  }
  if (!tls13_ciphersuites.empty()
     && !SSL_CTX_set_ciphersuites(ssl_ctx.get(), tls13_ciphersuites.c_str())) {
    return nullptr;
  }

  if (async && is_server) {
    // Disable the internal session cache. To test asynchronous session lookup,
    // we use an external session cache.
    SSL_CTX_set_session_cache_mode(
        ssl_ctx.get(), SSL_SESS_CACHE_BOTH | SSL_SESS_CACHE_NO_INTERNAL);
    SSL_CTX_sess_set_get_cb(ssl_ctx.get(), GetSessionCallback);
  } else {
    SSL_CTX_set_session_cache_mode(ssl_ctx.get(), SSL_SESS_CACHE_BOTH);
  }

  SSL_CTX_set_select_certificate_cb(ssl_ctx.get(), SelectCertificateCallback);

  if (use_old_client_cert_callback) {
    SSL_CTX_set_client_cert_cb(ssl_ctx.get(), ClientCertCallback);
  }

  SSL_CTX_set_next_protos_advertised_cb(ssl_ctx.get(),
                                        NextProtosAdvertisedCallback, NULL);
  if (!select_next_proto.empty()) {
    SSL_CTX_set_next_proto_select_cb(ssl_ctx.get(), NextProtoSelectCallback,
                                     NULL);
  }

  if (!select_alpn.empty() || decline_alpn || reject_alpn ||
      select_empty_alpn) {
    SSL_CTX_set_alpn_select_cb(ssl_ctx.get(), AlpnSelectCallback, NULL);
  }

  SSL_CTX_set_current_time_cb(ssl_ctx.get(), CurrentTimeCallback);

  SSL_CTX_set_info_callback(ssl_ctx.get(), InfoCallback);
  SSL_CTX_sess_set_new_cb(ssl_ctx.get(), NewSessionCallback);

  if (use_ticket_callback || handshake_hints) {
    // If using handshake hints, always enable the ticket callback, so we can
    // check that hints only mismatch when allowed. The ticket callback also
    // uses a constant key, which simplifies the test.
    SSL_CTX_set_tlsext_ticket_key_cb(ssl_ctx.get(), TicketKeyCallback);
  }

  if (enable_client_custom_extension &&
      !SSL_CTX_add_client_custom_ext(
          ssl_ctx.get(), kCustomExtensionValue, CustomExtensionAddCallback,
          CustomExtensionFreeCallback, kCustomExtensionAddArg,
          CustomExtensionParseCallback, kCustomExtensionParseArg)) {
    return nullptr;
  }

  if (enable_server_custom_extension &&
      !SSL_CTX_add_server_custom_ext(
          ssl_ctx.get(), kCustomExtensionValue, CustomExtensionAddCallback,
          CustomExtensionFreeCallback, kCustomExtensionAddArg,
          CustomExtensionParseCallback, kCustomExtensionParseArg)) {
    return nullptr;
  }

  if (!use_custom_verify_callback) {
    SSL_CTX_set_cert_verify_callback(ssl_ctx.get(), CertVerifyCallback, NULL);
  }

  if (!signed_cert_timestamps.empty() &&
      !SSL_CTX_set_signed_cert_timestamp_list(
          ssl_ctx.get(), (const uint8_t *)signed_cert_timestamps.data(),
          signed_cert_timestamps.size())) {
    return nullptr;
  }

  if (!use_client_ca_list.empty()) {
    if (use_client_ca_list == "<NULL>") {
      SSL_CTX_set_client_CA_list(ssl_ctx.get(), nullptr);
    } else if (use_client_ca_list == "<EMPTY>") {
      bssl::UniquePtr<STACK_OF(X509_NAME)> names;
      SSL_CTX_set_client_CA_list(ssl_ctx.get(), names.release());
    } else {
      bssl::UniquePtr<STACK_OF(X509_NAME)> names =
          DecodeHexX509Names(use_client_ca_list);
      SSL_CTX_set_client_CA_list(ssl_ctx.get(), names.release());
    }
  }

  if (enable_grease) {
    SSL_CTX_set_grease_enabled(ssl_ctx.get(), 1);
  }

  if (permute_extensions) {
    SSL_CTX_set_permute_extensions(ssl_ctx.get(), 1);
  }

  if (!expect_server_name.empty()) {
    SSL_CTX_set_tlsext_servername_callback(ssl_ctx.get(), ServerNameCallback);
  }

  if (enable_early_data) {
    SSL_CTX_set_early_data_enabled(ssl_ctx.get(), 1);
  }

  if (allow_unknown_alpn_protos) {
    SSL_CTX_set_allow_unknown_alpn_protos(ssl_ctx.get(), 1);
  }

  if (!verify_prefs.empty()) {
    if (!SSL_CTX_set_verify_algorithm_prefs(ssl_ctx.get(), verify_prefs.data(),
                                            verify_prefs.size())) {
      return nullptr;
    }
  }

  SSL_CTX_set_msg_callback(ssl_ctx.get(), MessageCallback);

  if (allow_false_start_without_alpn) {
    SSL_CTX_set_false_start_allowed_without_alpn(ssl_ctx.get(), 1);
  }

  if (use_ocsp_callback) {
    SSL_CTX_set_tlsext_status_cb(ssl_ctx.get(), LegacyOCSPCallback);
    int (*cb)(SSL *, void *) = nullptr;
    if(!SSL_CTX_get_tlsext_status_cb(ssl_ctx.get(), &cb) ||
        cb != LegacyOCSPCallback){
      return nullptr;
    }
  }

  if (old_ctx) {
    uint8_t keys[48];
    if (!SSL_CTX_get_tlsext_ticket_keys(old_ctx, &keys, sizeof(keys)) ||
        !SSL_CTX_set_tlsext_ticket_keys(ssl_ctx.get(), keys, sizeof(keys))) {
      return nullptr;
    }
    CopySessions(ssl_ctx.get(), old_ctx);
  } else if (!ticket_key.empty() &&
             !SSL_CTX_set_tlsext_ticket_keys(ssl_ctx.get(), ticket_key.data(),
                                             ticket_key.size())) {
    return nullptr;
  }

  // These mock compression algorithms match the corresponding ones in
  // |addCertCompressionTests|.
  if (!MaybeInstallCertCompressionAlg(
          this, ssl_ctx.get(), 0xff02,
          [](SSL *ssl, CBB *out, const uint8_t *in, size_t in_len) -> int {
            if (!CBB_add_u8(out, 1) || !CBB_add_u8(out, 2) ||
                !CBB_add_u8(out, 3) || !CBB_add_u8(out, 4) ||
                !CBB_add_bytes(out, in, in_len)) {
              return 0;
            }
            return 1;
          },
          [](SSL *ssl, CRYPTO_BUFFER **out, size_t uncompressed_len,
             const uint8_t *in, size_t in_len) -> int {
            if (in_len < 4 || in[0] != 1 || in[1] != 2 || in[2] != 3 ||
                in[3] != 4 || uncompressed_len != in_len - 4) {
              return 0;
            }
            const bssl::Span<const uint8_t> uncompressed(in + 4, in_len - 4);
            *out = CRYPTO_BUFFER_new(uncompressed.data(), uncompressed.size(),
                                     nullptr);
            return *out != nullptr;
          }) ||
      !MaybeInstallCertCompressionAlg(
          this, ssl_ctx.get(), 0xff01,
          [](SSL *ssl, CBB *out, const uint8_t *in, size_t in_len) -> int {
            if (in_len < 2 || in[0] != 0 || in[1] != 0) {
              return 0;
            }
            return CBB_add_bytes(out, in + 2, in_len - 2);
          },
          [](SSL *ssl, CRYPTO_BUFFER **out, size_t uncompressed_len,
             const uint8_t *in, size_t in_len) -> int {
            if (uncompressed_len != 2 + in_len) {
              return 0;
            }
            std::unique_ptr<uint8_t[]> buf(new uint8_t[2 + in_len]);
            buf[0] = 0;
            buf[1] = 0;
            OPENSSL_memcpy(&buf[2], in, in_len);
            *out = CRYPTO_BUFFER_new(buf.get(), 2 + in_len, nullptr);
            return *out != nullptr;
          }) ||
      !MaybeInstallCertCompressionAlg(
          this, ssl_ctx.get(), 0xff03,
          [](SSL *ssl, CBB *out, const uint8_t *in, size_t in_len) -> int {
            uint8_t byte;
            return RAND_bytes(&byte, 1) &&   //
                   CBB_add_u8(out, byte) &&  //
                   CBB_add_bytes(out, in, in_len);
          },
          [](SSL *ssl, CRYPTO_BUFFER **out, size_t uncompressed_len,
             const uint8_t *in, size_t in_len) -> int {
            if (uncompressed_len + 1 != in_len) {
              return 0;
            }
            *out = CRYPTO_BUFFER_new(in + 1, in_len - 1, nullptr);
            return *out != nullptr;
          })) {
    fprintf(stderr, "SSL_CTX_add_cert_compression_alg failed.\n");
    abort();
  }

  if (server_preference) {
    SSL_CTX_set_options(ssl_ctx.get(), SSL_OP_CIPHER_SERVER_PREFERENCE);
  }

  if (is_quic) {
    SSL_CTX_set_quic_method(ssl_ctx.get(), &g_quic_method);
  }

  return ssl_ctx;
}

static int DDoSCallback(const SSL_CLIENT_HELLO *client_hello) {
  const TestConfig *config = GetTestConfig(client_hello->ssl);
  return config->fail_ddos_callback ? 0 : 1;
}

static unsigned PskClientCallback(SSL *ssl, const char *hint,
                                  char *out_identity, unsigned max_identity_len,
                                  uint8_t *out_psk, unsigned max_psk_len) {
  const TestConfig *config = GetTestConfig(ssl);

  if (config->psk_identity.empty()) {
    if (hint != nullptr) {
      fprintf(stderr, "Server PSK hint was non-null.\n");
      return 0;
    }
  } else if (hint == nullptr ||
             strcmp(hint, config->psk_identity.c_str()) != 0) {
    fprintf(stderr, "Server PSK hint did not match.\n");
    return 0;
  }

  // Account for the trailing '\0' for the identity.
  if (config->psk_identity.size() >= max_identity_len ||
      config->psk.size() > max_psk_len) {
    fprintf(stderr, "PSK buffers too small.\n");
    return 0;
  }

  OPENSSL_strlcpy(out_identity, config->psk_identity.c_str(), max_identity_len);
  OPENSSL_memcpy(out_psk, config->psk.data(), config->psk.size());
  return static_cast<unsigned>(config->psk.size());
}

static unsigned PskServerCallback(SSL *ssl, const char *identity,
                                  uint8_t *out_psk, unsigned max_psk_len) {
  const TestConfig *config = GetTestConfig(ssl);

  if (strcmp(identity, config->psk_identity.c_str()) != 0) {
    fprintf(stderr, "Client PSK identity did not match.\n");
    return 0;
  }

  if (config->psk.size() > max_psk_len) {
    fprintf(stderr, "PSK buffers too small.\n");
    return 0;
  }

  OPENSSL_memcpy(out_psk, config->psk.data(), config->psk.size());
  return static_cast<unsigned>(config->psk.size());
}

static ssl_verify_result_t CustomVerifyCallback(SSL *ssl, uint8_t *out_alert) {
  const TestConfig *config = GetTestConfig(ssl);
  if (!CheckVerifyCallback(ssl)) {
    return ssl_verify_invalid;
  }

  if (config->async && !GetTestState(ssl)->custom_verify_ready) {
    return ssl_verify_retry;
  }

  GetTestState(ssl)->cert_verified = true;
  if (config->verify_fail) {
    return ssl_verify_invalid;
  }

  return ssl_verify_ok;
}

static int CertCallback(SSL *ssl, void *arg) {
  const TestConfig *config = GetTestConfig(ssl);

  // Check the peer certificate metadata is as expected.
  if ((!SSL_is_server(ssl) && !CheckCertificateRequest(ssl)) ||
      !CheckPeerVerifyPrefs(ssl)) {
    return -1;
  }

  if (config->fail_cert_callback) {
    return 0;
  }

  // The certificate will be installed via other means.
  if (!config->async || config->use_early_callback) {
    return 1;
  }

  if (!GetTestState(ssl)->cert_ready) {
    return -1;
  }
  if (!InstallCertificate(ssl)) {
    return 0;
  }
  return 1;
}

bssl::UniquePtr<SSL> TestConfig::NewSSL(
    SSL_CTX *ssl_ctx, SSL_SESSION *session,
    std::unique_ptr<TestState> test_state) const {
  bssl::UniquePtr<SSL> ssl(SSL_new(ssl_ctx));
  if (!ssl) {
    return nullptr;
  }

  if (!SetTestConfig(ssl.get(), this)) {
    return nullptr;
  }
  if (test_state != nullptr) {
    if (!SetTestState(ssl.get(), std::move(test_state))) {
      return nullptr;
    }
  }

  if (fallback_scsv && !SSL_set_mode(ssl.get(), SSL_MODE_SEND_FALLBACK_SCSV)) {
    return nullptr;
  }
  // Install the certificate synchronously if nothing else will handle it.
  if (!use_early_callback && !use_old_client_cert_callback && !async &&
      !InstallMultipleCertificates(ssl.get()) &&
      !InstallCertificate(ssl.get())) {
    return nullptr;
  }
  if (!use_old_client_cert_callback) {
    SSL_set_cert_cb(ssl.get(), CertCallback, nullptr);
  }
  int mode = SSL_VERIFY_NONE;
  if (require_any_client_certificate) {
    mode = SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT;
  }
  if (verify_peer) {
    mode = SSL_VERIFY_PEER;
  }
  if (verify_peer_if_no_obc) {
    // Set SSL_VERIFY_FAIL_IF_NO_PEER_CERT so testing whether client
    // certificates were requested is easy.
    mode = SSL_VERIFY_PEER | SSL_VERIFY_PEER_IF_NO_OBC |
           SSL_VERIFY_FAIL_IF_NO_PEER_CERT;
  }
  if (use_custom_verify_callback) {
    SSL_set_custom_verify(ssl.get(), mode, CustomVerifyCallback);
  } else if (mode != SSL_VERIFY_NONE) {
    SSL_set_verify(ssl.get(), mode, NULL);
  }
  if (false_start) {
    SSL_set_mode(ssl.get(), SSL_MODE_ENABLE_FALSE_START);
  }
  if (cbc_record_splitting) {
    SSL_set_mode(ssl.get(), SSL_MODE_CBC_RECORD_SPLITTING);
  }
  if (partial_write) {
    SSL_set_mode(ssl.get(), SSL_MODE_ENABLE_PARTIAL_WRITE);
  }
  if (reverify_on_resume) {
    SSL_CTX_set_reverify_on_resume(ssl_ctx, 1);
  }
  if (enforce_rsa_key_usage) {
    SSL_set_enforce_rsa_key_usage(ssl.get(), 1);
  }
  if (no_check_client_certificate_type) {
    SSL_set_check_client_certificate_type(ssl.get(), 0);
  }
  if (no_tls13) {
    SSL_set_options(ssl.get(), SSL_OP_NO_TLSv1_3);
  }
  if (no_tls12) {
    SSL_set_options(ssl.get(), SSL_OP_NO_TLSv1_2);
  }
  if (no_tls11) {
    SSL_set_options(ssl.get(), SSL_OP_NO_TLSv1_1);
  }
  if (no_tls1) {
    SSL_set_options(ssl.get(), SSL_OP_NO_TLSv1);
  }
  if (no_ticket) {
    SSL_set_options(ssl.get(), SSL_OP_NO_TICKET);
  }
  if (!expect_channel_id.empty() || enable_channel_id) {
    SSL_set_tls_channel_id_enabled(ssl.get(), 1);
  }
  if (enable_ech_grease) {
    SSL_set_enable_ech_grease(ssl.get(), 1);
  }
  if (!ech_config_list.empty() &&
      !SSL_set1_ech_config_list(
          ssl.get(), reinterpret_cast<const uint8_t *>(ech_config_list.data()),
          ech_config_list.size())) {
    return nullptr;
  }
  if (ech_server_configs.size() != ech_server_keys.size() ||
      ech_server_configs.size() != ech_is_retry_config.size()) {
    fprintf(stderr,
            "-ech-server-config, -ech-server-key, and -ech-is-retry-config "
            "flags must match.\n");
    return nullptr;
  }
  if (!ech_server_configs.empty()) {
    bssl::UniquePtr<SSL_ECH_KEYS> keys(SSL_ECH_KEYS_new());
    if (!keys) {
      return nullptr;
    }
    for (size_t i = 0; i < ech_server_configs.size(); i++) {
      const std::string &ech_config = ech_server_configs[i];
      const std::string &ech_private_key = ech_server_keys[i];
      const int is_retry_config = ech_is_retry_config[i];
      bssl::ScopedEVP_HPKE_KEY key;
      if (!EVP_HPKE_KEY_init(
              key.get(), EVP_hpke_x25519_hkdf_sha256(),
              reinterpret_cast<const uint8_t *>(ech_private_key.data()),
              ech_private_key.size()) ||
          !SSL_ECH_KEYS_add(
              keys.get(), is_retry_config,
              reinterpret_cast<const uint8_t *>(ech_config.data()),
              ech_config.size(), key.get())) {
        return nullptr;
      }
    }
    if (!SSL_CTX_set1_ech_keys(ssl_ctx, keys.get())) {
      return nullptr;
    }
  }
  if (!send_channel_id.empty()) {
    bssl::UniquePtr<EVP_PKEY> pkey = LoadPrivateKey(send_channel_id);
    if (!pkey || !SSL_set1_tls_channel_id(ssl.get(), pkey.get())) {
      return nullptr;
    }
  }
  if (!host_name.empty() &&
      !SSL_set_tlsext_host_name(ssl.get(), host_name.c_str())) {
    return nullptr;
  }
  if (!advertise_alpn.empty() &&
      SSL_set_alpn_protos(
          ssl.get(), reinterpret_cast<const uint8_t *>(advertise_alpn.data()),
          advertise_alpn.size()) != 0) {
    return nullptr;
  }
  if (!defer_alps) {
    for (const auto &pair : application_settings) {
      if (!SSL_add_application_settings(
              ssl.get(), reinterpret_cast<const uint8_t *>(pair.first.data()),
              pair.first.size(),
              reinterpret_cast<const uint8_t *>(pair.second.data()),
              pair.second.size())) {
        return nullptr;
      }
    }
  }
  if (!psk.empty()) {
    SSL_set_psk_client_callback(ssl.get(), PskClientCallback);
    SSL_set_psk_server_callback(ssl.get(), PskServerCallback);
  }
  if (!psk_identity.empty() &&
      !SSL_use_psk_identity_hint(ssl.get(), psk_identity.c_str())) {
    return nullptr;
  }
  if (!srtp_profiles.empty() &&
      !SSL_set_srtp_profiles(ssl.get(), srtp_profiles.c_str())) {
    return nullptr;
  }
  if (enable_ocsp_stapling) {
    SSL_enable_ocsp_stapling(ssl.get());
  }
  if (enable_signed_cert_timestamps) {
    SSL_enable_signed_cert_timestamps(ssl.get());
  }
  if (min_version != 0 &&
      !SSL_set_min_proto_version(ssl.get(), min_version)) {
    return nullptr;
  }
  if (max_version != 0 &&
      !SSL_set_max_proto_version(ssl.get(), max_version)) {
    return nullptr;
  }
  if (mtu != 0) {
    SSL_set_options(ssl.get(), SSL_OP_NO_QUERY_MTU);
    SSL_set_mtu(ssl.get(), mtu);
  }
  if (install_ddos_callback) {
    SSL_CTX_set_dos_protection_cb(ssl_ctx, DDoSCallback);
  }
  SSL_set_shed_handshake_config(ssl.get(), true);
  if (renegotiate_once) {
    SSL_set_renegotiate_mode(ssl.get(), ssl_renegotiate_once);
  }
  if (renegotiate_freely || forbid_renegotiation_after_handshake) {
    // |forbid_renegotiation_after_handshake| will disable renegotiation later.
    SSL_set_renegotiate_mode(ssl.get(), ssl_renegotiate_freely);
  }
  if (renegotiate_ignore) {
    SSL_set_renegotiate_mode(ssl.get(), ssl_renegotiate_ignore);
  }
  if (renegotiate_explicit) {
    SSL_set_renegotiate_mode(ssl.get(), ssl_renegotiate_explicit);
  }
  if (!check_close_notify) {
    SSL_set_quiet_shutdown(ssl.get(), 1);
  }
  if (!curves.empty()) {
    std::vector<int> nids;
    for (auto curve : curves) {
      switch (curve) {
        case SSL_GROUP_SECP224R1:
          nids.push_back(NID_secp224r1);
          break;

        case SSL_GROUP_SECP256R1:
          nids.push_back(NID_X9_62_prime256v1);
          break;

        case SSL_GROUP_SECP384R1:
          nids.push_back(NID_secp384r1);
          break;

        case SSL_GROUP_SECP521R1:
          nids.push_back(NID_secp521r1);
          break;

        case SSL_GROUP_X25519:
          nids.push_back(NID_X25519);
          break;
      }
      if (!SSL_set1_curves(ssl.get(), &nids[0], nids.size())) {
        return nullptr;
      }
    }
  }
  if (initial_timeout_duration_ms > 0) {
    DTLSv1_set_initial_timeout_duration(ssl.get(), initial_timeout_duration_ms);
  }
  if (max_cert_list > 0) {
    SSL_set_max_cert_list(ssl.get(), max_cert_list);
  }
  if (retain_only_sha256_client_cert) {
    SSL_set_retain_only_sha256_of_client_certs(ssl.get(), 1);
  }
  if (max_send_fragment > 0) {
    SSL_set_max_send_fragment(ssl.get(), max_send_fragment);
  }
  if (alps_use_new_codepoint) {
    SSL_set_alps_use_new_codepoint(ssl.get(), 1);
  }
  if (quic_use_legacy_codepoint != -1) {
    SSL_set_quic_use_legacy_codepoint(ssl.get(), quic_use_legacy_codepoint);
  }
  if (!quic_transport_params.empty()) {
    if (!SSL_set_quic_transport_params(
            ssl.get(),
            reinterpret_cast<const uint8_t *>(quic_transport_params.data()),
            quic_transport_params.size())) {
      return nullptr;
    }
  }
  if (jdk11_workaround) {
    SSL_set_jdk11_workaround(ssl.get(), 1);
  }

  if (session != NULL) {
    if (!is_server) {
      if (SSL_set_session(ssl.get(), session) != 1) {
        return nullptr;
      }
    } else if (async) {
      // The internal session cache is disabled, so install the session
      // manually.
      SSL_SESSION_up_ref(session);
      GetTestState(ssl.get())->pending_session.reset(session);
    }
  }

  if (!delegated_credential.empty()) {
    std::string::size_type comma = delegated_credential.find(',');
    if (comma == std::string::npos) {
      fprintf(stderr,
              "failed to find comma in delegated credential argument.\n");
      return nullptr;
    }

    const std::string dc_hex = delegated_credential.substr(0, comma);
    const std::string pkcs8_hex = delegated_credential.substr(comma + 1);
    std::string dc, pkcs8;
    if (!HexDecode(&dc, dc_hex) || !HexDecode(&pkcs8, pkcs8_hex)) {
      fprintf(stderr, "failed to hex decode delegated credential argument.\n");
      return nullptr;
    }

    CBS dc_cbs(bssl::Span<const uint8_t>(
        reinterpret_cast<const uint8_t *>(dc.data()), dc.size()));
    CBS pkcs8_cbs(bssl::Span<const uint8_t>(
        reinterpret_cast<const uint8_t *>(pkcs8.data()), pkcs8.size()));

    bssl::UniquePtr<EVP_PKEY> priv(EVP_parse_private_key(&pkcs8_cbs));
    if (!priv) {
      fprintf(stderr, "failed to parse delegated credential private key.\n");
      return nullptr;
    }

    bssl::UniquePtr<CRYPTO_BUFFER> dc_buf(
        CRYPTO_BUFFER_new_from_CBS(&dc_cbs, nullptr));
    if (!SSL_set1_delegated_credential(ssl.get(), dc_buf.get(),
                                      priv.get(), nullptr)) {
      fprintf(stderr, "SSL_set1_delegated_credential failed.\n");
      return nullptr;
    }
  }

  if (!quic_early_data_context.empty() &&
      !SSL_set_quic_early_data_context(
          ssl.get(),
          reinterpret_cast<const uint8_t *>(quic_early_data_context.data()),
          quic_early_data_context.size())) {
    return nullptr;
  }

  return ssl;
}
