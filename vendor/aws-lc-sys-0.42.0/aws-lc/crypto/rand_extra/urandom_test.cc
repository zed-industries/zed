// Copyright (c) 2019, Google Inc.
// SPDX-License-Identifier: ISC

#include <gtest/gtest.h>
#include <stdlib.h>

#include <openssl/ctrdrbg.h>
#include <openssl/rand.h>

#include "getrandom_fillin.h"
#include "internal.h"
#include "../ube/internal.h"
#include "../ube/vm_ube_detect.h"

#if defined(OPENSSL_RAND_URANDOM) && \
    defined(OPENSSL_X86_64) && \
    !defined(BORINGSSL_SHARED_LIBRARY) && \
    !defined(BORINGSSL_UNSAFE_DETERMINISTIC_MODE) && \
    defined(USE_NR_getrandom) && !defined(AWSLC_VM_UBE_TESTING) && \
    !defined(DISABLE_CPU_JITTER_ENTROPY)

#include <linux/types.h>

#include <linux/random.h>
#include <sys/ptrace.h>
#include <sys/syscall.h>
#include <sys/user.h>

#include "../ube/fork_ube_detect.h"
#include "getrandom_fillin.h"

#include "../test/test_util.h"

#include <cstdlib>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>

#if !defined(PTRACE_O_EXITKILL)
#define PTRACE_O_EXITKILL (1 << 20)
#endif

#if !defined(PTRACE_O_TRACESYSGOOD)
#define PTRACE_O_TRACESYSGOOD (1)
#endif

// This test can be run with $OPENSSL_ia32cap=~0x4000000000000000 in order to
// simulate the absence of RDRAND of machines that have it.

// Event represents a system call from urandom.c that is observed by the ptrace
// code in |GetTrace|.
struct Event {
  enum class Syscall {
    kGetRandom,
    kOpen,
    kUrandomRead,
    kUrandomIoctl,
    kNanoSleep,
    kAbort,
  };

  explicit Event(Syscall syscall) : type(syscall) {}

  bool operator==(const Event &other) const {
    return type == other.type && length == other.length &&
           flags == other.flags && filename == other.filename;
  }

  static Event GetRandom(size_t length, unsigned flags) {
    Event e(Syscall::kGetRandom);
    e.length = length;
    e.flags = flags;
    return e;
  }

  static Event Open(std::string filename) {
    Event e(Syscall::kOpen);
    e.filename = std::move(filename);
    return e;
  }

  static Event UrandomRead(size_t length) {
    Event e(Syscall::kUrandomRead);
    e.length = length;
    return e;
  }

  static Event UrandomIoctl() {
    Event e(Syscall::kUrandomIoctl);
    return e;
  }

  static Event Abort() {
    Event e(Syscall::kAbort);
    return e;
  }

  static Event NanoSleep() {
    Event e(Syscall::kNanoSleep);
    return e;
  }

  std::string String() const {
    char buf[256];

    switch (type) {
      case Syscall::kGetRandom:
        snprintf(buf, sizeof(buf), "getrandom(_, %zu, %u)", length, flags);
        break;

      case Syscall::kOpen:
        snprintf(buf, sizeof(buf), "open(%s, _)", filename.c_str());
        break;

      case Syscall::kUrandomRead:
        snprintf(buf, sizeof(buf), "read(urandom_fd, _, %zu)", length);
        break;

      case Syscall::kNanoSleep:
        return "nanosleep(_)";

      case Syscall::kUrandomIoctl:
        return "ioctl(urandom_fd, RNDGETENTCNT, _)";

      case Syscall::kAbort:
        return "abort()";
    }

    return std::string(buf);
  }

  const Syscall type;
  size_t length = 0;
  unsigned flags = 0;
  std::string filename;
};

static std::string ToString(const std::vector<Event> &trace) {
  std::string ret;
  for (const auto &event : trace) {
    if (!ret.empty()) {
      ret += ", ";
    }
    ret += event.String();
  }
  return ret;
}

