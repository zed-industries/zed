// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef HEADER_TEST_CONFIG
#define HEADER_TEST_CONFIG

#include <string>
#include <utility>
#include <vector>

#include <openssl/base.h>
#include <openssl/x509.h>

#include "test_state.h"

struct TestConfig {
  int port = 0;
  bool ipv6 = false;
  uint64_t shim_id = 0;
  bool is_server = false;
  bool is_dtls = false;
  bool is_quic = false;
  int resume_count = 0;
  std::string write_settings;
  bool fallback_scsv = false;
  std::vector<uint16_t> signing_prefs;
  std::vector<uint16_t> verify_prefs;
  std::vector<uint16_t> expect_peer_verify_prefs;
  std::vector<int> curves;
  std::string key_file;
  std::string cert_file;
  std::string trust_cert;
  std::string expect_server_name;
  bool enable_ech_grease = false;
  std::vector<std::string> ech_server_configs;
  std::vector<std::string> ech_server_keys;
  std::vector<int> ech_is_retry_config;
  bool expect_ech_accept = false;
  std::string expect_ech_name_override;
  bool expect_no_ech_name_override = false;
  std::string expect_ech_retry_configs;
  bool expect_no_ech_retry_configs = false;
  std::string ech_config_list;
  std::string expect_certificate_types;
  bool require_any_client_certificate = false;
  std::string advertise_npn;
  std::string expect_next_proto;
  bool false_start = false;
  std::string select_next_proto;
  bool async = false;
  bool write_different_record_sizes = false;
  bool cbc_record_splitting = false;
  bool partial_write = false;
  bool no_tls13 = false;
  bool no_tls12 = false;
  bool no_tls11 = false;
  bool no_tls1 = false;
  bool no_ticket = false;
  std::string expect_channel_id;
  bool enable_channel_id = false;
  std::string send_channel_id;
  bool shim_writes_first = false;
  std::string host_name;
  std::string advertise_alpn;
  std::string expect_alpn;
  std::string expect_advertised_alpn;
  std::string select_alpn;
  bool decline_alpn = false;
  bool reject_alpn = false;
  bool select_empty_alpn = false;
  bool defer_alps = false;
  std::vector<std::pair<std::string, std::string>> application_settings;
  std::unique_ptr<std::string> expect_peer_application_settings;
  bool alps_use_new_codepoint = false;
  std::string quic_transport_params;
  std::string expect_quic_transport_params;
  // Set quic_use_legacy_codepoint to 0 or 1 to configure, -1 uses default.
  int quic_use_legacy_codepoint = -1;
  bool expect_session_miss = false;
  bool expect_extended_master_secret = false;
  std::string psk;
  std::string psk_identity;
  std::string srtp_profiles;
  bool enable_ocsp_stapling = false;
  std::string expect_ocsp_response;
  bool enable_signed_cert_timestamps = false;
  std::string expect_signed_cert_timestamps;
  uint16_t min_version = 0;
  uint16_t max_version = 0;
  uint16_t expect_version = 0;
  int mtu = 0;
  bool implicit_handshake = false;
  bool use_early_callback = false;
  bool fail_early_callback = false;
  bool install_ddos_callback = false;
  bool fail_ddos_callback = false;
  bool fail_cert_callback = false;
  std::string cipher;
  bool handshake_never_done = false;
  int export_keying_material = 0;
  std::string export_label;
  std::string export_context;
  bool use_export_context = false;
  bool tls_unique = false;
  bool expect_ticket_renewal = false;
  bool expect_no_session = false;
  bool expect_ticket_supports_early_data = false;
  bool expect_accept_early_data = false;
  bool expect_reject_early_data = false;
  bool expect_no_offer_early_data = false;
  bool use_ticket_callback = false;
  bool renew_ticket = false;
  bool enable_early_data = false;
  bool enable_client_custom_extension = false;
  bool enable_server_custom_extension = false;
  bool custom_extension_skip = false;
  bool custom_extension_fail_add = false;
  std::string ocsp_response;
  bool check_close_notify = false;
  bool shim_shuts_down = false;
  bool verify_fail = false;
  bool verify_peer = false;
  bool verify_peer_if_no_obc = false;
  bool expect_verify_result = false;
  std::string signed_cert_timestamps;
  int expect_total_renegotiations = 0;
  bool renegotiate_once = false;
  bool renegotiate_freely = false;
  bool renegotiate_ignore = false;
  bool renegotiate_explicit = false;
  bool forbid_renegotiation_after_handshake = false;
  uint16_t expect_peer_signature_algorithm = 0;
  uint16_t expect_curve_id = 0;
  bool use_old_client_cert_callback = false;
  int initial_timeout_duration_ms = 0;
  std::string use_client_ca_list;
  std::string expect_client_ca_list;
  bool send_alert = false;
  bool peek_then_read = false;
  bool enable_grease = false;
  bool permute_extensions = false;
  int max_cert_list = 0;
  std::string ticket_key;
  bool use_exporter_between_reads = false;
  uint16_t expect_cipher_aes = 0;
  uint16_t expect_cipher_no_aes = 0;
  uint16_t expect_cipher = 0;
  std::string expect_peer_cert_file;
  int resumption_delay = 0;
  bool retain_only_sha256_client_cert = false;
  bool expect_sha256_client_cert = false;
  bool read_with_unfinished_write = false;
  bool expect_secure_renegotiation = false;
  bool expect_no_secure_renegotiation = false;
  int max_send_fragment = 0;
  int read_size = 0;
  bool expect_session_id = false;
  bool expect_no_session_id = false;
  int expect_ticket_age_skew = 0;
  bool no_op_extra_handshake = false;
  bool handshake_twice = false;
  bool allow_unknown_alpn_protos = false;
  bool use_custom_verify_callback = false;
  std::string expect_msg_callback;
  bool allow_false_start_without_alpn = false;
  bool handoff = false;
  bool handshake_hints = false;
  bool allow_hint_mismatch = false;
  bool use_ocsp_callback = false;
  bool set_ocsp_in_callback = false;
  bool decline_ocsp_callback = false;
  bool fail_ocsp_callback = false;
  bool install_cert_compression_algs = false;
  int install_one_cert_compression_alg = 0;
  bool reverify_on_resume = false;
  bool enforce_rsa_key_usage = false;
  bool expect_key_usage_invalid = false;
  bool is_handshaker_supported = false;
  bool handshaker_resume = false;
  std::string handshaker_path;
  bool jdk11_workaround = false;
  bool server_preference = false;
  bool export_traffic_secrets = false;
  bool key_update = false;
  bool expect_delegated_credential_used = false;
  std::string delegated_credential;
  std::string expect_early_data_reason;
  bool expect_hrr = false;
  bool expect_no_hrr = false;
  bool wait_for_debugger = false;
  std::string quic_early_data_context;
  int early_write_after_message = 0;
  // When check_ssl_transfer is true, bssl checks if the ssl can be transferred.
  bool check_ssl_transfer = false;
  // when do_ssl_transfer is false, no transfer will happen.
  // when do_ssl_transfer is true, transfer will happen if the ssl is server.
  bool do_ssl_transfer = false;
  // When not zero this enables read ahead and sets the buffer to this size.
  int read_ahead_buffer_size = 0;
  // When not empty, this prefix with random suffix is used to create a file
  // stores the output of |SSL_to_bytes|.
  std::string ssl_fuzz_seed_path_prefix;
  // When not empty, the value is passed to |SSL_CTX_set_ciphersuites|.
  std::string tls13_ciphersuites;
  // multiple_certs_slot is used to associate the server with the multiple
  // certificate/private key slot configuration. The certificate comes first,
  // then the private key.
  // When |multiple_certs_slot| is defined, the certificates defined are
  // prioritized over certs defined with |cert_file| and |key_file|.
  std::vector<std::pair<std::string, std::string>> multiple_certs_slot;
  bool no_check_client_certificate_type = false;

  std::vector<const char*> handshaker_args;

  bssl::UniquePtr<SSL_CTX> SetupCtx(SSL_CTX *old_ctx) const;

  bssl::UniquePtr<SSL> NewSSL(SSL_CTX *ssl_ctx, SSL_SESSION *session,
                              std::unique_ptr<TestState> test_state) const;
};

bool ParseConfig(int argc, char **argv, bool is_shim, TestConfig *out_initial,
                 TestConfig *out_resume, TestConfig *out_retry);

bool SetTestConfig(SSL *ssl, const TestConfig *config);

const TestConfig *GetTestConfig(const SSL *ssl);

bool LoadCertificate(bssl::UniquePtr<X509> *out_x509,
                     bssl::UniquePtr<STACK_OF(X509)> *out_chain,
                     const std::string &file);

bssl::UniquePtr<EVP_PKEY> LoadPrivateKey(const std::string &file);

#endif  // HEADER_TEST_CONFIG
