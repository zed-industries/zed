// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include "handshake_util.h"

#include <assert.h>
#if defined(HANDSHAKER_SUPPORTED)
#include <errno.h>
#include <fcntl.h>
#include <spawn.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#endif

#include <functional>
#include <map>
#include <vector>

#include "async_bio.h"
#include "packeted_bio.h"
#include "test_config.h"
#include "test_state.h"

#include <openssl/bytestring.h>
#include <openssl/ssl.h>

using namespace bssl;

bool RetryAsync(SSL *ssl, int ret) {
  const TestConfig *config = GetTestConfig(ssl);
  TestState *test_state = GetTestState(ssl);
  if (ret >= 0) {
    return false;
  }

  int ssl_err = SSL_get_error(ssl, ret);
  if (ssl_err == SSL_ERROR_WANT_RENEGOTIATE && config->renegotiate_explicit) {
    test_state->explicit_renegotiates++;
    return SSL_renegotiate(ssl);
  }

  if (test_state->quic_transport && ssl_err == SSL_ERROR_WANT_READ) {
    return test_state->quic_transport->ReadHandshake();
  }

  if (!config->async) {
    // Only asynchronous tests should trigger other retries.
    return false;
  }

  if (test_state->packeted_bio != nullptr &&
      PacketedBioAdvanceClock(test_state->packeted_bio)) {
    // The DTLS retransmit logic silently ignores write failures. So the test
    // may progress, allow writes through synchronously.
    AsyncBioEnforceWriteQuota(test_state->async_bio, false);
    int timeout_ret = DTLSv1_handle_timeout(ssl);
    AsyncBioEnforceWriteQuota(test_state->async_bio, true);

    if (timeout_ret < 0) {
      fprintf(stderr, "Error retransmitting.\n");
      return false;
    }
    return true;
  }

  // See if we needed to read or write more. If so, allow one byte through on
  // the appropriate end to maximally stress the state machine.
  switch (ssl_err) {
    case SSL_ERROR_WANT_READ:
      AsyncBioAllowRead(test_state->async_bio, 1);
      return true;
    case SSL_ERROR_WANT_WRITE:
      AsyncBioAllowWrite(test_state->async_bio, 1);
      return true;
    case SSL_ERROR_WANT_X509_LOOKUP:
      test_state->cert_ready = true;
      return true;
    case SSL_ERROR_PENDING_SESSION:
      test_state->session = std::move(test_state->pending_session);
      return true;
    case SSL_ERROR_PENDING_CERTIFICATE:
      test_state->early_callback_ready = true;
      return true;
    case SSL_ERROR_WANT_PRIVATE_KEY_OPERATION:
      test_state->private_key_retries++;
      return true;
    case SSL_ERROR_WANT_CERTIFICATE_VERIFY:
      test_state->custom_verify_ready = true;
      return true;
    default:
      return false;
  }
}

int CheckIdempotentError(const char *name, SSL *ssl,
                         std::function<int()> func) {
  int ret = func();
  int ssl_err = SSL_get_error(ssl, ret);
  uint32_t err = ERR_peek_error();
  if (ssl_err == SSL_ERROR_SSL || ssl_err == SSL_ERROR_ZERO_RETURN) {
    int ret2 = func();
    int ssl_err2 = SSL_get_error(ssl, ret2);
    uint32_t err2 = ERR_peek_error();
    if (ret != ret2 || ssl_err != ssl_err2 || err != err2) {
      fprintf(stderr, "Repeating %s did not replay the error.\n", name);
      char buf[256];
      ERR_error_string_n(err, buf, sizeof(buf));
      fprintf(stderr, "Wanted: %d %d %s\n", ret, ssl_err, buf);
      ERR_error_string_n(err2, buf, sizeof(buf));
      fprintf(stderr, "Got:    %d %d %s\n", ret2, ssl_err2, buf);
      // runner treats exit code 90 as always failing. Otherwise, it may
      // accidentally consider the result an expected protocol failure.
      exit(90);
    }
  }
  return ret;
}

