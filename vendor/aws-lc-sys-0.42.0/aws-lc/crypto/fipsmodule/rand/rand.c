// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/rand.h>
#include <openssl/mem.h>
#include <openssl/ctrdrbg.h>
#include <openssl/type_check.h>

#include "entropy/internal.h"
#include "internal.h"
#include "../../internal.h"
#include "../../ube/internal.h"
#include "../delocate.h"
#include "../service_indicator/internal.h"

// rand_thread_state contains the per-thread state for the RNG.
struct rand_thread_local_state {
  // Thread-local CTR-DRBG state. UBE unique state.
  CTR_DRBG_STATE drbg;

  // generate_calls_since_seed is the number of generate calls made on |drbg|
  // since it was last (re)seeded. Must be bounded by |kCtrDrbgReseedInterval|.
  uint64_t generate_calls_since_seed;

  // reseed_calls_since_initialization is the number of reseed calls made on
  // |drbg| since its initialization.
  // We assume 2^64 - 1 is an upper bound on the number of reseeds. Type must
  // support that.
  uint64_t reseed_calls_since_initialization;

  // generation_number caches the UBE generation number.
  uint64_t generation_number;

  // Entropy source. UBE unique state.
  struct entropy_source_t *entropy_source;

  // Backward and forward references to nodes in a doubly-linked list.
  struct rand_thread_local_state *next;
  struct rand_thread_local_state *previous;

  // Lock used when globally clearing (zeroising) all thread-local states at
  // process exit.
  CRYPTO_MUTEX state_clear_lock;
};
OPENSSL_STATIC_ASSERT((sizeof((struct rand_thread_local_state*)0)->generate_calls_since_seed) * 8 >= 48, value_can_overflow)

DEFINE_BSS_GET(struct rand_thread_local_state *, thread_states_list_head)
DEFINE_STATIC_MUTEX(thread_local_states_list_lock)

// thread_local_drbg_shutdown_started is set to a non-zero value during process
// exit by |rand_thread_local_state_clear_all|, while the linked-list lock is
// held. All reads and writes occur under |thread_local_states_list_lock|, so
// no atomic accessors are needed; doing the check under the lock also avoids
// the otherwise-racy window where a TLS destructor could observe the flag as
// unset, then block on the lock while shutdown zeroization runs, and then
// free a state whose |state_clear_lock| has been intentionally write-locked
// forever.
DEFINE_BSS_GET(int, thread_local_drbg_shutdown_started)

#if defined(_MSC_VER)
#pragma section(".CRT$XCU", read)
static void rand_thread_local_state_clear_all(void);
static void windows_install_rand_thread_local_state_clear_all(void) {
  atexit(&rand_thread_local_state_clear_all);
}
__declspec(allocate(".CRT$XCU")) void(*rand_fips_library_destructor)(void) =
    windows_install_rand_thread_local_state_clear_all;
#else
static void rand_thread_local_state_clear_all(void) __attribute__ ((destructor));
#endif

