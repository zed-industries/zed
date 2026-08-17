//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_CHECK_ASSERTION_H
#define TEST_SUPPORT_CHECK_ASSERTION_H

#include <array>
#include <cassert>
#include <csignal>
#include <cstdarg>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <exception>
#include <functional>
#include <regex>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>

#include <unistd.h>
#include <errno.h>
#include <signal.h>
#include <sys/wait.h>

#include "test_macros.h"
#include "test_allocator.h"

#if TEST_STD_VER < 11
#  error "C++11 or greater is required to use this header"
#endif

// When printing the assertion message to `stderr`, delimit it with a marker to make it easier to match the message
// later.
static constexpr const char* Marker = "###";

// (success, error-message-if-failed)
using MatchResult = std::pair<bool, std::string>;
using Matcher     = std::function<MatchResult(const std::string& /*text*/)>;

MatchResult MatchAssertionMessage(const std::string& text, std::string_view expected_message) {
  // Extract information from the error message. This has to stay synchronized with how we format assertions in the
  // library.
  std::regex assertion_format(".*###\\n(.*):(\\d+): assertion (.*) failed: (.*)\\n###");

  std::smatch match_result;
  bool has_match = std::regex_match(text, match_result, assertion_format);
  assert(has_match);
  assert(match_result.size() == 5);

  const std::string& file = match_result[1];
  int line                = std::stoi(match_result[2]);
  // Omitting `expression` in `match_result[3]`
  const std::string& assertion_message = match_result[4];

  bool result = assertion_message == expected_message;
  if (!result) {
    std::stringstream matching_error;
    matching_error                                                       //
        << "Expected message:   '" << expected_message.data() << "'\n"   //
        << "Actual message:     '" << assertion_message.c_str() << "'\n" //
        << "Source location:     " << file << ":" << std::to_string(line) << "\n";
    return MatchResult(/*success=*/false, matching_error.str());
  }

  return MatchResult(/*success=*/true, /*maybe_error=*/"");
}

Matcher MakeAssertionMessageMatcher(std::string_view assertion_message) {
  return [=](const std::string& text) { //
    return MatchAssertionMessage(text, assertion_message);
  };
}

Matcher MakeAnyMatcher() {
  return [](const std::string&) { //
    return MatchResult(/*success=*/true, /*maybe_error=*/"");
  };
}

enum class DeathCause {
  // Valid causes
  VerboseAbort = 1,
  StdAbort,
  StdTerminate,
  Trap,
  // Invalid causes
  DidNotDie,
  SetupFailure,
  Unknown
};

bool IsValidCause(DeathCause cause) {
  switch (cause) {
  case DeathCause::VerboseAbort:
  case DeathCause::StdAbort:
  case DeathCause::StdTerminate:
  case DeathCause::Trap:
    return true;
  default:
    return false;
  }
}

std::string ToString(DeathCause cause) {
  switch (cause) {
  case DeathCause::VerboseAbort:
    return "verbose abort";
  case DeathCause::StdAbort:
    return "`std::abort`";
  case DeathCause::StdTerminate:
    return "`std::terminate`";
  case DeathCause::Trap:
    return "trap";
  case DeathCause::DidNotDie:
    return "<invalid cause (child did not die)>";
  case DeathCause::SetupFailure:
    return "<invalid cause (child failed to set up test environment)>";
  case DeathCause::Unknown:
    return "<invalid cause (cause unknown)>";
  }

  assert(false && "Unreachable");
}

template <std::size_t N>
std::string ToString(std::array<DeathCause, N> const& causes) {
  std::stringstream ss;
  ss << "{";
  for (std::size_t i = 0; i != N; ++i) {
    ss << ToString(causes[i]);
    if (i + 1 != N)
      ss << ", ";
  }
  ss << "}";
  return ss.str();
}

[[noreturn]] void StopChildProcess(DeathCause cause) { std::exit(static_cast<int>(cause)); }

DeathCause ConvertToDeathCause(int val) {
  if (val < static_cast<int>(DeathCause::VerboseAbort) || val > static_cast<int>(DeathCause::Unknown)) {
    return DeathCause::Unknown;
  }
  return static_cast<DeathCause>(val);
}

enum class Outcome {
  Success,
  UnexpectedCause,
  UnexpectedErrorMessage,
  InvalidCause,
};

std::string ToString(Outcome outcome) {
  switch (outcome) {
  case Outcome::Success:
    return "success";
  case Outcome::UnexpectedCause:
    return "unexpected death cause";
  case Outcome::UnexpectedErrorMessage:
    return "unexpected error message";
  case Outcome::InvalidCause:
    return "invalid death cause";
  }

  assert(false && "Unreachable");
}

class DeathTestResult {
public:
  DeathTestResult() = default;
  DeathTestResult(Outcome set_outcome, DeathCause set_cause, const std::string& set_failure_description = "")
      : outcome_(set_outcome), cause_(set_cause), failure_description_(set_failure_description) {}