#if defined(HANDSHAKER_SUPPORTED)

// MoveBIOs moves the |BIO|s of |src| to |dst|.  It is used for handoff.
static void MoveBIOs(SSL *dest, SSL *src) {
  BIO *rbio = SSL_get_rbio(src);
  BIO_up_ref(rbio);
  SSL_set0_rbio(dest, rbio);

  BIO *wbio = SSL_get_wbio(src);
  BIO_up_ref(wbio);
  SSL_set0_wbio(dest, wbio);

  SSL_set0_rbio(src, nullptr);
  SSL_set0_wbio(src, nullptr);
}

static bool HandoffReady(SSL *ssl, int ret) {
  return ret < 0 && SSL_get_error(ssl, ret) == SSL_ERROR_HANDOFF;
}

static ssize_t read_eintr(int fd, void *out, size_t len) {
  ssize_t ret;
  do {
    ret = read(fd, out, len);
  } while (ret < 0 && errno == EINTR);
  return ret;
}

static ssize_t write_eintr(int fd, const void *in, size_t len) {
  ssize_t ret;
  do {
    ret = write(fd, in, len);
  } while (ret < 0 && errno == EINTR);
  return ret;
}

static ssize_t waitpid_eintr(pid_t pid, int *wstatus, int options) {
  pid_t ret;
  do {
    ret = waitpid(pid, wstatus, options);
  } while (ret < 0 && errno == EINTR);
  return ret;
}

// Proxy relays data between |socket|, which is connected to the client, and the
// handshaker, which is connected to the numerically specified file descriptors,
// until the handshaker returns control.
static bool Proxy(BIO *socket, bool async, int control, int rfd, int wfd) {
  for (;;) {
    fd_set rfds;
    FD_ZERO(&rfds);
    FD_SET(wfd, &rfds);
    FD_SET(control, &rfds);
    int fd_max = wfd > control ? wfd : control;
    if (select(fd_max + 1, &rfds, nullptr, nullptr, nullptr) == -1) {
      perror("select");
      return false;
    }

    char buf[64];
    ssize_t bytes;
    if (FD_ISSET(wfd, &rfds) &&
        (bytes = read_eintr(wfd, buf, sizeof(buf))) > 0) {
      char *b = buf;
      while (bytes) {
        int written = BIO_write(socket, b, bytes);
        if (!written) {
          fprintf(stderr, "BIO_write wrote nothing\n");
          return false;
        }
        if (written < 0) {
          if (async) {
            AsyncBioAllowWrite(socket, 1);
            continue;
          }
          fprintf(stderr, "BIO_write failed\n");
          return false;
        }
        b += written;
        bytes -= written;
      }
      // Flush all pending data from the handshaker to the client before
      // considering control messages.
      continue;
    }

    if (!FD_ISSET(control, &rfds)) {
      continue;
    }

    char msg;
    if (read_eintr(control, &msg, 1) != 1) {
      perror("read");
      return false;
    }
    switch (msg) {
      case kControlMsgDone:
        return true;
      case kControlMsgError:
        return false;
      case kControlMsgWantRead:
        break;
      default:
        fprintf(stderr, "Unknown control message from handshaker: %c\n", msg);
        return false;
    }

    auto proxy_data = [&](uint8_t *out, size_t len) -> bool {
      if (async) {
        AsyncBioAllowRead(socket, len);
      }

      while (len > 0) {
        int bytes_read = BIO_read(socket, out, len);
        if (bytes_read < 1) {
          fprintf(stderr, "BIO_read failed\n");
          return false;
        }

        ssize_t bytes_written = write_eintr(rfd, out, bytes_read);
        if (bytes_written == -1) {
          perror("write");
          return false;
        }
        if (bytes_written != bytes_read) {
          fprintf(stderr, "short write (%zd of %d bytes)\n", bytes_written,
                  bytes_read);
          return false;
        }

        len -= bytes_read;
        out += bytes_read;
      }
      return true;
    };

    // Process one SSL record at a time.  That way, we don't send the handshaker
    // anything it doesn't want to process, e.g. early data.
    uint8_t header[SSL3_RT_HEADER_LENGTH];
    if (!proxy_data(header, sizeof(header))) {
      return false;
    }
    if (header[1] != 3) {
       fprintf(stderr, "bad header\n");
       return false;
    }
    size_t remaining = (header[3] << 8) + header[4];
    while (remaining > 0) {
      uint8_t readbuf[64];
      size_t len = remaining > sizeof(readbuf) ? sizeof(readbuf) : remaining;
      if (!proxy_data(readbuf, len)) {
        return false;
      }
      remaining -= len;
    }

    // The handshaker blocks on the control channel, so we have to signal
    // it that the data have been written.
    msg = kControlMsgWriteCompleted;
    if (write_eintr(control, &msg, 1) != 1) {
      perror("write");
      return false;
    }
  }
}