// At process exit not all threads will be scheduled and proper exited. To
// ensure no secret state is left, globally clear all thread-local states. This
// is a FIPS-derived requirement, see ISO/IEC 19790-2012 7.9.7.
//
// This is problematic because a thread might be scheduled and return
// randomness from a non-valid state. The linked application should obviously
// arrange that all threads are gracefully exited before exiting the process.
// Yet, in cases where such graceful exit does not happen we ensure that no
// output can be returned by locking each thread-local state's
// |state_clear_lock| and deliberately not releasing it. A synchronization step
// in the core randomness generation routine |RAND_bytes_core| then ensures
// that no randomness generation can occur after a thread-local state has been
// locked.
//
// We additionally set |thread_local_drbg_shutdown_started| under the
// linked-list lock so that any thread which has not yet registered a
// thread-local state cannot do so after this routine has begun zeroization;
// without this, a fresh thread could allocate a state and bypass zeroization.
// The linked-list lock itself is released at the end of this function. If we
// instead held it forever, |thread_local_list_delete_node| (called from a
// thread's TLS destructor in |rand_thread_local_state_free|) would block
// indefinitely. On Windows, TLS destructors run under the loader lock, so
// blocking there causes |ExitProcess| to hang waiting for the loader lock.
// See https://github.com/aws/aws-lc/issues/3197.
//
// When a thread-local DRBG is gated from returning output, we can invoke the
// entropy source zeroization from |state->entropy_source|. The entropy source
// implementation can assume that any returned seed is never used to generate
// any randomness that is later returned to a consumer.
static void rand_thread_local_state_clear_all(void) {
  CRYPTO_STATIC_MUTEX_lock_write(thread_local_states_list_lock_bss_get());

  // Idempotency guard. Under normal operation this routine runs exactly once
  // (via |atexit|), but it is also exposed for testing via
  // |rand_thread_local_state_clear_all_FOR_TESTING|, and re-running would
  // attempt to write-lock per-state |state_clear_lock|s that are already held
  // write-locked by the first invocation -- which is UB for a non-recursive
  // rwlock.
  if (*thread_local_drbg_shutdown_started_bss_get() != 0) {
    CRYPTO_STATIC_MUTEX_unlock_write(thread_local_states_list_lock_bss_get());
    return;
  }
  *thread_local_drbg_shutdown_started_bss_get() = 1;

  for (struct rand_thread_local_state *state = *thread_states_list_head_bss_get();
    state != NULL; state = state->next) {
    CRYPTO_MUTEX_lock_write(&state->state_clear_lock);
    CTR_DRBG_clear(&state->drbg);
  }

  for (struct rand_thread_local_state *state = *thread_states_list_head_bss_get();
    state != NULL; state = state->next) {
    state->entropy_source->methods->zeroize_thread(state->entropy_source);
  }

  CRYPTO_STATIC_MUTEX_unlock_write(thread_local_states_list_lock_bss_get());
}

void rand_thread_local_state_clear_all_FOR_TESTING(void) {
  rand_thread_local_state_clear_all();
}

// thread_local_list_delete_node removes |node_delete| from the global
// linked list and returns 1. If process-wide shutdown zeroization has already
// begun, the node is left in place and 0 is returned -- the caller must not
// free the node in that case because |rand_thread_local_state_clear_all| has
// write-locked its |state_clear_lock| and intentionally never releases it.
static int thread_local_list_delete_node(
  struct rand_thread_local_state *node_delete) {

  // Mutating the global linked list. Need to synchronize over all threads.
  CRYPTO_STATIC_MUTEX_lock_write(thread_local_states_list_lock_bss_get());

  // Re-check the shutdown flag under the lock. This makes the
  // "free vs. shutdown-zeroize" decision atomic with respect to
  // |rand_thread_local_state_clear_all|: either we delete and free before
  // shutdown begins, or shutdown wins and we leak the node deliberately.
  if (*thread_local_drbg_shutdown_started_bss_get() != 0) {
    CRYPTO_STATIC_MUTEX_unlock_write(thread_local_states_list_lock_bss_get());
    return 0;
  }

  struct rand_thread_local_state *node_head = *thread_states_list_head_bss_get();

  // We have [node_delete->previous] <--> [node_delete] <--> [node_delete->next]
  // and must end up with [node_delete->previous] <--> [node_delete->next]
  if (node_head == node_delete) {
    // If node node_delete is the head, we know that the backwards reference
    // does not exist but we need to update the head pointer.
    *thread_states_list_head_bss_get() = node_delete->next;
  } else {
    // On the other hand, if node_delete is not the head, then we need to update
    // the node node_delete->previous to point to the node node_delete->next.
    // But only if node_delete->previous actually exists.
    if (node_delete->previous != NULL) {
      (node_delete->previous)->next = node_delete->next;
    }
  }

  // Now [node_delete->previous] --> [node_delete->next]
  //                                          |
  //                         [node_delete] <--|
  // Final thing to do is to update the backwards reference for the node
  // node_delete->next, if it exists.
  if (node_delete->next != NULL) {
    // If node_delete is the head, then node_delete->previous is NULL. But that
    // is OK because node_delete->next is the new head and should therefore have
    // a backwards reference that is NULL.
    (node_delete->next)->previous = node_delete->previous;
  }

  CRYPTO_STATIC_MUTEX_unlock_write(thread_local_states_list_lock_bss_get());
  return 1;
}

