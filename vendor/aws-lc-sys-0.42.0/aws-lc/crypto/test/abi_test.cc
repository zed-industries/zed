// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include "abi_test.h"

#include <stdarg.h>
#include <stdio.h>

#include <algorithm>
#include <array>

#include <openssl/mem.h>
#include <openssl/rand.h>
#include <openssl/span.h>

#if defined(OPENSSL_X86_64) && defined(SUPPORTS_ABI_TEST)
#if defined(OPENSSL_LINUX) && defined(BORINGSSL_HAVE_LIBUNWIND)
#define SUPPORTS_UNWIND_TEST
#define UNW_LOCAL_ONLY
#include <errno.h>
#include <fcntl.h>
#include <libunwind.h>
#include <pthread.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#elif defined(OPENSSL_WINDOWS)
#define SUPPORTS_UNWIND_TEST
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <windows.h>
OPENSSL_MSVC_PRAGMA(warning(disable : 4091))
#include <dbghelp.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#endif
#endif  // X86_64 && SUPPORTS_ABI_TEST

// FIPS mode breaks unwind tests. See https://crbug.com/boringssl/289.
#if defined(BORINGSSL_FIPS)
#undef SUPPORTS_UNWIND_TEST
#endif


namespace abi_test {

namespace internal {

static bool g_unwind_tests_enabled = false;

std::string FixVAArgsString(const char *str) {
  std::string ret = str;
  size_t idx = ret.find(',');
  if (idx == std::string::npos) {
    return ret + "()";
  }
  size_t idx2 = idx + 1;
  while (idx2 < ret.size() && ret[idx2] == ' ') {
    idx2++;
  }
  while (idx > 0 && ret[idx - 1] == ' ') {
    idx--;
  }
  return ret.substr(0, idx) + "(" + ret.substr(idx2) + ")";
}

#if defined(SUPPORTS_ABI_TEST)
// ForEachMismatch calls |func| for each register where |a| and |b| differ.
template <typename Func>
static void ForEachMismatch(const CallerState &a, const CallerState &b,
                            const Func &func) {
#define CALLER_STATE_REGISTER(type, name) \
  if (a.name != b.name) {                 \
    func(#name);                          \
  }
  LOOP_CALLER_STATE_REGISTERS()
#undef CALLER_STATE_REGISTER
}
#endif  // SUPPORTS_ABI_TEST

#if defined(SUPPORTS_UNWIND_TEST)
// We test unwind metadata by running the function under test with the trap flag
// set. This results in |SIGTRAP| and |EXCEPTION_SINGLE_STEP| on Linux and
// Windows, respectively. We hande these and verify libunwind or the Windows
// unwind APIs unwind successfully.

// IsAncestorStackFrame returns true if |a_sp| is an ancestor stack frame of
// |b_sp|.
static bool IsAncestorStackFrame(crypto_word_t a_sp, crypto_word_t b_sp) {
#if defined(OPENSSL_X86_64)
  // The stack grows down, so ancestor stack frames have higher addresses.
  return a_sp > b_sp;
#else
#error "unknown architecture"
#endif
}

// Implement some string formatting utilties. Ideally we would use |snprintf|,
// but this is called in a signal handler and |snprintf| is not async-signal-
// safe.

#if !defined(OPENSSL_WINDOWS)
static std::array<char, DECIMAL_SIZE(crypto_word_t) + 1> WordToDecimal(
    crypto_word_t v) {
  std::array<char, DECIMAL_SIZE(crypto_word_t) + 1> ret;
  size_t len = 0;
  do {
    ret[len++] = '0' + v % 10;
    v /= 10;
  } while (v != 0);
  for (size_t i = 0; i < len / 2; i++) {
    std::swap(ret[i], ret[len - 1 - i]);
  }
  ret[len] = '\0';
  return ret;
}
#endif  // !OPENSSL_WINDOWS

static std::array<char, sizeof(crypto_word_t) * 2 + 1> WordToHex(
    crypto_word_t v) {
  static const char kHex[] = "0123456789abcdef";
  std::array<char, sizeof(crypto_word_t) * 2 + 1> ret;
  for (size_t i = sizeof(crypto_word_t) - 1; i < sizeof(crypto_word_t); i--) {
    uint8_t b = v & 0xff;
    v >>= 8;
    ret[i * 2] = kHex[b >> 4];
    ret[i * 2 + 1] = kHex[b & 0xf];
  }
  ret[sizeof(crypto_word_t) * 2] = '\0';
  return ret;
}

static void StrCatSignalSafeImpl(bssl::Span<char> out) {}

template <typename... Args>
static void StrCatSignalSafeImpl(bssl::Span<char> out, const char *str,
                                 Args... args) {
  OPENSSL_strlcat(out.data(), str, out.size());
  StrCatSignalSafeImpl(out, args...);
}

template <typename... Args>
static void StrCatSignalSafe(bssl::Span<char> out, Args... args) {
  if (out.empty()) {
    return;
  }
  out[0] = '\0';
  StrCatSignalSafeImpl(out, args...);
}

template <typename... Args>
[[noreturn]] static void FatalError(Args... args) {
  // We cannot use |snprintf| here because it is not async-signal-safe.
  char buf[512];
  StrCatSignalSafe(buf, args..., "\n");
#if defined(OPENSSL_WINDOWS)
  HANDLE stderr_handle = GetStdHandle(STD_ERROR_HANDLE);
  if (stderr_handle != INVALID_HANDLE_VALUE) {
    DWORD unused;
    WriteFile(stderr_handle, buf, strlen(buf), &unused, nullptr);
  }
#else
  OPENSSL_UNUSED ssize_t unused_ret =
          write(STDERR_FILENO, buf, strlen(buf));
#endif
  abort();
}

class UnwindStatus {
 public:
  UnwindStatus() : err_(nullptr) {}
  explicit UnwindStatus(const char *err) : err_(err) {}

  bool ok() const { return err_ == nullptr; }
  const char *Error() const { return err_; }

 private:
  const char *err_;
};

template<typename T>
class UnwindStatusOr {
 public:
  UnwindStatusOr(UnwindStatus status) : status_(status) {
    assert(!status_.ok());
  }

  UnwindStatusOr(const T &value) : status_(UnwindStatus()), value_(value) {}

  bool ok() const { return status_.ok(); }
  const char *Error() const { return status_.Error(); }

  const T &ValueOrDie(const char *msg = "Unexpected error") const {
    if (!ok()) {
      FatalError(msg, ": ", Error());
    }
    return value_;
  }

 private:
  UnwindStatus status_;
  T value_;
};

// UnwindCursor abstracts between libunwind and Windows unwind APIs. It is
// async-signal-safe.
#if defined(OPENSSL_WINDOWS)
class UnwindCursor {
 public:
  explicit UnwindCursor(const CONTEXT &ctx) : ctx_(ctx) {
    starting_ip_ = ctx_.Rip;
  }

  crypto_word_t starting_ip() const { return starting_ip_; }

  // Step unwinds the cursor by one frame. On success, it returns whether there
  // were more frames to unwind.
  UnwindStatusOr<bool> Step() {
    bool is_top = is_top_;
    is_top_ = false;

    DWORD64 image_base;
    RUNTIME_FUNCTION *entry =
        RtlLookupFunctionEntry(ctx_.Rip, &image_base, nullptr);
    if (entry == nullptr) {
      // This is a leaf function. Leaf functions do not touch stack or
      // callee-saved registers, so they may be unwound by simulating a ret.
      if (!is_top) {
        return UnwindStatus("leaf function found below the top frame");
      }
      memcpy(&ctx_.Rip, reinterpret_cast<const void *>(ctx_.Rsp),
             sizeof(ctx_.Rip));
      ctx_.Rsp += 8;
      return true;
    }

    // This is a frame function. Call into the Windows unwinder.
    void *handler_data;
    DWORD64 establisher_frame;
    RtlVirtualUnwind(UNW_FLAG_NHANDLER, image_base, ctx_.Rip, entry, &ctx_,
                     &handler_data, &establisher_frame, nullptr);
    return ctx_.Rip != 0;
  }

  // GetIP returns the instruction pointer at the current frame.
  UnwindStatusOr<crypto_word_t> GetIP() { return ctx_.Rip; }

  // GetSP returns the stack pointer at the current frame.
  UnwindStatusOr<crypto_word_t> GetSP() { return ctx_.Rsp; }

  // GetCallerState returns the callee-saved registers at the current frame.
  UnwindStatusOr<CallerState> GetCallerState() {
    CallerState state;
    state.rbx = ctx_.Rbx;
    state.rbp = ctx_.Rbp;
    state.rdi = ctx_.Rdi;
    state.rsi = ctx_.Rsi;
    state.r12 = ctx_.R12;
    state.r13 = ctx_.R13;
    state.r14 = ctx_.R14;
    state.r15 = ctx_.R15;
    memcpy(&state.xmm6, &ctx_.Xmm6, sizeof(Reg128));
    memcpy(&state.xmm7, &ctx_.Xmm7, sizeof(Reg128));
    memcpy(&state.xmm8, &ctx_.Xmm8, sizeof(Reg128));
    memcpy(&state.xmm9, &ctx_.Xmm9, sizeof(Reg128));
    memcpy(&state.xmm10, &ctx_.Xmm10, sizeof(Reg128));
    memcpy(&state.xmm11, &ctx_.Xmm11, sizeof(Reg128));
    memcpy(&state.xmm12, &ctx_.Xmm12, sizeof(Reg128));
    memcpy(&state.xmm13, &ctx_.Xmm13, sizeof(Reg128));
    memcpy(&state.xmm14, &ctx_.Xmm14, sizeof(Reg128));
    memcpy(&state.xmm15, &ctx_.Xmm15, sizeof(Reg128));
    return state;
  }

  // ToString returns a human-readable representation of the address the cursor
  // started at.
  const char *ToString() {
    StrCatSignalSafe(starting_ip_buf_, "0x", WordToHex(starting_ip_).data());
    return starting_ip_buf_;
  }

 private:
  CONTEXT ctx_;
  crypto_word_t starting_ip_;
  char starting_ip_buf_[64];
  bool is_top_ = true;
};
#else  // !OPENSSL_WINDOWS
class UnwindCursor {
 public:
  explicit UnwindCursor(unw_context_t *ctx) : ctx_(ctx) {
    int ret = InitAtSignalFrame(&cursor_);
    if (ret < 0) {
      FatalError("Error getting unwind context: ", unw_strerror(ret));
    }
    starting_ip_ = GetIP().ValueOrDie("Error getting instruction pointer");
  }

  // Step unwinds the cursor by one frame. On success, it returns whether there
  // were more frames to unwind.
  UnwindStatusOr<bool> Step() {
    int ret = unw_step(&cursor_);
    if (ret < 0) {
      return UNWError(ret);
    }
    return ret != 0;
  }

  // GetIP returns the instruction pointer at the current frame.
  UnwindStatusOr<crypto_word_t> GetIP() {
    crypto_word_t ip;
    int ret = GetReg(&ip, UNW_REG_IP);
    if (ret < 0) {
      return UNWError(ret);
    }
    return ip;
  }

  // GetSP returns the stack pointer at the current frame.
  UnwindStatusOr<crypto_word_t> GetSP() {
    crypto_word_t sp;
    int ret = GetReg(&sp, UNW_REG_SP);
    if (ret < 0) {
      return UNWError(ret);
    }
    return sp;
  }

  // GetCallerState returns the callee-saved registers at the current frame.
  UnwindStatusOr<CallerState> GetCallerState() {
    CallerState state;
    int ret = 0;
#if defined(OPENSSL_X86_64)
    ret = ret < 0 ? ret : GetReg(&state.rbx, UNW_X86_64_RBX);
    ret = ret < 0 ? ret : GetReg(&state.rbp, UNW_X86_64_RBP);
    ret = ret < 0 ? ret : GetReg(&state.r12, UNW_X86_64_R12);
    ret = ret < 0 ? ret : GetReg(&state.r13, UNW_X86_64_R13);
    ret = ret < 0 ? ret : GetReg(&state.r14, UNW_X86_64_R14);
    ret = ret < 0 ? ret : GetReg(&state.r15, UNW_X86_64_R15);
#else
#error "unknown architecture"
#endif
    if (ret < 0) {
      return UNWError(ret);
    }
    return state;
  }

  // ToString returns a human-readable representation of the address the cursor
  // started at, using debug information if available.
  const char *ToString() {
    // Use a new cursor. |cursor_| has already been unwound, and
    // |unw_get_proc_name| is slow so we do not sample it unconditionally in the
    // constructor.
    unw_cursor_t cursor;
    unw_word_t off;
    if (InitAtSignalFrame(&cursor) != 0 ||
        unw_get_proc_name(&cursor, starting_ip_buf_, sizeof(starting_ip_buf_),
                          &off) != 0) {
      StrCatSignalSafe(starting_ip_buf_, "0x", WordToHex(starting_ip_).data());
      return starting_ip_buf_;
    }
    size_t len = strlen(starting_ip_buf_);
    // Print the offset in decimal, to match gdb's disassembly output and ease
    // debugging.
    StrCatSignalSafe(bssl::Span<char>(starting_ip_buf_).subspan(len), "+",
                     WordToDecimal(off).data(), " (0x",
                     WordToHex(starting_ip_).data(), ")");
    return starting_ip_buf_;
  }

 private:
  static UnwindStatus UNWError(int ret) {
    assert(ret < 0);
    const char *msg = unw_strerror(ret);
    return UnwindStatus(msg == nullptr ? "unknown error" : msg);
  }

  int InitAtSignalFrame(unw_cursor_t *cursor) {
    // Work around a bug in libunwind which breaks rax and rdx recovery. This
    // breaks functions which temporarily use rax as the CFA register. See
    // https://git.savannah.gnu.org/gitweb/?p=libunwind.git;a=commit;h=819bf51bbd2da462c2ec3401e8ac9153b6e725e3
    OPENSSL_memset(cursor, 0, sizeof(*cursor));
    int ret = unw_init_local(cursor, ctx_);
    if (ret < 0) {
      return ret;
    }
    for (;;) {
      ret = unw_is_signal_frame(cursor);
      if (ret < 0) {
        return ret;
      }
      if (ret != 0) {
        return 0;  // Found the signal frame.
      }
      ret = unw_step(cursor);
      if (ret < 0) {
        return ret;
      }
    }
  }

  int GetReg(crypto_word_t *out, unw_regnum_t reg) {
    unw_word_t val;
    int ret = unw_get_reg(&cursor_, reg, &val);
    if (ret >= 0) {
      static_assert(sizeof(crypto_word_t) == sizeof(unw_word_t),
                    "crypto_word_t and unw_word_t are inconsistent");
      *out = val;
    }
    return ret;
  }

  unw_context_t *ctx_;
  unw_cursor_t cursor_;
  crypto_word_t starting_ip_;
  char starting_ip_buf_[64];
};
#endif  // OPENSSL_WINDOWS

// g_in_trampoline is true if we are in an instrumented |abi_test_trampoline|
// call, in the region that triggers |SIGTRAP|.
static bool g_in_trampoline = false;
// g_unwind_function_done, if |g_in_trampoline| is true, is whether the function
// under test has returned. It is undefined otherwise.
static bool g_unwind_function_done;
// g_trampoline_state, during an unwind-enabled ABI test, is the state the
// function under test must preserve. It is undefined otherwise.
static CallerState g_trampoline_state;
// g_trampoline_sp, if |g_in_trampoline| is true, is the stack pointer of the
// trampoline frame. It is undefined otherwise.
static crypto_word_t g_trampoline_sp;

// kMaxUnwindErrors is the maximum number of unwind errors reported per
// function. If a function's unwind tables are wrong, we are otherwise likely to
// repeat the same error at multiple addresses.
static constexpr size_t kMaxUnwindErrors = 10;

// Errors are saved in a signal handler. We use a static buffer to avoid
// allocation.
static size_t g_num_unwind_errors = 0;

struct UnwindError {
#if defined(OPENSSL_WINDOWS)
  crypto_word_t ip;
#endif
  char str[512];
};

static UnwindError g_unwind_errors[kMaxUnwindErrors];

template <typename... Args>
static void AddUnwindError(UnwindCursor *cursor, Args... args) {
  if (g_num_unwind_errors >= kMaxUnwindErrors) {
    return;
  }
#if defined(OPENSSL_WINDOWS)
  // Windows symbol functions should not be called when handling an
  // exception. Stash the instruction pointer, to be symbolized later.
  g_unwind_errors[g_num_unwind_errors].ip = cursor->starting_ip();
  StrCatSignalSafe(g_unwind_errors[g_num_unwind_errors].str, args...);
#else
  StrCatSignalSafe(g_unwind_errors[g_num_unwind_errors].str,
                   "unwinding at ", cursor->ToString(), ": ", args...);
#endif
  g_num_unwind_errors++;
}

static void CheckUnwind(UnwindCursor *cursor) {
  const crypto_word_t kStartAddress =
      reinterpret_cast<crypto_word_t>(&abi_test_unwind_start);
  const crypto_word_t kReturnAddress =
      reinterpret_cast<crypto_word_t>(&abi_test_unwind_return);
  const crypto_word_t kStopAddress =
      reinterpret_cast<crypto_word_t>(&abi_test_unwind_stop);

  crypto_word_t sp = cursor->GetSP().ValueOrDie("Error getting stack pointer");
  crypto_word_t ip =
      cursor->GetIP().ValueOrDie("Error getting instruction pointer");
  if (!g_in_trampoline) {
    if (ip != kStartAddress) {
      FatalError("Unexpected SIGTRAP at ", cursor->ToString());
    }

    // Save the current state and begin.
    g_in_trampoline = true;
    g_unwind_function_done = false;
    g_trampoline_sp = sp;
  } else {
    if (sp == g_trampoline_sp || g_unwind_function_done) {
      // |g_unwind_function_done| should imply |sp| is |g_trampoline_sp|, but
      // clearing the trap flag in x86 briefly displaces the stack pointer.
      //
      // Also note we check both |ip| and |sp| below, in case the function under
      // test is also |abi_test_trampoline|.
      if (ip == kReturnAddress && sp == g_trampoline_sp) {
        g_unwind_function_done = true;
      }
      if (ip == kStopAddress && sp == g_trampoline_sp) {
        // |SIGTRAP| is fatal again.
        g_in_trampoline = false;
      }
    } else if (IsAncestorStackFrame(sp, g_trampoline_sp)) {
      // This should never happen. We went past |g_trampoline_sp| without
      // stopping at |kStopAddress|.
      AddUnwindError(cursor, "stack frame is before caller");
      g_in_trampoline = false;
    } else if (g_num_unwind_errors < kMaxUnwindErrors) {
      for (;;) {
        UnwindStatusOr<bool> step_ret = cursor->Step();
        if (!step_ret.ok()) {
          AddUnwindError(cursor, "error unwinding: ", step_ret.Error());
          break;
        }
        // |Step| returns whether there was a frame to unwind.
        if (!step_ret.ValueOrDie()) {
          AddUnwindError(cursor, "could not unwind to starting frame");
          break;
        }

        UnwindStatusOr<crypto_word_t> cur_sp = cursor->GetSP();
        if (!cur_sp.ok()) {
          AddUnwindError(cursor,
                         "error recovering stack pointer: ", cur_sp.Error());
          break;
        }
        if (IsAncestorStackFrame(cur_sp.ValueOrDie(), g_trampoline_sp)) {
          AddUnwindError(cursor, "unwound past starting frame");
          break;
        }
        if (cur_sp.ValueOrDie() == g_trampoline_sp) {
          // We found the parent frame. Check the return address.
          UnwindStatusOr<crypto_word_t> cur_ip = cursor->GetIP();
          if (!cur_ip.ok()) {
            AddUnwindError(cursor,
                           "error recovering return address: ", cur_ip.Error());
          } else if (cur_ip.ValueOrDie() != kReturnAddress) {
            AddUnwindError(cursor, "wrong return address");
          }

          // Check the remaining registers.
          UnwindStatusOr<CallerState> state = cursor->GetCallerState();
          if (!state.ok()) {
            AddUnwindError(cursor,
                           "error recovering registers: ", state.Error());
          } else {
            ForEachMismatch(state.ValueOrDie(), g_trampoline_state,
                            [&](const char *reg) {
                              AddUnwindError(cursor, reg, " was not recovered");
                            });
          }
          break;
        }
      }
    }
  }
}

// ReadUnwindResult adds the results of the most recent unwind test to |out|.
static void ReadUnwindResult(Result *out) {
  for (size_t i = 0; i < g_num_unwind_errors; i++) {
#if defined(OPENSSL_WINDOWS)
    const crypto_word_t ip = g_unwind_errors[i].ip;
    char buf[256];
    DWORD64 displacement;
    struct {
      SYMBOL_INFO info;
      char name_buf[128];
    } symbol;
    memset(&symbol, 0, sizeof(symbol));
    symbol.info.SizeOfStruct = sizeof(symbol.info);
    symbol.info.MaxNameLen = sizeof(symbol.name_buf);
    if (SymFromAddr(GetCurrentProcess(), ip, &displacement, &symbol.info)) {
      snprintf(buf, sizeof(buf), "unwinding at %s+%llu (0x%s): %s",
               symbol.info.Name, displacement, WordToHex(ip).data(),
               g_unwind_errors[i].str);
    } else {
      snprintf(buf, sizeof(buf), "unwinding at 0x%s: %s",
               WordToHex(ip).data(), g_unwind_errors[i].str);
    }
    out->errors.emplace_back(buf);
#else
    out->errors.emplace_back(g_unwind_errors[i].str);
#endif
  }
  if (g_num_unwind_errors == kMaxUnwindErrors) {
    out->errors.emplace_back("(additional errors omitted)");
  }
  g_num_unwind_errors = 0;
}

#if defined(OPENSSL_WINDOWS)
static DWORD g_main_thread;

static long ExceptionHandler(EXCEPTION_POINTERS *info) {
  if (info->ExceptionRecord->ExceptionCode != EXCEPTION_SINGLE_STEP ||
      GetCurrentThreadId() != g_main_thread) {
    return EXCEPTION_CONTINUE_SEARCH;
  }

  UnwindCursor cursor(*info->ContextRecord);
  CheckUnwind(&cursor);
  if (g_in_trampoline) {
    // Windows clears the trap flag, so we must restore it.
    info->ContextRecord->EFlags |= 0x100;
  }
  return EXCEPTION_CONTINUE_EXECUTION;
}

static void EnableUnwindTestsImpl() {
  if (IsDebuggerPresent()) {
    // Unwind tests drive logic via |EXCEPTION_SINGLE_STEP|, which conflicts with
    // debuggers.
    fprintf(stderr, "Debugger detected. Disabling unwind tests.\n");
    return;
  }

  g_main_thread = GetCurrentThreadId();

  SymSetOptions(SYMOPT_DEFERRED_LOADS);
  if (!SymInitialize(GetCurrentProcess(), nullptr, TRUE)) {
    fprintf(stderr, "Could not initialize symbols.\n");
  }

  if (AddVectoredExceptionHandler(0, ExceptionHandler) == nullptr) {
    fprintf(stderr, "Error installing exception handler.\n");
    abort();
  }

  g_unwind_tests_enabled = true;
}
#else  // !OPENSSL_WINDOWS
// HandleEINTR runs |func| and returns the result, retrying the operation on
// |EINTR|.
template <typename Func>
static auto HandleEINTR(const Func &func) -> decltype(func()) {
  decltype(func()) ret;
  do {
    ret = func();
  } while (ret < 0 && errno == EINTR);
  return ret;
}

static bool ReadFileToString(std::string *out, const char *path) {
  out->clear();

  int fd = HandleEINTR([&] { return open(path, O_RDONLY); });
  if (fd < 0) {
    return false;
  }

  for (;;) {
    char buf[1024];
    ssize_t ret = HandleEINTR([&] { return read(fd, buf, sizeof(buf)); });
    if (ret < 0) {
      close(fd);
      return false;
    }
    if (ret == 0) {
      close(fd);
      return true;
    }
    out->append(buf, static_cast<size_t>(ret));
  }
}

static bool IsBeingDebugged() {
  std::string status;
  if (!ReadFileToString(&status, "/proc/self/status")) {
    perror("error reading /proc/self/status");
    return false;
  }
  std::string key = "\nTracerPid:\t";
  size_t idx = status.find(key);
  if (idx == std::string::npos) {
    return false;
  }
  idx += key.size();
  return idx < status.size() && status[idx] != '0';
}

static pthread_t g_main_thread;

static void TrapHandler(int sig) {
  // Note this is a signal handler, so only async-signal-safe functions may be
  // used here. See signal-safety(7). libunwind promises local unwind is
  // async-signal-safe.

  // |pthread_equal| is not listed as async-signal-safe, but this is clearly an
  // oversight.
  if (!pthread_equal(g_main_thread, pthread_self())) {
    FatalError("SIGTRAP on background thread");
  }

  unw_context_t ctx;
  int ret = unw_getcontext(&ctx);
  if (ret < 0) {
    FatalError("Error getting unwind context: ", unw_strerror(ret));
  }

  UnwindCursor cursor(&ctx);
  CheckUnwind(&cursor);
}

static void EnableUnwindTestsImpl() {
  if (IsBeingDebugged()) {
    // Unwind tests drive logic via |SIGTRAP|, which conflicts with debuggers.
    fprintf(stderr, "Debugger detected. Disabling unwind tests.\n");
    return;
  }

  g_main_thread = pthread_self();

  struct sigaction trap_action;
  OPENSSL_memset(&trap_action, 0, sizeof(trap_action));
  sigemptyset(&trap_action.sa_mask);
  trap_action.sa_handler = TrapHandler;
  if (sigaction(SIGTRAP, &trap_action, NULL) != 0) {
    perror("sigaction");
    abort();
  }

  g_unwind_tests_enabled = true;
}
#endif  // OPENSSL_WINDOWS

#else  // !SUPPORTS_UNWIND_TEST

#if defined(SUPPORTS_ABI_TEST)
static void ReadUnwindResult(Result *) {}
#endif
static void EnableUnwindTestsImpl() {}

#endif  // SUPPORTS_UNWIND_TEST

#if defined(SUPPORTS_ABI_TEST)
crypto_word_t RunTrampoline(Result *out, crypto_word_t func,
                            const crypto_word_t *argv, size_t argc,
                            bool unwind) {
  CallerState state;
  RAND_bytes(reinterpret_cast<uint8_t *>(&state), sizeof(state));

  unwind &= g_unwind_tests_enabled;
#if defined(SUPPORTS_UNWIND_TEST)
  if (unwind) {
    // Save the caller state for the unwind tester to check for.
    g_trampoline_state = state;
  }
#endif
  CallerState state2 = state;
  crypto_word_t ret = abi_test_trampoline(func, &state2, argv, argc, unwind);
#if defined(OPENSSL_X86_64) || defined(OPENSSL_X86)
  // Query and clear the direction flag early, so negative tests do not
  // interfere with |malloc|.
  bool direction_flag = abi_test_get_and_clear_direction_flag();
#endif  // OPENSSL_X86_64 || OPENSSL_X86

  *out = Result();
  ForEachMismatch(state, state2, [&](const char *reg) {
    out->errors.push_back(std::string(reg) + " was not restored after return");
  });
#if defined(OPENSSL_X86_64) || defined(OPENSSL_X86)
  // Linux and Windows ABIs for x86 require the direction flag be cleared on
  // return. (Some OpenSSL assembly preserves it, which is stronger, but we only
  // require what is specified by the ABI so |CHECK_ABI| works with C compiler
  // output.)
  if (direction_flag) {
    out->errors.emplace_back("Direction flag set after return");
  }
#endif  // OPENSSL_X86_64 || OPENSSL_X86
  if (unwind) {
    ReadUnwindResult(out);
  }
  return ret;
}
#endif  // SUPPORTS_ABI_TEST

}  // namespace internal

void EnableUnwindTests() { internal::EnableUnwindTestsImpl(); }

bool UnwindTestsEnabled() { return internal::g_unwind_tests_enabled; }

}  // namespace abi_test