  bool success() const { return outcome() == Outcome::Success; }
  Outcome outcome() const { return outcome_; }
  DeathCause cause() const { return cause_; }
  const std::string& failure_description() const { return failure_description_; }

private:
  Outcome outcome_  = Outcome::Success;
  DeathCause cause_ = DeathCause::Unknown;
  std::string failure_description_;
};

class DeathTest {
public:
  DeathTest()                            = default;
  DeathTest(DeathTest const&)            = delete;
  DeathTest& operator=(DeathTest const&) = delete;

  template <std::size_t N, class Func>
  DeathTestResult Run(const std::array<DeathCause, N>& expected_causes, Func&& func, const Matcher& matcher) {
    std::signal(SIGABRT, [](int) { StopChildProcess(DeathCause::StdAbort); });
    std::set_terminate([] { StopChildProcess(DeathCause::StdTerminate); });

    DeathCause cause = Run(func);

    if (!IsValidCause(cause)) {
      return DeathTestResult(Outcome::InvalidCause, cause, ToString(cause));
    }

    if (std::find(expected_causes.begin(), expected_causes.end(), cause) == expected_causes.end()) {
      std::stringstream failure_description;
      failure_description                                               //
          << "Child died, but with a different death cause\n"           //
          << "Expected cause(s): " << ToString(expected_causes) << "\n" //
          << "Actual cause:      " << ToString(cause) << "\n";
      return DeathTestResult(Outcome::UnexpectedCause, cause, failure_description.str());
    }

    MatchResult match_result = matcher(GetChildStdErr());
    if (!match_result.first) {
      auto failure_description = std::string("Child died, but with a different error message\n") + match_result.second;
      return DeathTestResult(Outcome::UnexpectedErrorMessage, cause, failure_description);
    }

    return DeathTestResult(Outcome::Success, cause);
  }

  void PrintFailureDetails(std::string_view failure_description, std::string_view stmt, DeathCause cause) const {
    std::fprintf(
        stderr, "Failure: EXPECT_DEATH( %s ) failed!\n(reason: %s)\n\n", stmt.data(), failure_description.data());

    if (cause != DeathCause::Unknown) {
      std::fprintf(stderr, "child exit code: %d\n", GetChildExitCode());
    }
    std::fprintf(stderr, "---------- standard err ----------\n%s", GetChildStdErr().c_str());
    std::fprintf(stderr, "\n----------------------------------\n");
    std::fprintf(stderr, "---------- standard out ----------\n%s", GetChildStdOut().c_str());
    std::fprintf(stderr, "\n----------------------------------\n");
  };

private:
  int GetChildExitCode() const { return exit_code_; }
  std::string const& GetChildStdOut() const { return stdout_from_child_; }
  std::string const& GetChildStdErr() const { return stderr_from_child_; }

  template <class Func>
  DeathCause Run(Func&& f) {
    int pipe_res = pipe(stdout_pipe_fd_);
    assert(pipe_res != -1 && "failed to create pipe");
    pipe_res = pipe(stderr_pipe_fd_);
    assert(pipe_res != -1 && "failed to create pipe");
    pid_t child_pid = fork();
    assert(child_pid != -1 && "failed to fork a process to perform a death test");
    child_pid_ = child_pid;
    if (child_pid_ == 0) {
      RunForChild(std::forward<Func>(f));
      assert(false && "unreachable");
    }
    return RunForParent();
  }

  template <class Func>
  [[noreturn]] void RunForChild(Func&& f) {
    close(GetStdOutReadFD()); // don't need to read from the pipe in the child.
    close(GetStdErrReadFD());
    auto DupFD = [](int DestFD, int TargetFD) {
      int dup_result = dup2(DestFD, TargetFD);
      if (dup_result == -1)
        StopChildProcess(DeathCause::SetupFailure);
    };
    DupFD(GetStdOutWriteFD(), STDOUT_FILENO);
    DupFD(GetStdErrWriteFD(), STDERR_FILENO);

    f();
    StopChildProcess(DeathCause::DidNotDie);
  }

  static std::string ReadChildIOUntilEnd(int FD) {
    std::string error_msg;
    char buffer[256];
    int num_read;
    do {
      while ((num_read = read(FD, buffer, 255)) > 0) {
        buffer[num_read] = '\0';
        error_msg += buffer;
      }
    } while (num_read == -1 && errno == EINTR);
    return error_msg;
  }

  void CaptureIOFromChild() {
    close(GetStdOutWriteFD()); // no need to write from the parent process
    close(GetStdErrWriteFD());
    stdout_from_child_ = ReadChildIOUntilEnd(GetStdOutReadFD());
    stderr_from_child_ = ReadChildIOUntilEnd(GetStdErrReadFD());
    close(GetStdOutReadFD());
    close(GetStdErrReadFD());
  }