class ScopedFD {
 public:
  ScopedFD() : fd_(-1) {}
  explicit ScopedFD(int fd) : fd_(fd) {}
  ~ScopedFD() { Reset(); }

  ScopedFD(ScopedFD &&other) { *this = std::move(other); }
  ScopedFD &operator=(ScopedFD &&other) {
    Reset(other.fd_);
    other.fd_ = -1;
    return *this;
  }

  int fd() const { return fd_; }

  void Reset(int fd = -1) {
    if (fd_ >= 0) {
      close(fd_);
    }
    fd_ = fd;
  }

 private:
  int fd_;
};

class ScopedProcess {
 public:
  ScopedProcess() : pid_(-1) {}
  ~ScopedProcess() { Reset(); }

  ScopedProcess(ScopedProcess &&other) { *this = std::move(other); }
  ScopedProcess &operator=(ScopedProcess &&other) {
    Reset(other.pid_);
    other.pid_ = -1;
    return *this;
  }

  pid_t pid() const { return pid_; }

  void Reset(pid_t pid = -1) {
    if (pid_ >= 0) {
      kill(pid_, SIGTERM);
      int unused;
      Wait(&unused);
    }
    pid_ = pid;
  }

  bool Wait(int *out_status) {
    if (pid_ < 0) {
      return false;
    }
    if (waitpid_eintr(pid_, out_status, 0) != pid_) {
      return false;
    }
    pid_ = -1;
    return true;
  }

 private:
  pid_t pid_;
};

class FileActionsDestroyer {
 public:
  explicit FileActionsDestroyer(posix_spawn_file_actions_t *actions)
      : actions_(actions) {}
  ~FileActionsDestroyer() { posix_spawn_file_actions_destroy(actions_); }
  FileActionsDestroyer(const FileActionsDestroyer &) = delete;
  FileActionsDestroyer &operator=(const FileActionsDestroyer &) = delete;

 private:
  posix_spawn_file_actions_t *actions_;
};