// The following are flags to tell |GetTrace| to inject faults, using ptrace,
// into the entropy-related system calls.

// getrandom gives |ENOSYS|.
static const unsigned NO_GETRANDOM = 1;
// opening /dev/urandom fails.
static const unsigned NO_URANDOM = 2;
// getrandom always returns |EAGAIN| if given |GRNG_NONBLOCK|.
static const unsigned GETRANDOM_NOT_READY = 4;
// The ioctl on urandom returns only 255 bits of entropy the first time that
// it's called.
static const unsigned URANDOM_NOT_READY = 8;
// getrandom gives |EINVAL| unless |NO_GETRANDOM| is set.
static const unsigned GETRANDOM_ERROR = 16;
// Reading from /dev/urandom gives |EINVAL|.
static const unsigned URANDOM_ERROR = 32;
static const unsigned NEXT_FLAG = 64;

// ReadString parses string at address |addr| in child process |pid|.
static std::string ReadString(pid_t pid, unsigned long addr) {
  std::string result;
  size_t i = 0;
  while (i < 4096) { // Don't accept paths longer than this.
    long data = ptrace(PTRACE_PEEKDATA, pid, addr + i, NULL);
    if (data == -1 && errno) {
      break;
    }

    char *p = (char*)&data;
    for (size_t j = 0; j < sizeof(long); j++) {
      if (p[j] == '\0') {
        return result;
      }
      result += p[j];
    }
    i += sizeof(long);
  }
  return result;
}

// HasPrefix returns true of |prefix| is a prefix of |str| and false otherwise.
static bool HasPrefix(const std::string& str, const std::string& prefix) {
    return str.length() >= prefix.length() &&
           (str.compare(0, prefix.length(), prefix) == 0);
}