  DeathCause RunForParent() {
    CaptureIOFromChild();

    int status_value;
    pid_t result = waitpid(child_pid_, &status_value, 0);
    assert(result != -1 && "there is no child process to wait for");

    if (WIFEXITED(status_value)) {
      exit_code_ = WEXITSTATUS(status_value);
      return ConvertToDeathCause(exit_code_);
    }

    if (WIFSIGNALED(status_value)) {
      exit_code_ = WTERMSIG(status_value);
      // `__builtin_trap` generqtes `SIGILL` on x86 and `SIGTRAP` on ARM.
      if (exit_code_ == SIGILL || exit_code_ == SIGTRAP) {
        return DeathCause::Trap;
      }
    }

    return DeathCause::Unknown;
  }

  int GetStdOutReadFD() const { return stdout_pipe_fd_[0]; }
  int GetStdOutWriteFD() const { return stdout_pipe_fd_[1]; }
  int GetStdErrReadFD() const { return stderr_pipe_fd_[0]; }
  int GetStdErrWriteFD() const { return stderr_pipe_fd_[1]; }

  pid_t child_pid_ = -1;
  int exit_code_   = -1;
  int stdout_pipe_fd_[2];
  int stderr_pipe_fd_[2];
  std::string stdout_from_child_;
  std::string stderr_from_child_;
};

#ifdef _LIBCPP_VERSION
void std::__libcpp_verbose_abort(char const* format, ...) noexcept {
  va_list args;
  va_start(args, format);

  std::fprintf(stderr, "%s\n", Marker);
  std::vfprintf(stderr, format, args);
  std::fprintf(stderr, "%s", Marker);

  va_end(args);

  StopChildProcess(DeathCause::VerboseAbort);
}
#endif // _LIBCPP_VERSION

template <std::size_t N, class Func>
bool ExpectDeath(
    const std::array<DeathCause, N>& expected_causes, const char* stmt, Func&& func, const Matcher& matcher) {
  for (auto cause : expected_causes)
    assert(IsValidCause(cause));

  DeathTest test_case;
  DeathTestResult test_result = test_case.Run(expected_causes, func, matcher);
  if (!test_result.success()) {
    test_case.PrintFailureDetails(test_result.failure_description(), stmt, test_result.cause());
  }

  return test_result.success();
}

template <class Func>
bool ExpectDeath(DeathCause expected_cause, const char* stmt, Func&& func, const Matcher& matcher) {
  return ExpectDeath(std::array<DeathCause, 1>{expected_cause}, stmt, func, matcher);
}

template <std::size_t N, class Func>
bool ExpectDeath(const std::array<DeathCause, N>& expected_causes, const char* stmt, Func&& func) {
  return ExpectDeath(expected_causes, stmt, func, MakeAnyMatcher());
}

template <class Func>
bool ExpectDeath(DeathCause expected_cause, const char* stmt, Func&& func) {
  return ExpectDeath(std::array<DeathCause, 1>{expected_cause}, stmt, func, MakeAnyMatcher());
}

// clang-format off

/// Assert that the specified expression aborts with the expected cause and, optionally, error message.
#define EXPECT_ANY_DEATH(...)                         \
    assert(( ExpectDeath(std::array<DeathCause, 4>{DeathCause::VerboseAbort, DeathCause::StdAbort, DeathCause::StdTerminate, DeathCause::Trap}, #__VA_ARGS__, [&]() { __VA_ARGS__; } ) ))
#define EXPECT_DEATH(...)                         \
    assert(( ExpectDeath(DeathCause::VerboseAbort, #__VA_ARGS__, [&]() { __VA_ARGS__; } ) ))
#define EXPECT_DEATH_MATCHES(matcher, ...)        \
    assert(( ExpectDeath(DeathCause::VerboseAbort, #__VA_ARGS__, [&]() { __VA_ARGS__; }, matcher) ))
#define EXPECT_STD_ABORT(...)                 \
    assert(  ExpectDeath(DeathCause::StdAbort, #__VA_ARGS__, [&]() { __VA_ARGS__; })  )
#define EXPECT_STD_TERMINATE(...)                 \
    assert(  ExpectDeath(DeathCause::StdTerminate, #__VA_ARGS__, __VA_ARGS__)  )

#if defined(_LIBCPP_HARDENING_MODE) && _LIBCPP_HARDENING_MODE == _LIBCPP_HARDENING_MODE_DEBUG
#define TEST_LIBCPP_ASSERT_FAILURE(expr, message) \
    assert(( ExpectDeath(DeathCause::VerboseAbort, #expr, [&]() { (void)(expr); }, MakeAssertionMessageMatcher(message)) ))
#else
#define TEST_LIBCPP_ASSERT_FAILURE(expr, message) \
    assert(( ExpectDeath(DeathCause::Trap,         #expr, [&]() { (void)(expr); }) ))
#endif // _LIBCPP_HARDENING_MODE == _LIBCPP_HARDENING_MODE_DEBUG

// clang-format on

#endif // TEST_SUPPORT_CHECK_ASSERTION_H