// StartHandshaker starts the handshaker process and, on success, returns a
// handle to the process in |*out|. It sets |*out_control| to a control pipe to
// the process. |map_fds| maps from desired fd number in the child process to
// the source fd in the calling process. |close_fds| is the list of additional
// fds to close, which may overlap with |map_fds|. Other than stdin, stdout, and
// stderr, the status of fds not listed in either set is undefined.
static bool StartHandshaker(ScopedProcess *out, ScopedFD *out_control,
                            const TestConfig *config, bool is_resume,
                            std::map<int, int> map_fds,
                            std::vector<int> close_fds) {
  if (config->handshaker_path.empty()) {
    fprintf(stderr, "no -handshaker-path specified\n");
    return false;
  }
  struct stat dummy;
  if (stat(config->handshaker_path.c_str(), &dummy) == -1) {
    perror(config->handshaker_path.c_str());
    return false;
  }

  std::vector<const char *> args;
  args.push_back(config->handshaker_path.c_str());
  static const char kResumeFlag[] = "-handshaker-resume";
  if (is_resume) {
    args.push_back(kResumeFlag);
  }
  // config->handshaker_args omits argv[0].
  for (const char *arg : config->handshaker_args) {
    args.push_back(arg);
  }
  args.push_back(nullptr);

  // A datagram socket guarantees that writes are all-or-nothing.
  int control[2];
  if (socketpair(AF_LOCAL, SOCK_DGRAM, 0, control) != 0) {
    perror("socketpair");
    return false;
  }
  ScopedFD scoped_control0(control[0]), scoped_control1(control[1]);
  close_fds.push_back(control[0]);
  map_fds[kFdControl] = control[1];

  posix_spawn_file_actions_t actions;
  if (posix_spawn_file_actions_init(&actions) != 0) {
    return false;
  }
  FileActionsDestroyer actions_destroyer(&actions);
  for (int fd : close_fds) {
    if (posix_spawn_file_actions_addclose(&actions, fd) != 0) {
      return false;
    }
  }
  if (!map_fds.empty()) {
    int max_fd = STDERR_FILENO;
    for (const auto &pair : map_fds) {
      max_fd = std::max(max_fd, pair.first);
      max_fd = std::max(max_fd, pair.second);
    }
    // |map_fds| may contain cycles, so make a copy of all the source fds.
    // |posix_spawn| can only use |dup2|, not |dup|, so we assume |max_fd| is
    // the last fd we care about inheriting. |temp_fds| maps from fd number in
    // the parent process to a temporary fd number in the child process.
    std::map<int, int> temp_fds;
    int next_fd = max_fd + 1;
    for (const auto &pair : map_fds) {
      if (temp_fds.count(pair.second)) {
        continue;
      }
      temp_fds[pair.second] = next_fd;
      if (posix_spawn_file_actions_adddup2(&actions, pair.second, next_fd) !=
          0 ||
          posix_spawn_file_actions_addclose(&actions, pair.second) != 0) {
        return false;
      }
      next_fd++;
    }
    for (const auto &pair : map_fds) {
      if (posix_spawn_file_actions_adddup2(&actions, temp_fds[pair.second],
                                           pair.first) != 0) {
        return false;
      }
    }
    // Clean up temporary fds.
    for (int fd = max_fd + 1; fd < next_fd; fd++) {
      if (posix_spawn_file_actions_addclose(&actions, fd) != 0) {
        return false;
      }
    }
  }

  fflush(stdout);
  fflush(stderr);

  // MSan doesn't know that |posix_spawn| initializes its output, so initialize
  // it to -1.
  pid_t pid = -1;
  if (posix_spawn(&pid, args[0], &actions, nullptr,
                  const_cast<char *const *>(args.data()), environ) != 0) {
    return false;
  }

  out->Reset(pid);
  *out_control = std::move(scoped_control0);
  return true;
}