// thread_local_list_add adds the state |node_add| to the linked list. Note that
// |node_add| is not added at the tail of the linked list, but is replacing the
// current head to keep the add operation at low time-complexity.
static void thread_local_list_add_node(
  struct rand_thread_local_state *node_add) {

  // node_add will be the new head and will not have a backwards reference.
  node_add->previous = NULL;

  // Mutating the global linked list. Need to synchronize over all threads.
  CRYPTO_STATIC_MUTEX_lock_write(thread_local_states_list_lock_bss_get());

  // If process-wide zeroization has already started, do not add a new state to
  // the list -- it would not have been zeroized by
  // |rand_thread_local_state_clear_all|. Instead, write-lock the state's
  // |state_clear_lock| and never release it. This causes any subsequent
  // |RAND_bytes_core| call on this thread to block forever on the read lock,
  // matching the FIPS-derived guarantee that no output is returned after
  // shutdown zeroization.
  if (*thread_local_drbg_shutdown_started_bss_get() != 0) {
    CRYPTO_MUTEX_lock_write(&node_add->state_clear_lock);
    CRYPTO_STATIC_MUTEX_unlock_write(thread_local_states_list_lock_bss_get());
    return;
  }

  // First get a reference to the pointer of the head of the linked list.
  // That is, the pointer to the head node node_head is *thread_states_head.
  struct rand_thread_local_state **thread_states_head = thread_states_list_head_bss_get();

  // We have [node_head] <--> [node_head->next] and must end up with
  // [node_add] <--> [node_head] <--> [node_head->next]
  // First make the forward reference
  node_add->next = *thread_states_head;

  // Only add a backwards reference if a head already existed (this might be
  // the first add).
  if (*thread_states_head != NULL) {
    (*thread_states_head)->previous = node_add;
  }

  // The last thing is to assign the new head.
  *thread_states_head = node_add;

  CRYPTO_STATIC_MUTEX_unlock_write(thread_local_states_list_lock_bss_get());
}

// rand_thread_local_state frees a |rand_thread_local_state|. This is called
// when a thread exits.
static void rand_thread_local_state_free(void *state_in) {

  struct rand_thread_local_state *state = state_in;
  if (state_in == NULL) {
    return;
  }

  // If process-wide shutdown zeroization has begun, the per-state
  // |state_clear_lock| has been write-locked and is intentionally never
  // released (so any in-flight |RAND_bytes| call cannot return output from a
  // zeroized state). |thread_local_list_delete_node| therefore detects this
  // case under the linked-list lock and refuses to delete; we must then leak
  // the node, since freeing it would destroy a held mutex. The OS reclaims
  // the memory at process exit. See issue #3197.
  if (thread_local_list_delete_node(state) == 0) {
    return;
  }

  // Potentially, something could kill the thread before an entropy source has
  // been associated to the thread-local randomness generator object.
  if (state->entropy_source != NULL) {
    state->entropy_source->methods->free_thread(state->entropy_source);
  }

  OPENSSL_free(state->entropy_source);
  OPENSSL_free(state);
}

// rand_ensure_valid_state determines whether |state| is in a valid state. The
// reasons are documented with inline comments in the function.
//
// Returns 1 if |state| is in a valid state and 0 otherwise.
static int rand_ensure_valid_state(const struct rand_thread_local_state *state) {

  // Currently, the Go based test runner cannot execute a unit test stanza with
  // guaranteed sequential execution. VM UBE testing is using a global file
  // that all unit tests will read if ever taking a path to |RAND_bytes|. The
  // validation below will have a high likelihood of triggering. Disable the
  // validation for VM UBE testing, until Go test runner can guarantee
  // sequential execution.

#if !defined(AWSLC_VM_UBE_TESTING)
  // We do not allow the UBE generation number to change while executing AWS-LC
  // randomness generation code e.g. while |RAND_bytes| executes. One way to hit
  // this error is if snapshotting the address space while executing
  // |RAND_bytes| and while VM UBE is active.
  uint64_t current_generation_number = 0;
  if (CRYPTO_get_ube_generation_number(&current_generation_number) == 1 &&
      current_generation_number != state->generation_number) {
    return 0;
  }
#endif

  return 1;
}