// GetTrace runs |thunk| in a forked process and observes the resulting system
// calls using ptrace. It simulates a variety of failures based on the contents
// of |flags| and records the observed events by appending to |out_trace|.
static void GetTrace(std::vector<Event> *out_trace, unsigned flags,
                     std::function<void()> thunk) {
  const int child_pid = fork();
  ASSERT_NE(-1, child_pid);

  if (child_pid == 0) {
    // Child process
    if (ptrace(PTRACE_TRACEME, 0, 0, 0) != 0) {
      perror("PTRACE_TRACEME");
      _exit(1);
    }
    raise(SIGSTOP);
    thunk();
    _exit(0);
  }

  // Parent process
  int status;
  ASSERT_EQ(child_pid, waitpid(child_pid, &status, 0));
  ASSERT_TRUE(WIFSTOPPED(status) && WSTOPSIG(status) == SIGSTOP)
      << "Child was not stopped with SIGSTOP: " << status;

  // Set options so that:
  //   a) the child process is killed once this process dies.
  //   b) System calls result in a WSTOPSIG value of (SIGTRAP | 0x80) rather
  //      than just SIGTRAP. (This doesn't matter here, but it's recommended
  //      practice so that it's distinct from the signal itself.)
  ASSERT_EQ(0, ptrace(PTRACE_SETOPTIONS, child_pid, nullptr,
                      PTRACE_O_EXITKILL | PTRACE_O_TRACESYSGOOD))
      << strerror(errno);

  // urandom_fd tracks the file descriptor number for /dev/urandom in the child
  // process, if it opens it.
  int urandom_fd = -1;
  bool urandom_not_ready_was_cleared = false;

  for (;;) {
    // Advance the child to the next system call.
    ASSERT_EQ(0, ptrace(PTRACE_SYSCALL, child_pid, 0, 0));
    ASSERT_EQ(child_pid, waitpid(child_pid, &status, 0));

    // The child may have aborted rather than made a system call.
    if (WIFSTOPPED(status) && WSTOPSIG(status) == SIGABRT) {
      out_trace->push_back(Event::Abort());
      break;
    }

    // Otherwise the only valid ptrace event is a system call stop.
    ASSERT_TRUE(WIFSTOPPED(status) && WSTOPSIG(status) == (SIGTRAP | 0x80))
        << "Child was not stopped with a syscall stop: " << status;

    struct user_regs_struct regs;
    ASSERT_EQ(0, ptrace(PTRACE_GETREGS, child_pid, nullptr, &regs));
    const auto syscall_number = regs.orig_rax;
    static auto previous_syscall = regs.orig_rax;

    bool is_opening_urandom = false;
    bool is_urandom_ioctl = false;
    uintptr_t ioctl_output_addr = 0;
    // inject_error is zero to indicate that the system call should run
    // normally. Otherwise it's, e.g. -EINVAL, to indicate that the system call
    // should not run and that error should be injected on return.
    int inject_error = 0;

    switch (syscall_number) {
      case __NR_getrandom:
        if (flags & NO_GETRANDOM) {
          inject_error = -ENOSYS;
        } else if (flags & GETRANDOM_ERROR) {
          inject_error = -EINVAL;
        } else if (flags & GETRANDOM_NOT_READY) {
          if (regs.rdx & GRND_NONBLOCK) {
            inject_error = -EAGAIN;
          }
        }
        out_trace->push_back(
            Event::GetRandom(/*length=*/regs.rsi, /*flags=*/regs.rdx));
        break;

      case __NR_openat:
      case __NR_open: {
        std::string filename = ReadString(child_pid,
            (syscall_number == __NR_openat) ? regs.rsi : regs.rdi);

        // From https://github.com/aws/aws-lc/blob/6c961b6617adb773fd9fb79dd805498e7ecc7a8b/third_party/jitterentropy/jitterentropy-base-user.h#L273
        // We do not model these system calls, because they are part of the
        // internal implementation detail of Jitter Entropy and there is
        // currently no exported method from Jitter Entropy that allow us to
        // continuously predict the behaviour.
        if (HasPrefix(filename, "/sys/devices/system/cpu/cpu0/cache")) {
          break;
        }

        if (filename != CRYPTO_get_sysgenid_path()) {
          out_trace->push_back(Event::Open(filename));
        }

        is_opening_urandom = (filename == "/dev/urandom");
        if (is_opening_urandom && (flags & NO_URANDOM)) {
          inject_error = -ENOENT;
        }
        break;
      }

      case __NR_read: {
        const int read_fd = regs.rdi;
        if (urandom_fd >= 0 && urandom_fd == read_fd) {
          out_trace->push_back(Event::UrandomRead(/*length=*/regs.rdx));
          if (flags & URANDOM_ERROR) {
            inject_error = -EINVAL;
          }
        }
        break;
      }

      case __NR_nanosleep: {
        // If blocking, an |ioctl| call with command |RNDGETENTCNT| is used. If
        // this fails, a delay is injected. The failure happens when the test
        // flag |URANDOM_NOT_READY| is set. But since this bit is cleared below
        // we detect this event using |urandom_not_ready_was_cleared|.
        //
        // Second true condition: We can have two or more consecutive
        // |nanosleep| calls. This happens if |nanosleep| returns -1. The PRNG
        // model only accounts for one |nanosleep| call. Do the same here.
        if (urandom_not_ready_was_cleared ||
            ((flags & URANDOM_ERROR) && (previous_syscall != __NR_nanosleep))) {
          out_trace->push_back(Event::NanoSleep());
        }
        break;
      }

      // Alias for |__NR_nanosleep| on, at least, Ubuntu 20.04.
      case __NR_clock_nanosleep: {
        if (urandom_not_ready_was_cleared ||
            ((flags & URANDOM_ERROR) &&
             (previous_syscall != __NR_clock_nanosleep))) {
          out_trace->push_back(Event::NanoSleep());
        }
        break;
      }

      case __NR_ioctl: {
        const int ioctl_fd = regs.rdi;
        // Apparently, some operating systems sign-extend registers into the
        // register object when read through ptrace. I assume this is when
        // registers are 32-bit, while |struct user_regs_struct| contains all
        // 64-bit type fields. This is, at least, currently the case on Alpine
        // Linux. This works very poorly when checking the RNDGETENTCNT
        // condition below. Chop off the leading 32-bits to have a consistent
        // check over all operating systems that this test supports.
        if (urandom_fd >= 0 && ioctl_fd == urandom_fd &&
            (regs.rsi & 0xFFFFFFFF) == RNDGETENTCNT) {
          out_trace->push_back(Event::UrandomIoctl());
          is_urandom_ioctl = true;
          ioctl_output_addr = regs.rdx;
        }
      }
    }

    previous_syscall = syscall_number;

    if (inject_error) {
      // Replace the system call number with -1 to cause the kernel to ignore
      // the call. The -ENOSYS will be replaced later with the value of
      // |inject_error|.
      regs.orig_rax = -1;
      ASSERT_EQ(0, ptrace(PTRACE_SETREGS, child_pid, nullptr, &regs));
    }

    ASSERT_EQ(0, ptrace(PTRACE_SYSCALL, child_pid, 0, 0));
    ASSERT_EQ(child_pid, waitpid(child_pid, &status, 0));
    // If the system call was exit/exit_group, the process may be terminated
    // rather than have exited the system call.
    if (WIFEXITED(status)) {
      ASSERT_EQ(0, WEXITSTATUS(status));
      return;
    }

    // Otherwise the next state must be a system call exit stop. This is
    // indistinguishable from a system call entry, we just have to keep track
    // and know that these events happen in pairs.
    ASSERT_TRUE(WIFSTOPPED(status) && WSTOPSIG(status) == (SIGTRAP | 0x80));

    if (inject_error) {
      if (inject_error != -ENOSYS) {
        ASSERT_EQ(0, ptrace(PTRACE_GETREGS, child_pid, nullptr, &regs));
        regs.rax = inject_error;
        ASSERT_EQ(0, ptrace(PTRACE_SETREGS, child_pid, nullptr, &regs));
      }
    } else if (is_opening_urandom) {
      ASSERT_EQ(0, ptrace(PTRACE_GETREGS, child_pid, nullptr, &regs));
      urandom_fd = regs.rax;
    } else if (is_urandom_ioctl) {
      // The result is the number of bits of entropy that the kernel currently
      // believes that it has. urandom.c waits until 256 bits are ready.
      int result = 256;

      // If we are simulating urandom not being ready then we have the ioctl
      // indicate one too few bits of entropy the first time it's queried.
      if (flags & URANDOM_NOT_READY) {
        result--;
        flags &= ~URANDOM_NOT_READY;
        urandom_not_ready_was_cleared = true;
      }

      // ptrace always works with ill-defined "words", which appear to be 64-bit
      // on x86-64. Since the ioctl result is a 32-bit int, do a
      // read-modify-write to inject the answer.
      const uintptr_t aligned_addr = ioctl_output_addr & ~7;
      const uintptr_t offset = ioctl_output_addr - aligned_addr;
      union {
        uint64_t word;
        uint8_t bytes[8];
      } u;
      u.word = ptrace(PTRACE_PEEKDATA, child_pid,
                      reinterpret_cast<void *>(aligned_addr), nullptr);
      memcpy(&u.bytes[offset], &result, sizeof(result));
      ASSERT_EQ(0, ptrace(PTRACE_POKEDATA, child_pid,
                          reinterpret_cast<void *>(aligned_addr),
                          reinterpret_cast<void *>(u.word)));
    }
  }
}