// RunHandshaker forks and execs the handshaker binary, handing off |input|,
// and, after proxying some amount of handshake traffic, handing back |out|.
static bool RunHandshaker(BIO *bio, const TestConfig *config, bool is_resume,
                          Span<const uint8_t> input,
                          std::vector<uint8_t> *out) {
  int rfd[2], wfd[2];
  // We use pipes, rather than some other mechanism, for their buffers.  During
  // the handshake, this process acts as a dumb proxy until receiving the
  // handback signal, which arrives asynchronously.  The race condition means
  // that this process could incorrectly proxy post-handshake data from the
  // client to the handshaker.
  //
  // To avoid this, this process never proxies data to the handshaker that the
  // handshaker has not explicitly requested as a result of hitting
  // |SSL_ERROR_WANT_READ|.  Pipes allow the data to sit in a buffer while the
  // two processes synchronize over the |control| channel.
  if (pipe(rfd) != 0) {
    perror("pipe");
    return false;
  }
  ScopedFD rfd0_closer(rfd[0]), rfd1_closer(rfd[1]);

  if (pipe(wfd) != 0) {
    perror("pipe");
    return false;
  }
  ScopedFD wfd0_closer(wfd[0]), wfd1_closer(wfd[1]);

  ScopedProcess handshaker;
  ScopedFD control;
  if (!StartHandshaker(
          &handshaker, &control, config, is_resume,
          {{kFdProxyToHandshaker, rfd[0]}, {kFdHandshakerToProxy, wfd[1]}},
          {rfd[1], wfd[0]})) {
    return false;
  }

  rfd0_closer.Reset();
  wfd1_closer.Reset();

  if (write_eintr(control.fd(), input.data(), input.size()) == -1) {
    perror("write");
    return false;
  }
  bool ok = Proxy(bio, config->async, control.fd(), rfd[1], wfd[0]);
  int wstatus;
  if (!handshaker.Wait(&wstatus)) {
    perror("waitpid");
    return false;
  }
  if (ok && wstatus) {
    fprintf(stderr, "handshaker exited irregularly\n");
    return false;
  }
  if (!ok) {
    return false;  // This is a "good", i.e. expected, error.
  }

  constexpr size_t kBufSize = 1024 * 1024;
  std::vector<uint8_t> buf(kBufSize);
  ssize_t len = read_eintr(control.fd(), buf.data(), buf.size());
  if (len == -1) {
    perror("read");
    return false;
  }
  buf.resize(len);
  *out = std::move(buf);
  return true;
}

static bool RequestHandshakeHint(const TestConfig *config, bool is_resume,
                                 Span<const uint8_t> input, bool *out_has_hints,
                                 std::vector<uint8_t> *out_hints) {
  ScopedProcess handshaker;
  ScopedFD control;
  if (!StartHandshaker(&handshaker, &control, config, is_resume, {}, {})) {
    return false;
  }

  if (write_eintr(control.fd(), input.data(), input.size()) == -1) {
    perror("write");
    return false;
  }

  char msg;
  if (read_eintr(control.fd(), &msg, 1) != 1) {
    perror("read");
    return false;
  }

  switch (msg) {
    case kControlMsgDone: {
      constexpr size_t kBufSize = 1024 * 1024;
      out_hints->resize(kBufSize);
      ssize_t len =
          read_eintr(control.fd(), out_hints->data(), out_hints->size());
      if (len == -1) {
        perror("read");
        return false;
      }
      out_hints->resize(len);
      *out_has_hints = true;
      break;
    }
    case kControlMsgError:
      *out_has_hints = false;
      break;
    default:
      fprintf(stderr, "Unknown control message from handshaker: %c\n", msg);
      return false;
  }

  int wstatus;
  if (!handshaker.Wait(&wstatus)) {
    perror("waitpid");
    return false;
  }
  if (wstatus) {
    fprintf(stderr, "handshaker exited irregularly\n");
    return false;
  }

  return true;
}

// PrepareHandoff accepts the |ClientHello| from |ssl| and serializes state to
// be passed to the handshaker.  The serialized state includes both the SSL
// handoff, as well test-related state.
static bool PrepareHandoff(SSL *ssl, SettingsWriter *writer,
                           std::vector<uint8_t> *out_handoff) {
  SSL_set_handoff_mode(ssl, 1);

  const TestConfig *config = GetTestConfig(ssl);
  int ret = -1;
  do {
    ret = CheckIdempotentError(
        "SSL_do_handshake", ssl,
        [&]() -> int { return SSL_do_handshake(ssl); });
  } while (!HandoffReady(ssl, ret) &&
           config->async &&
           RetryAsync(ssl, ret));
  if (!HandoffReady(ssl, ret)) {
    fprintf(stderr, "Handshake failed while waiting for handoff.\n");
    return false;
  }

  ScopedCBB cbb;
  SSL_CLIENT_HELLO hello;
  if (!CBB_init(cbb.get(), 512) ||
      !SSL_serialize_handoff(ssl, cbb.get(), &hello) ||
      !writer->WriteHandoff({CBB_data(cbb.get()), CBB_len(cbb.get())}) ||
      !SerializeContextState(SSL_get_SSL_CTX(ssl), cbb.get()) ||
      !GetTestState(ssl)->Serialize(cbb.get())) {
    fprintf(stderr, "Handoff serialisation failed.\n");
    return false;
  }
  out_handoff->assign(CBB_data(cbb.get()),
                      CBB_data(cbb.get()) + CBB_len(cbb.get()));
  return true;
}

