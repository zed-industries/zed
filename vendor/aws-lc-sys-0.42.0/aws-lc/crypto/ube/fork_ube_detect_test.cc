// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/base.h>

// TSAN cannot cope with this test and complains that "starting new threads
// after multi-threaded fork is not supported".
#if defined(OPENSSL_LINUX) && !defined(OPENSSL_TSAN)
#include <errno.h>

#ifndef __STDC_FORMAT_MACROS
#define __STDC_FORMAT_MACROS
#endif
#include <inttypes.h>

#include <stdio.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

#include <functional>

#if defined(OPENSSL_THREADS)
#include <thread>
#include <vector>
#endif

#include <gtest/gtest.h>

#include "fork_ube_detect.h"

#include "../test/test_util.h"


static pid_t WaitpidEINTR(pid_t pid, int *out_status, int options) {
  pid_t ret;
  do {
    ret = waitpid(pid, out_status, options);
  } while (ret < 0 && errno == EINTR);

  return ret;
}

// The *InChild functions run inside a child process and must report errors via
// |stderr| and |_exit| rather than GTest.

static void CheckGenerationInChild(const char *name, uint64_t expected) {
  uint64_t generation = CRYPTO_get_fork_ube_generation();
  if (generation != expected) {
    fprintf(stderr, "%s generation (#1) was %" PRIu64 ", wanted %" PRIu64 ".\n",
            name, generation, expected);
    _exit(1);
  }

  // The generation should be stable.
  generation = CRYPTO_get_fork_ube_generation();
  if (generation != expected) {
    fprintf(stderr, "%s generation (#2) was %" PRIu64 ", wanted %" PRIu64 ".\n",
            name, generation, expected);
    _exit(1);
  }
}

// ForkInChild forks a child which runs |f|. If the child exits unsuccessfully,
// this function will also exit unsuccessfully.
static void ForkInChild(std::function<void()> f) {
  fflush(stderr);  // Avoid duplicating any buffered output.

  const pid_t pid = fork();
  if (pid < 0) {
    perror("fork");
    _exit(1);
  } else if (pid == 0) {
    f();
    _exit(0);
  }

  // Wait for the child and pass its exit code up.
  int status;
  if (WaitpidEINTR(pid, &status, 0) < 0) {
    perror("waitpid");
    _exit(1);
  }
  if (!WIFEXITED(status)) {
    fprintf(stderr, "Child did not exit cleanly.\n");
    _exit(1);
  }
  if (WEXITSTATUS(status) != 0) {
    // Pass the failure up.
    _exit(WEXITSTATUS(status));
  }
}

TEST(ForkDetect, Test) {

  maybeDisableSomeForkUbeDetectMechanisms();

  const uint64_t start = CRYPTO_get_fork_ube_generation();
  if (start == 0) {
    fprintf(stderr, "Fork detection not supported. Skipping test.\n");
    return;
  }

  // The fork generation should be stable.
  EXPECT_EQ(start, CRYPTO_get_fork_ube_generation());

  fflush(stderr);
  const pid_t child = fork();

  if (child == 0) {
    // Fork grandchildren before observing the fork generation. The
    // grandchildren will observe |start| + 1.
    for (int i = 0; i < 2; i++) {
      ForkInChild([&] { CheckGenerationInChild("Grandchild", start + 1); });
    }

    // Now the child also observes |start| + 1. This is fine because it has
    // already diverged from the grandchild at this point.
    CheckGenerationInChild("Child", start + 1);

    // Forked grandchildren will now observe |start| + 2.
    for (int i = 0; i < 2; i++) {
      ForkInChild([&] { CheckGenerationInChild("Grandchild", start + 2); });
    }

#if defined(OPENSSL_THREADS)
    // The fork generation logic itself must be thread-safe. We test this in a
    // child process to capture the actual fork detection. This segment is meant
    // to be tested in TSan.
    ForkInChild([&] {
      std::vector<std::thread> threads(4);
      for (int i = 0; i < 2; i++) {
        for (auto &t : threads) {
          t = std::thread(
              [&] { CheckGenerationInChild("Grandchild thread", start + 2); });
        }
        for (auto &t : threads) {
          t.join();
        }
      }
    });
#endif  // OPENSSL_THREADS

    // The child still observes |start| + 1.
    CheckGenerationInChild("Child", start + 1);
    _exit(0);
  }

  ASSERT_GT(child, 0) << "Error in fork: " << strerror(errno);
  int status;
  ASSERT_EQ(child, WaitpidEINTR(child, &status, 0))
      << "Error in waitpid: " << strerror(errno);
  ASSERT_TRUE(WIFEXITED(status));
  EXPECT_EQ(0, WEXITSTATUS(status)) << "Error in child process";

  // We still observe |start|.
  EXPECT_EQ(start, CRYPTO_get_fork_ube_generation());
}

#endif  // OPENSSL_LINUX && !OPENSSL_TSAN