// rand_check_ctr_drbg_uniqueness computes whether |state| must be randomized
// to ensure uniqueness.
//
// Note: If |rand_check_ctr_drbg_uniqueness| returns 0 it does not necessarily
// imply that an UBE occurred. It can also mean that no UBE detection is
// supported or that UBE detection failed. In these cases, |state| must also be
// randomized to ensure uniqueness. Any special future cases can be handled in
// this function.
//
// Return 0 if |state| must be randomized. 1 otherwise.
static int rand_check_ctr_drbg_uniqueness(struct rand_thread_local_state *state) {

  uint64_t current_generation_number = 0;
  if (CRYPTO_get_ube_generation_number(&current_generation_number) != 1) {
    return 0;
  }

  if (current_generation_number != state->generation_number) {
    state->generation_number = current_generation_number;
    return 0;
  }

  return 1;
}

// rand_maybe_get_ctr_drbg_pred_resistance maybe fills |pred_resistance| with
// |RAND_PRED_RESISTANCE_LEN| bytes. The bytes are sourced from the prediction
// resistance source from the entropy source in |state|, if such a source has
// been configured.
//
// |*pred_resistance_len| is set to 0 if no prediction resistance source is
// available and |RAND_PRED_RESISTANCE_LEN| otherwise.
static void rand_maybe_get_ctr_drbg_pred_resistance(
  const struct entropy_source_t *entropy_source,
  uint8_t pred_resistance[RAND_PRED_RESISTANCE_LEN],
  size_t *pred_resistance_len) {

  GUARD_PTR_ABORT(entropy_source);
  GUARD_PTR_ABORT(pred_resistance_len);

  *pred_resistance_len = 0;

  if (entropy_source->methods->get_prediction_resistance != NULL) {
    if (entropy_source->methods->get_prediction_resistance(
      entropy_source, pred_resistance) != 1) {
      abort();
    }
    *pred_resistance_len = RAND_PRED_RESISTANCE_LEN;
  }
}

// rand_get_ctr_drbg_seed_entropy source entropy for seeding and reseeding the
// CTR-DRBG state. Firstly, |seed| is filled with |CTR_DRBG_ENTROPY_LEN| bytes
// from the seed source configured in |entropy_source|. Secondly, if available,
// |CTR_DRBG_ENTROPY_LEN| bytes is filled into |extra_entropy| sourced
// from the extra entropy source configured in |entropy_source|.
//
// |*extra_entropy_len| is set to 0 if no extra entropy source
// is available and |CTR_DRBG_ENTROPY_LEN| otherwise.
static void rand_get_ctr_drbg_seed_entropy(
  const struct entropy_source_t *entropy_source,
  uint8_t seed[CTR_DRBG_ENTROPY_LEN],
  uint8_t extra_entropy[CTR_DRBG_ENTROPY_LEN],
  size_t *extra_entropy_len) {

  GUARD_PTR_ABORT(entropy_source);
  GUARD_PTR_ABORT(extra_entropy_len);

  *extra_entropy_len = 0;

  // If the seed source is missing it is impossible to source any entropy.
  if (entropy_source->methods->get_seed(entropy_source, seed) != 1) {
    abort();
  }

  // Not all entropy source configurations will have a personalization string
  // source. Hence, it's optional. But use it if configured.
  if (entropy_source->methods->get_extra_entropy != NULL) {
    if(entropy_source->methods->get_extra_entropy(
        entropy_source, extra_entropy) != 1) {
      abort();
    }
    *extra_entropy_len = CTR_DRBG_ENTROPY_LEN;
  }
}