// TestFunction is the function that |GetTrace| is asked to trace.
static void TestFunction() {
  uint8_t byte;
  RAND_bytes(&byte, sizeof(byte));
  RAND_bytes(&byte, sizeof(byte));
}

static bool have_ube_detection() {
  uint64_t tmp_gn = 0;
  return CRYPTO_get_ube_generation_number(&tmp_gn) != 0;
}

// TestFunctionPRNGModel is a model of how the urandom.c code will behave when
// |TestFunction| is run. It should return the same trace of events that
// |GetTrace| will observe the real code making.
static std::vector<Event> TestFunctionPRNGModel(unsigned flags) {

  std::vector<Event> ret;
  bool urandom_ready = false;
  bool getrandom_ready = false;

  // Probe for getrandom support
  ret.push_back(Event::GetRandom(1, GRND_NONBLOCK));

  // Define callbacks that model system calls made for each of the random
  // function flavors defined in urandom.c; currently, this is either getrandom
  // or /dev/urandom.|ensure_entropy_pool_is_initialized| models
  // |ensure_entropy_state_is_initd_once| while |sysrand| models either
  // |wrapper_getrandom| or |wrapper_dev_urandom|.
  std::function<void()> ensure_entropy_pool_is_initialized;
  std::function<bool(bool, size_t)> sysrand;

  if (flags & NO_GETRANDOM) {
    ret.push_back(Event::Open(std::string("/dev/urandom")));
    if (flags & NO_URANDOM) {
      ret.push_back(Event::Abort());
      return ret;
    }

    ensure_entropy_pool_is_initialized = [&ret, &urandom_ready, flags] {
      if (urandom_ready) {
        return;
      }

      // Probe urandom for entropy.
      ret.push_back(Event::UrandomIoctl());
      if (flags & URANDOM_NOT_READY) {
        ret.push_back(Event::NanoSleep());
        // If the first attempt doesn't report enough entropy, probe
        // repeatedly until it does, which will happen with the second attempt.
        ret.push_back(Event::UrandomIoctl());
      }

      urandom_ready = true;
    };

    sysrand = [&ret, &ensure_entropy_pool_is_initialized, flags](bool block, size_t len) {
      if (block) {
        ensure_entropy_pool_is_initialized();
      }
      ret.push_back(Event::UrandomRead(len));
      if (flags & URANDOM_ERROR) {
        if (block) {
          for (size_t i = 0; i < MAX_BACKOFF_RETRIES; i++) {
            ret.push_back(Event::NanoSleep());
            ret.push_back(Event::UrandomRead(len));
          }
        }

        ret.push_back(Event::Abort());
        return false;
      }
      return true;
    };
  } else {
    if (flags & GETRANDOM_ERROR) {
      ret.push_back(Event::Abort());
      return ret;
    }

    getrandom_ready = (flags & GETRANDOM_NOT_READY) == 0;
    ensure_entropy_pool_is_initialized = [&ret, &getrandom_ready] {
      if (getrandom_ready) {
        return;
      }

      ret.push_back(Event::GetRandom(1, GRND_NONBLOCK));
      ret.push_back(Event::GetRandom(1, 0));
      getrandom_ready = true;
    };

    sysrand = [&ret, &ensure_entropy_pool_is_initialized](bool block, size_t len) {
      if (block) {
        ensure_entropy_pool_is_initialized();
      }

      ret.push_back(Event::GetRandom(len, block ? 0 : GRND_NONBLOCK));
      return true;
    };
  }

  const size_t kPersonalizationStringLength = CTR_DRBG_ENTROPY_LEN;
  const size_t kPredictionResistanceStringLength = RAND_PRED_RESISTANCE_LEN;
  const bool kHaveUbeDetection = have_ube_detection();

  // We now build the randomness generation model. Only system call events
  // can be captured. To build the model, we reason about the expected workflow
  // for randomness generation and must correctly predict when a specific
  // system call is made. We assume two consecutive RAND_bytes() calls, as
  // specified by the test function TestFunction().
  //
  // First call to RAND_bytes(): Seed the frontend CTR-DRBG using seed source
  // and personalization string source. The seed source is the tree-DRBG and
  // personalization string the operating system source. The tree-DRBG will use
  // Jitter Entropy at its root. The tree-DRBG per-thread CTR-DRBG will use the
  // operating system entropy source for prediction resistance if there is no
  // UBE detection.

  // Capture tree-DRBG per-thread CTR-DRBG maybe using prediction resistance.
  if (!kHaveUbeDetection) {
    if (!sysrand(true, kPredictionResistanceStringLength)) {
      return ret;
    }
  }

  // Seeding of frontend CTR-DRBG will always use a personalization string.
  if (!sysrand(true, kPersonalizationStringLength)) {
    return ret;
  }

  // Second call to RAND_bytes(): If there is no UBE detection, we initiate a
  // reseed before generating any output.
  if (!kHaveUbeDetection) {
    // Again, the tree-DRBG per-thread CTR-DRBG will use prediction resistance
    // if there is no UBE detection.
    if (!kHaveUbeDetection) {
      if (!sysrand(true, kPredictionResistanceStringLength)) {
        return ret;
      }
    }

    // Seeding of frontend CTR-DRBG will always use a personalization string.
    if (!sysrand(true, kPersonalizationStringLength)) {
      return ret;
    }
  }

  return ret;
}