// DoSplitHandshake delegates the SSL handshake to a separate process, called
// the handshaker.  This process proxies I/O between the handshaker and the
// client, using the |BIO| from |ssl|.  After a successful handshake, |ssl| is
// replaced with a new |SSL| object, in a way that is intended to be invisible
// to the caller.
bool DoSplitHandshake(UniquePtr<SSL> *ssl, SettingsWriter *writer,
                      bool is_resume) {
  assert(SSL_get_rbio(ssl->get()) == SSL_get_wbio(ssl->get()));
  std::vector<uint8_t> handshaker_input;
  const TestConfig *config = GetTestConfig(ssl->get());
  // out is the response from the handshaker, which includes a serialized
  // handback message, but also serialized updates to the |TestState|.
  std::vector<uint8_t> out;
  if (!PrepareHandoff(ssl->get(), writer, &handshaker_input) ||
      !RunHandshaker(SSL_get_rbio(ssl->get()), config, is_resume,
                     handshaker_input, &out)) {
    fprintf(stderr, "Handoff failed.\n");
    return false;
  }

  SSL_CTX *ctx = SSL_get_SSL_CTX(ssl->get());
  UniquePtr<SSL> ssl_handback = config->NewSSL(ctx, nullptr, nullptr);
  if (!ssl_handback) {
    return false;
  }
  CBS output, handback;
  CBS_init(&output, out.data(), out.size());
  if (!CBS_get_u24_length_prefixed(&output, &handback) ||
      !DeserializeContextState(&output, ctx) ||
      !SetTestState(ssl_handback.get(), TestState::Deserialize(&output, ctx)) ||
      !GetTestState(ssl_handback.get()) || !writer->WriteHandback(handback) ||
      !SSL_apply_handback(ssl_handback.get(), handback)) {
    fprintf(stderr, "Handback failed.\n");
    return false;
  }
  MoveBIOs(ssl_handback.get(), ssl->get());
  GetTestState(ssl_handback.get())->async_bio =
      GetTestState(ssl->get())->async_bio;
  GetTestState(ssl->get())->async_bio = nullptr;

  *ssl = std::move(ssl_handback);
  return true;
}

bool GetHandshakeHint(SSL *ssl, SettingsWriter *writer, bool is_resume,
                      const SSL_CLIENT_HELLO *client_hello) {
  ScopedCBB input;
  CBB child;
  if (!CBB_init(input.get(), client_hello->client_hello_len + 256) ||
      !CBB_add_u24_length_prefixed(input.get(), &child) ||
      !CBB_add_bytes(&child, client_hello->client_hello,
                     client_hello->client_hello_len) ||
      !CBB_add_u24_length_prefixed(input.get(), &child) ||
      !SSL_serialize_capabilities(ssl, &child) ||  //
      !CBB_flush(input.get())) {
    return false;
  }

  bool has_hints;
  std::vector<uint8_t> hints;
  if (!RequestHandshakeHint(
          GetTestConfig(ssl), is_resume,
          MakeConstSpan(CBB_data(input.get()), CBB_len(input.get())),
          &has_hints, &hints)) {
    return false;
  }
  if (has_hints &&
      (!writer->WriteHints(hints) ||
       !SSL_set_handshake_hints(ssl, hints.data(), hints.size()))) {
    return false;
  }

  return true;
}

#endif  // defined(HANDSHAKER_SUPPORTED)
