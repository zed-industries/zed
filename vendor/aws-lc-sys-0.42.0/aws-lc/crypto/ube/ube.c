// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/base.h>

#include "fork_ube_detect.h"
#include "vm_ube_detect.h"

#include "internal.h"
#include "../internal.h"

// What is a uniqueness-breaking event (UBE)?
// AWS-LC manages state that must be unique on each usage. For example, the
// CTR-DRBG state must be unique on each usage, otherwise the generation
// function will produce duplicated values. We name such state "unique state"
// or "unique memory" if referring directly to memory. Before using a unique
// state, we must make sure to randomize it to preserve its uniqueness.
//
// When/how to randomize state depends on the context. For example, the CTR-DRBG
// state, that is managed by AWS-LC, will randomize itself before usage during
// normal operation. But there are events where this assumption is violated. We
// call such events "uniqueness breaking events" (UBE). Forking an address space
// is an example of an UBE.
//
// By detecting UBE's we can ensure that unique state is properly randomized.
// AWS-LC is currently able to detect two different type of UBE's that would
// violate usage requirements of a unique state. The two UBE types are:
// Forking and VM resuming. Note, however, that detection methods rely on
// technology that is unique to a platform. Hence, support for UBE detection
// also depends on the platform AWS-LC is executed on.
//
// This file implements and manages the general machinery to detect an UBE. This
// should be used by the rest of the code-base to implement proper randomization
// of unique states before usage.
//
// ATTENTION: Before attempting to re-write this logic and add more mechanisms
// review P269907867 carefully. 

static CRYPTO_once_t ube_state_initialize_once = CRYPTO_ONCE_INIT;
static CRYPTO_once_t ube_detection_unavailable_once = CRYPTO_ONCE_INIT;
static struct CRYPTO_STATIC_MUTEX ube_lock = CRYPTO_STATIC_MUTEX_INIT;
// Locking for testing-specific code. Don't use |ube_lock| to avoid any
// potential for deadlocks.
static struct CRYPTO_STATIC_MUTEX ube_testing_lock = CRYPTO_STATIC_MUTEX_INIT;
static uint8_t ube_detection_unavailable = 0;
static uint8_t allow_mocked_detection = 0;

static uint64_t override_fork_generation_number = 0;
void set_fork_ube_generation_number_FOR_TESTING(uint64_t fork_gn) {
  CRYPTO_STATIC_MUTEX_lock_write(&ube_testing_lock);
  override_fork_generation_number = fork_gn;
  CRYPTO_STATIC_MUTEX_unlock_write(&ube_testing_lock);
}

static uint32_t override_vm_ube_generation_number = 0;
void set_vm_ube_generation_number_FOR_TESTING(uint32_t vm_ube_gn) {
  CRYPTO_STATIC_MUTEX_lock_write(&ube_testing_lock);
  override_vm_ube_generation_number = vm_ube_gn;
  CRYPTO_STATIC_MUTEX_unlock_write(&ube_testing_lock);
}

static int get_vm_ube_generation_number(uint32_t *gn) {
  if (allow_mocked_detection == 1) {
    CRYPTO_STATIC_MUTEX_lock_read(&ube_testing_lock);
    *gn = override_vm_ube_generation_number;
    CRYPTO_STATIC_MUTEX_unlock_read(&ube_testing_lock);
    return 1;
  }

  return CRYPTO_get_vm_ube_generation(gn);
}

static int get_fork_generation_number(uint64_t *gn) {
  if (allow_mocked_detection == 1) {
    CRYPTO_STATIC_MUTEX_lock_read(&ube_testing_lock);
    *gn = override_fork_generation_number;
    CRYPTO_STATIC_MUTEX_unlock_read(&ube_testing_lock);
    return 1;
  }

  uint64_t fork_gn = CRYPTO_get_fork_ube_generation();
  if (fork_gn == 0) {
    return 0;
  }

  *gn = fork_gn;
  return 1;
}


