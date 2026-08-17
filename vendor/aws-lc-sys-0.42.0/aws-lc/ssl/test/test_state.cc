// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include "test_state.h"

#include <openssl/ssl.h>

#include "../../crypto/internal.h"
#include "../internal.h"

using namespace bssl;

static CRYPTO_once_t g_once = CRYPTO_ONCE_INIT;
static int g_state_index = 0;
// Some code treats the zero time special, so initialize the clock to a
// non-zero time.
static timeval g_clock = { 1234, 1234 };

static void TestStateExFree(void *parent, void *ptr, CRYPTO_EX_DATA *ad,
                            int index, long argl, void *argp) {
  delete ((TestState *)ptr);
}

static void init_once() {
  g_state_index = SSL_get_ex_new_index(0, NULL, NULL, NULL, TestStateExFree);
  if (g_state_index < 0) {
    abort();
  }
}

struct timeval *GetClock() {
  CRYPTO_once(&g_once, init_once);
  return &g_clock;
}

void AdvanceClock(unsigned seconds) {
  CRYPTO_once(&g_once, init_once);
  g_clock.tv_sec += seconds;
}

bool SetTestState(SSL *ssl, std::unique_ptr<TestState> state) {
  CRYPTO_once(&g_once, init_once);
  // |SSL_set_ex_data| takes ownership of |state| only on success.
  if (SSL_set_ex_data(ssl, g_state_index, state.get()) == 1) {
    state.release();
    return true;
  }
  return false;
}

TestState *GetTestState(const SSL *ssl) {
  CRYPTO_once(&g_once, init_once);
  return (TestState *)SSL_get_ex_data(ssl, g_state_index);
}

static void ssl_ctx_add_session(SSL_SESSION *session, void *void_param) {
  SSL_CTX *ctx = reinterpret_cast<SSL_CTX *>(void_param);
  UniquePtr<SSL_SESSION> new_session = SSL_SESSION_dup(
      session, SSL_SESSION_INCLUDE_NONAUTH | SSL_SESSION_INCLUDE_TICKET);
  if (new_session != nullptr) {
    SSL_CTX_add_session(ctx, new_session.get());
  }
}

void CopySessions(SSL_CTX *dst, const SSL_CTX *src) {
  lh_SSL_SESSION_doall_arg(src->sessions, ssl_ctx_add_session, dst);
}

static void push_session(SSL_SESSION *session, void *arg) {
  auto s = reinterpret_cast<std::vector<SSL_SESSION *> *>(arg);
  s->push_back(session);
}

bool SerializeContextState(SSL_CTX *ctx, CBB *cbb) {
  CBB out, ctx_sessions, ticket_keys;
  uint8_t keys[48];
  if (!CBB_add_u24_length_prefixed(cbb, &out) ||
      !CBB_add_u16(&out, 0 /* version */) ||
      !SSL_CTX_get_tlsext_ticket_keys(ctx, &keys, sizeof(keys)) ||
      !CBB_add_u8_length_prefixed(&out, &ticket_keys) ||
      !CBB_add_bytes(&ticket_keys, keys, sizeof(keys)) ||
      !CBB_add_asn1(&out, &ctx_sessions, CBS_ASN1_SEQUENCE)) {
    return false;
  }
  std::vector<SSL_SESSION *> sessions;
  lh_SSL_SESSION_doall_arg(ctx->sessions, push_session, &sessions);
  for (const auto &sess : sessions) {
    if (!ssl_session_serialize(sess, &ctx_sessions)) {
      return false;
    }
  }
  return CBB_flush(cbb);
}

bool DeserializeContextState(CBS *cbs, SSL_CTX *ctx) {
  CBS in, sessions, ticket_keys;
  uint16_t version;
  constexpr uint16_t kVersion = 0;
  if (!CBS_get_u24_length_prefixed(cbs, &in) ||
      !CBS_get_u16(&in, &version) ||
      version > kVersion ||
      !CBS_get_u8_length_prefixed(&in, &ticket_keys) ||
      !SSL_CTX_set_tlsext_ticket_keys(ctx, CBS_data(&ticket_keys),
                                      CBS_len(&ticket_keys)) ||
      !CBS_get_asn1(&in, &sessions, CBS_ASN1_SEQUENCE)) {
    return false;
  }
  while (CBS_len(&sessions)) {
    UniquePtr<SSL_SESSION> session =
        SSL_SESSION_parse(&sessions, ctx->x509_method, ctx->pool);
    if (!session) {
      return false;
    }
    SSL_CTX_add_session(ctx, session.get());
  }
  return true;
}

bool TestState::Serialize(CBB *cbb) const {
  CBB out, pending, text;
  if (!CBB_add_u24_length_prefixed(cbb, &out) ||
      !CBB_add_u16(&out, 0 /* version */) ||
      !CBB_add_u24_length_prefixed(&out, &pending) ||
      (pending_session &&
       !ssl_session_serialize(pending_session.get(), &pending)) ||
      !CBB_add_u16_length_prefixed(&out, &text) ||
      !CBB_add_bytes(
          &text, reinterpret_cast<const uint8_t *>(msg_callback_text.data()),
          msg_callback_text.length()) ||
      !CBB_add_asn1_uint64(&out, g_clock.tv_sec) ||
      !CBB_add_asn1_uint64(&out, g_clock.tv_usec) ||
      !CBB_flush(cbb)) {
    return false;
  }
  return true;
}

std::unique_ptr<TestState> TestState::Deserialize(CBS *cbs, SSL_CTX *ctx) {
  CBS in, pending_session, text;
  std::unique_ptr<TestState> out_state(new TestState());
  uint16_t version;
  constexpr uint16_t kVersion = 0;
  uint64_t sec, usec;
  if (!CBS_get_u24_length_prefixed(cbs, &in) ||
      !CBS_get_u16(&in, &version) ||
      version > kVersion ||
      !CBS_get_u24_length_prefixed(&in, &pending_session) ||
      !CBS_get_u16_length_prefixed(&in, &text)) {
    return nullptr;
  }
  if (CBS_len(&pending_session)) {
    out_state->pending_session = SSL_SESSION_parse(
        &pending_session, ctx->x509_method, ctx->pool);
    if (!out_state->pending_session) {
      return nullptr;
    }
  }
  out_state->msg_callback_text = std::string(
      reinterpret_cast<const char *>(CBS_data(&text)), CBS_len(&text));
  // TODO(2020-05-01): Make this unconditional & merge into above.
  if (CBS_len(&in) > 0) {
    if (!CBS_get_asn1_uint64(&in, &sec) ||
        !CBS_get_asn1_uint64(&in, &usec)) {
      return nullptr;
    }
    g_clock.tv_sec = sec;
    g_clock.tv_usec = usec;
  }
  return out_state;
}