// rand_ctr_drbg_reseed reseeds the CTR-DRBG state in |state|.
static void rand_ctr_drbg_reseed(struct rand_thread_local_state *state,
  const uint8_t seed[CTR_DRBG_ENTROPY_LEN],
  const uint8_t additional_data[CTR_DRBG_ENTROPY_LEN],
  size_t additional_data_len) {

  GUARD_PTR_ABORT(state);

  if (CTR_DRBG_reseed(&(state->drbg), seed, additional_data,
        additional_data_len) != 1) {
    abort();
  }

  state->reseed_calls_since_initialization++;
  state->generate_calls_since_seed = 0;
}

// rand_state_initialize initializes the thread-local state |state|. In
// particular initializes the CTR-DRBG state with the initial seed material.
static void rand_state_initialize(struct rand_thread_local_state *state) {

  GUARD_PTR_ABORT(state);

  state->entropy_source = get_entropy_source();
  if (state->entropy_source == NULL) {
    abort();
  }

  uint8_t seed[CTR_DRBG_ENTROPY_LEN];
  uint8_t personalization_string[CTR_DRBG_ENTROPY_LEN];
  size_t personalization_string_len = 0;
  rand_get_ctr_drbg_seed_entropy(state->entropy_source, seed,
    personalization_string, &personalization_string_len);

  assert(personalization_string_len == 0 ||
         personalization_string_len == CTR_DRBG_ENTROPY_LEN);

  if (!CTR_DRBG_init(&(state->drbg), seed, personalization_string,
        personalization_string_len)) {
    abort();
  }

  state->reseed_calls_since_initialization = 0;
  state->generate_calls_since_seed = 0;
  uint64_t current_generation_number = 0;
  if (CRYPTO_get_ube_generation_number(&current_generation_number) != 1) {
    state->generation_number = 0;
  } else {
    state->generation_number = current_generation_number;
  }
  CRYPTO_MUTEX_init(&state->state_clear_lock);

  OPENSSL_cleanse(seed, CTR_DRBG_ENTROPY_LEN);
  OPENSSL_cleanse(personalization_string, CTR_DRBG_ENTROPY_LEN);
}