#define SCOPED_TRACE_FLAG(flag)                                  \
  snprintf(buf, sizeof(buf), #flag ": %d", (flags & flag) != 0); \
  SCOPED_TRACE(buf);

// Tests that |TestFunctionPRNGModel| is a correct model for the code in
// urandom.c, at least to the limits of the the |Event| type.
//
// |TestFunctionPRNGModel| creates the entropy function call model, for
// various configs. |GetTrace| records the actual entropy function calls for
// each config and compares it against the model.
// Only system entropy function calls are modeled e.g. /dev/random and getrandom.
TEST(URandomTest, Test) {
  char buf[256];

  // Some Android systems lack getrandom.
  uint8_t scratch[1];
  const bool has_getrandom =
      (syscall(__NR_getrandom, scratch, sizeof(scratch), GRND_NONBLOCK) != -1 ||
       errno != ENOSYS);

  for (unsigned flags = 0; flags < NEXT_FLAG; flags++) {
    if (!has_getrandom && !(flags & NO_GETRANDOM)) {
        continue;
    }

    // Prints test configuration if an error is reported below. Scoped to this
    // iteration of the for-loop.
    SCOPED_TRACE_FLAG(NO_GETRANDOM);
    SCOPED_TRACE_FLAG(NO_URANDOM);
    SCOPED_TRACE_FLAG(GETRANDOM_NOT_READY);
    SCOPED_TRACE_FLAG(URANDOM_NOT_READY);
    SCOPED_TRACE_FLAG(GETRANDOM_ERROR);
    SCOPED_TRACE_FLAG(URANDOM_ERROR);

    // From PRNG model, generate the expected trace of system calls.
    const std::vector<Event> expected_trace = TestFunctionPRNGModel(flags);

    // Generate the real trace of system calls.
    std::vector<Event> actual_trace;
    GetTrace(&actual_trace, flags, TestFunction);

    if (expected_trace != actual_trace) {
      ADD_FAILURE() << "Expected: " << ToString(expected_trace)
                    << "\nFound:    " << ToString(actual_trace);
    }
  }
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);

  maybeDisableSomeForkUbeDetectMechanisms();

  return RUN_ALL_TESTS();
}

#else

int main(int argc, char **argv) {
  printf("PASS\n");
  return 0;
}

#endif  // X86_64 && !SHARED_LIBRARY && !UNSAFE_DETERMINISTIC_MODE &&
        // USE_NR_getrandom && !AWSLC_VM_UBE_TESTING