// The UBE generation number is shared for the entire address space. One could
// implement a per-thread UBE generation number. However, this could be
// inefficient for global unique states if care is not taken. Because every
// thread only have visibility on their own per-thread generation number, we
// could trigger a new randomization of the unique state per-thread. While a
// single randomization of the global unique state should be sufficient.
struct ube_state {
  uint64_t generation_number;
  uint64_t cached_fork_gn;
  uint32_t cached_vm_ube_gn;
};
static struct ube_state ube_global_state = { 0, 0, 0 };

// Convenience object that makes it easier to extend the number of detection
// methods without having to modify function signatures.
struct detection_gn {
#define NUMBER_OF_DETECTION_GENERATION_NUMBERS 2
  uint64_t current_fork_gn;
  uint32_t current_vm_ube_gn;
};

// set_ube_detection_unavailable_once is the single mutation point of
// |ube_detection_unavailable|. Sets the variable to 1 (true).
static void set_ube_detection_unavailable_once(void) {
  ube_detection_unavailable = 1;
}

// ube_failed is a convenience function to synchronize mutation of
// |ube_detection_unavailable|. In practice, synchronization is not strictly
// needed because currently the mutation is only ever assigning 1 to the
// variable.
static void ube_failed(void) {
  CRYPTO_once(&ube_detection_unavailable_once, set_ube_detection_unavailable_once);
}

// ube_state_initialize initializes the global state |ube_global_state|.
static void ube_state_initialize(void) {

  ube_global_state.generation_number = 0;
  int ret_fork_gn = get_fork_generation_number(
                      &(ube_global_state.cached_fork_gn));
  int ret_vm_ube_gn = get_vm_ube_generation_number(
                          &(ube_global_state.cached_vm_ube_gn));

  if (ret_fork_gn == 0 || ret_vm_ube_gn == 0) {
    ube_failed();
  }
}

// ube_update_state updates the global state |ube_global_state| with the
// generation numbers loaded into |current_detection_gn| and increments the UBE
// generation number.
static void ube_update_state(struct detection_gn *current_detection_gn) {

  ube_global_state.generation_number += 1;

  // Make sure we cache all new generation numbers. Otherwise, we might detect
  // a fork UBE but, in fact, both a fork and vm_ube UBE occurred. Then next
  // time we enter, a redundant reseed will be emitted.
  ube_global_state.cached_fork_gn = current_detection_gn->current_fork_gn;
  ube_global_state.cached_vm_ube_gn = current_detection_gn->current_vm_ube_gn;
}

// ube_get_detection_generation_numbers loads the current detection generation
// numbers into |current_detection_gn|.
//
// Returns 1 on success and 0 otherwise. The 0 return value means that a
// detection method we expected to be available, is in fact not.
static int ube_get_detection_generation_numbers(
  struct detection_gn *current_detection_gn) {

  // An attempt to prevent a situation where new detection methods are added but
  // we forget to load them.
  OPENSSL_STATIC_ASSERT(NUMBER_OF_DETECTION_GENERATION_NUMBERS == 2,
    not_handling_the_exact_number_of_detection_generation_numbers);

  int ret_detect_gn = get_fork_generation_number(
                        &(current_detection_gn->current_fork_gn));
  int ret_vm_ube_gn = get_vm_ube_generation_number(
                          &(current_detection_gn->current_vm_ube_gn));

  if (ret_detect_gn == 0 || ret_vm_ube_gn == 0) {
    return 0;
  }

  return 1;
}

// ube_is_detected computes whether a UBE has been detected or not by comparing
// the cached detection generation numbers with the current detection generation
// numbers. The current generation numbers must be loaded into
// |current_detection_gn| before calling this function.
//
// Returns 1 if UBE has been detected and 0 if no UBE has been detected.
//
// ATTENTION: Before attempting to re-write this logic and add more mechanisms
// review P269907867 carefully.
static int ube_is_detected(struct detection_gn *current_detection_gn) {

  if (ube_global_state.cached_fork_gn != current_detection_gn->current_fork_gn ||
      ube_global_state.cached_vm_ube_gn != current_detection_gn->current_vm_ube_gn) {
    return 1;
  }
  return 0;
}