// RAND_bytes_core generates |out_len| bytes of randomness and puts them in
// |out|. The CTR-DRBG state in |state| is managed to ensure uniqueness and
// usage requirements are met.
//
// The argument |use_user_pred_resistance| must be either
// |RAND_USE_USER_PRED_RESISTANCE| or |RAND_NO_USER_PRED_RESISTANCE|. The former
// cause the content of |user_pred_resistance| to be mixed in as prediction
// resistance. The latter ensures that |user_pred_resistance| is not used.
//
// If the state has just been initialized, then |ctr_drbg_state_is_fresh| is 1.
// Otherwise, 0.
static void rand_bytes_core(
  struct rand_thread_local_state *state,
  uint8_t *out, size_t out_len,
  const uint8_t user_pred_resistance[RAND_PRED_RESISTANCE_LEN],
  int use_user_pred_resistance, int ctr_drbg_state_is_fresh) {

  GUARD_PTR_ABORT(state);
  GUARD_PTR_ABORT(out);

  // must_reseed_before_generate is 1 if we must reseed before invoking the
  // CTR-DRBG generate function CTR_DRBG_generate().
  int must_reseed_before_generate = 0;

  // Ensure that the CTR-DRBG state is unique. If the state is fresh then
  // uniqueness is guaranteed.
  if (rand_check_ctr_drbg_uniqueness(state) != 1 &&
      ctr_drbg_state_is_fresh != 1) {
    must_reseed_before_generate = 1;
  }

  // If a prediction resistance source is available, use it.
  // Prediction resistance is only used on first invocation of the CTR-DRBG,
  // ensuring that its state is randomized before generating output.
  size_t first_pred_resistance_len = 0;
  uint8_t pred_resistance[RAND_PRED_RESISTANCE_LEN] = {0};
  rand_maybe_get_ctr_drbg_pred_resistance(state->entropy_source,
    pred_resistance, &first_pred_resistance_len);

  // If caller input user-controlled prediction resistance, use it.
  if (use_user_pred_resistance == RAND_USE_USER_PRED_RESISTANCE) {
    for (size_t i = 0; i < RAND_PRED_RESISTANCE_LEN; i++) {
      pred_resistance[i] ^= user_pred_resistance[i];
    }
    first_pred_resistance_len = RAND_PRED_RESISTANCE_LEN;
  }

  assert(first_pred_resistance_len == 0 ||
         first_pred_resistance_len == RAND_PRED_RESISTANCE_LEN);

  // Synchronize with |rand_thread_local_state_clear_all|. In case a
  // thread-local state has been zeroized, thread execution will block here
  // because there is no secure way to generate randomness from that state.
  // Note that this lock is thread-local and therefore not contended except at
  // process exit.
  CRYPTO_MUTEX_lock_read(&state->state_clear_lock);

  // Iterate CTR-DRBG generate until |out_len| bytes of randomness have been
  // generated. CTR_DRBG_generate can maximally generate
  // |CTR_DRBG_MAX_GENERATE_LENGTH| bytes per usage of its state see
  // SP800-90A Rev 1 Table 3. If user requests more, we must generate output in
  // chunks and concatenate.
  while (out_len > 0) {
    size_t todo = out_len;

    if (todo > CTR_DRBG_MAX_GENERATE_LENGTH) {
      todo = CTR_DRBG_MAX_GENERATE_LENGTH;
    }

    if (must_reseed_before_generate == 1 ||
       (state->generate_calls_since_seed + 1) > kCtrDrbgReseedInterval) {

      // An unlock-lock cycle is located here to not acquire any locks while we
      // might perform system calls (e.g. when sourcing OS entropy). This
      // shields against known bugs. For example, glibc can implement locks
      // using memory transactions on powerpc that has been observed to break
      // when reaching |getrandom| through |syscall|. For this, see
      // https://github.com/google/boringssl/commit/17ce286e0792fc2855fb7e34a968bed17ae914af
      // https://www.kernel.org/doc/Documentation/powerpc/transactional_memory.txt
      //
      // Even though the unlock-lock cycle is under the loop iteration,
      // practically a request size (i.e. the value of |out_len|), will
      // almost-always be strictly less than |CTR_DRBG_MAX_GENERATE_LENGTH|.
      // Hence, practically, only one lock-unlock rotation will be required.
      CRYPTO_MUTEX_unlock_read(&state->state_clear_lock);
      uint8_t seed[CTR_DRBG_ENTROPY_LEN];
      uint8_t additional_data[CTR_DRBG_ENTROPY_LEN];
      size_t additional_data_len = 0;
      rand_get_ctr_drbg_seed_entropy(state->entropy_source, seed,
        additional_data, &additional_data_len);
      CRYPTO_MUTEX_lock_read(&state->state_clear_lock);

      rand_ctr_drbg_reseed(state, seed, additional_data,
        additional_data_len);
      must_reseed_before_generate = 0;

      OPENSSL_cleanse(seed, CTR_DRBG_ENTROPY_LEN);
      OPENSSL_cleanse(additional_data, CTR_DRBG_ENTROPY_LEN);
    }

    if (!CTR_DRBG_generate(&(state->drbg), out, todo, pred_resistance,
          first_pred_resistance_len)) {
      abort();
    }

    out += todo;
    out_len -= todo;
    state->generate_calls_since_seed++;
    first_pred_resistance_len = 0;
  }

  OPENSSL_cleanse(pred_resistance, RAND_PRED_RESISTANCE_LEN);

  if (rand_ensure_valid_state(state) != 1) {
    abort();
  }

  CRYPTO_MUTEX_unlock_read(&state->state_clear_lock);
}