int CRYPTO_get_ube_generation_number(uint64_t *current_generation_number) {

  GUARD_PTR(current_generation_number);

  CRYPTO_once(&ube_state_initialize_once, ube_state_initialize);

  // If something failed at an earlier point short-circuit immediately. This
  // saves work in case the UBE detection is not supported. The check below
  // must be done after attempting to initialize the UBE state. Because
  // initialization might fail and we can short-circuit here.
  //
  // |ube_detection_unavailable| and |allow_mocked_detection| are both global
  // variables that can be mutated in multiple threads. They are uint8_t typed
  // though and we assume read and write to them is atomic. This path is hot.
  // The assumption avoids us having to acquire (up to) two read locks that
  // would probably never be contended anyway. That would not block other
  // threads, but cost cycles regardless.
  if (ube_detection_unavailable == 1 &&
      allow_mocked_detection != 1) {
    return 0;
  }

  struct detection_gn current_detection_gn = { 0, 0 };

  // First read generation numbers for each supported detection method. We do
  // not mutate |ube_global_state|. So, a read lock is sufficient at this point.
  // Each individual detection method will have their own concurrency controls
  // if needed.

  if (ube_get_detection_generation_numbers(&current_detection_gn) != 1) {
    ube_failed();
    return 0;
  }
  CRYPTO_STATIC_MUTEX_lock_read(&ube_lock);
  if (ube_is_detected(&current_detection_gn) == 0) {
    // No UBE detected, so just grab UBE generation number from the state.
    *current_generation_number = ube_global_state.generation_number;
    CRYPTO_STATIC_MUTEX_unlock_read(&ube_lock);
    return 1;
  }
  CRYPTO_STATIC_MUTEX_unlock_read(&ube_lock);

  // Reaching this point means that an UBE has been detected. We must now
  // synchronize an update to the UBE generation number. To avoid redundant
  // reseeds, we must ensure the generation number is only incremented once for
  // all UBE's that might have happened. Therefore, first take a write lock but
  // before mutating the state, check for an UBE again. Checking again ensures
  // that only one thread increments the UBE generation number, because the
  // cached detection method generation numbers have been updated by the thread
  // that had the first entry.

  CRYPTO_STATIC_MUTEX_lock_write(&ube_lock);
  if (ube_get_detection_generation_numbers(&current_detection_gn) != 1) {
    ube_failed();
    CRYPTO_STATIC_MUTEX_unlock_write(&ube_lock);
    return 0;
  }
  if (ube_is_detected(&current_detection_gn) == 0) {
    // Another thread already updated the global state. Just load the UBE
    // generation number instead.
    *current_generation_number = ube_global_state.generation_number;
    CRYPTO_STATIC_MUTEX_unlock_write(&ube_lock);
    return 1;
  }

  // Okay, we are really the first to update the state after detecting an UBE.
  ube_update_state(&current_detection_gn);
  *current_generation_number = ube_global_state.generation_number;
  CRYPTO_STATIC_MUTEX_unlock_write(&ube_lock);

  return 1;
}

// Synchronize writing to |allow_mocked_detection|. But only to more easily
// reason about ordering. They are supposed to only be used in testing code
// and called, initially, from a single-threaded process at initialization time.
// We generally don't care about any contention that might happen.

void allow_mocked_ube_detection_FOR_TESTING(void) {
  CRYPTO_STATIC_MUTEX_lock_write(&ube_testing_lock);
  allow_mocked_detection = 1;
  CRYPTO_STATIC_MUTEX_unlock_write(&ube_testing_lock);
}

void disable_mocked_ube_detection_FOR_TESTING(void) {
  CRYPTO_STATIC_MUTEX_lock_write(&ube_testing_lock);
  allow_mocked_detection = 0;
  CRYPTO_STATIC_MUTEX_unlock_write(&ube_testing_lock);

  set_fork_ube_generation_number_FOR_TESTING(0);
  set_vm_ube_generation_number_FOR_TESTING(0);
}