static void rand_bytes_impl(thread_local_data_t tls_key, uint8_t *out,
  size_t out_len, const uint8_t user_pred_resistance[RAND_PRED_RESISTANCE_LEN],
  int use_user_pred_resistance) {

  if (out_len == 0) {
    return;
  }

  // Lock state here because CTR-DRBG-generate can be invoked multiple times
  // and every successful invocation increments the service indicator.
  FIPS_service_indicator_lock_state();

  struct rand_thread_local_state *state =
      CRYPTO_get_thread_local(tls_key);

  int ctr_drbg_state_is_fresh = 0;

  if (state == NULL) {
    state = OPENSSL_zalloc(sizeof(struct rand_thread_local_state));
    if (state == NULL ||
        CRYPTO_set_thread_local(tls_key, state,
                                   rand_thread_local_state_free) != 1) {
      abort();
    }

    rand_state_initialize(state);
    thread_local_list_add_node(state);

    ctr_drbg_state_is_fresh = 1;
  }

  rand_bytes_core(state, out, out_len, user_pred_resistance,
    use_user_pred_resistance, ctr_drbg_state_is_fresh);

  FIPS_service_indicator_unlock_state();
  FIPS_service_indicator_update_state();
}

int RAND_bytes_with_user_prediction_resistance(uint8_t *out, size_t out_len,
  const uint8_t user_pred_resistance[RAND_PRED_RESISTANCE_LEN]) {

  GUARD_PTR_ABORT(user_pred_resistance);

  rand_bytes_impl(OPENSSL_THREAD_LOCAL_PRIVATE_RAND, out, out_len,
    user_pred_resistance, RAND_USE_USER_PRED_RESISTANCE);
  return 1;
}

int RAND_bytes(uint8_t *out, size_t out_len) {

  static const uint8_t kZeroPredResistance[RAND_PRED_RESISTANCE_LEN] = {0};
  rand_bytes_impl(OPENSSL_THREAD_LOCAL_PRIVATE_RAND, out, out_len,
    kZeroPredResistance, RAND_NO_USER_PRED_RESISTANCE);
  return 1;
}

int RAND_priv_bytes(uint8_t *out, size_t out_len) {
  return RAND_bytes(out, out_len);
}

int RAND_public_bytes(uint8_t *out, size_t out_len) {
  static const uint8_t kZeroPredResistance[RAND_PRED_RESISTANCE_LEN] = {0};
  rand_bytes_impl(OPENSSL_THREAD_LOCAL_PUBLIC_RAND, out, out_len,
    kZeroPredResistance, RAND_NO_USER_PRED_RESISTANCE);
  return 1;
}

int RAND_pseudo_bytes(uint8_t *out, size_t out_len) {
  return RAND_bytes(out, out_len);
}

// Returns the number of generate calls made on the private thread-local state
// since last seed/reseed. Returns 0 if private thread-local state has not been
// initialized.
uint64_t get_private_thread_generate_calls_since_seed(void) {

  struct rand_thread_local_state *state =
      CRYPTO_get_thread_local(OPENSSL_THREAD_LOCAL_PRIVATE_RAND);
  if (state == NULL) {
    return 0;
  }

  return state->generate_calls_since_seed;
}

// Returns the number of reseed calls made on the private thread-local state
// since initialization. Returns 0 if private thread-local state has not been
// initialized.
uint64_t get_private_thread_reseed_calls_since_initialization(void) {

  struct rand_thread_local_state *state =
      CRYPTO_get_thread_local(OPENSSL_THREAD_LOCAL_PRIVATE_RAND);
  if (state == NULL) {
    return 0;
  }

  return state->reseed_calls_since_initialization;
}

// Returns the number of generate calls made on the public thread-local state
// since last seed/reseed. Returns 0 if public thread-local state has not been
// initialized.
uint64_t get_public_thread_generate_calls_since_seed(void) {

  struct rand_thread_local_state *state =
      CRYPTO_get_thread_local(OPENSSL_THREAD_LOCAL_PUBLIC_RAND);
  if (state == NULL) {
    return 0;
  }

  return state->generate_calls_since_seed;
}

// Returns the number of reseed calls made on the public thread-local state
// since initialization. Returns 0 if public thread-local state has not been
// initialized.
uint64_t get_public_thread_reseed_calls_since_initialization(void) {

  struct rand_thread_local_state *state =
      CRYPTO_get_thread_local(OPENSSL_THREAD_LOCAL_PUBLIC_RAND);
  if (state == NULL) {
    return 0;
  }

  return state->reseed_calls_since_initialization;
}
