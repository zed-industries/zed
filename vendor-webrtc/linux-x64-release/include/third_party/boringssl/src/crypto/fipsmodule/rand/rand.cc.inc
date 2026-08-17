// Copyright 2014 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <assert.h>
#include <limits.h>
#include <string.h>

#if defined(BORINGSSL_FIPS)
#include <unistd.h>
#endif

#include <openssl/chacha.h>
#include <openssl/ctrdrbg.h>
#include <openssl/mem.h>

#include "../../bcm_support.h"
#include "../bcm_interface.h"
#include "../delocate.h"
#include "internal.h"


// It's assumed that the operating system always has an unfailing source of
// entropy which is accessed via |CRYPTO_sysrand[_for_seed]|. (If the operating
// system entropy source fails, it's up to |CRYPTO_sysrand| to abort the
// process—we don't try to handle it.)
//
// In addition, the hardware may provide a low-latency RNG. Intel's rdrand
// instruction is the canonical example of this. When a hardware RNG is
// available we don't need to worry about an RNG failure arising from fork()ing
// the process or moving a VM, so we can keep thread-local RNG state and use it
// as an additional-data input to CTR-DRBG.
//
// (We assume that the OS entropy is safe from fork()ing and VM duplication.
// This might be a bit of a leap of faith, esp on Windows, but there's nothing
// that we can do about it.)

// kReseedInterval is the number of generate calls made to CTR-DRBG before
// reseeding.
static const unsigned kReseedInterval = 4096;

// CRNGT_BLOCK_SIZE is the number of bytes in a “block” for the purposes of the
// continuous random number generator test in FIPS 140-2, section 4.9.2.
#define CRNGT_BLOCK_SIZE 16

namespace {
// rand_thread_state contains the per-thread state for the RNG.
struct rand_thread_state {
  CTR_DRBG_STATE drbg;
  uint64_t fork_generation;
  // calls is the number of generate calls made on |drbg| since it was last
  // (re)seeded. This is bound by |kReseedInterval|.
  unsigned calls;
  // last_block_valid is non-zero iff |last_block| contains data from
  // |get_seed_entropy|.
  int last_block_valid;
  // fork_unsafe_buffering is non-zero iff, when |drbg| was last (re)seeded,
  // fork-unsafe buffering was enabled.
  int fork_unsafe_buffering;

#if defined(BORINGSSL_FIPS)
  // last_block contains the previous block from |get_seed_entropy|.
  uint8_t last_block[CRNGT_BLOCK_SIZE];
  // next and prev form a NULL-terminated, double-linked list of all states in
  // a process.
  struct rand_thread_state *next, *prev;
  // clear_drbg_lock synchronizes between uses of |drbg| and
  // |rand_thread_state_clear_all| clearing it. This lock should be uncontended
  // in the common case, except on shutdown.
  CRYPTO_MUTEX clear_drbg_lock;
#endif
};
}  // namespace

#if defined(BORINGSSL_FIPS)
// thread_states_list is the head of a linked-list of all |rand_thread_state|
// objects in the process, one per thread. This is needed because FIPS requires
// that they be zeroed on process exit, but thread-local destructors aren't
// called when the whole process is exiting.
DEFINE_BSS_GET(struct rand_thread_state *, thread_states_list, nullptr)
DEFINE_STATIC_MUTEX(thread_states_list_lock)

static void rand_thread_state_clear_all(void) __attribute__((destructor));
static void rand_thread_state_clear_all(void) {
  CRYPTO_MUTEX_lock_write(thread_states_list_lock_bss_get());
  for (struct rand_thread_state *cur = *thread_states_list_bss_get();
       cur != NULL; cur = cur->next) {
    CRYPTO_MUTEX_lock_write(&cur->clear_drbg_lock);
    CTR_DRBG_clear(&cur->drbg);
  }
  // The locks are deliberately left locked so that any threads that are still
  // running will hang if they try to call |BCM_rand_bytes|. It also ensures
  // |rand_thread_state_free| cannot free any thread state while we've taken the
  // lock.
}
#endif

// rand_thread_state_free frees a |rand_thread_state|. This is called when a
// thread exits.
static void rand_thread_state_free(void *state_in) {
  struct rand_thread_state *state =
      reinterpret_cast<rand_thread_state *>(state_in);

  if (state_in == NULL) {
    return;
  }

#if defined(BORINGSSL_FIPS)
  CRYPTO_MUTEX_lock_write(thread_states_list_lock_bss_get());

  if (state->prev != NULL) {
    state->prev->next = state->next;
  } else if (*thread_states_list_bss_get() == state) {
    // |state->prev| may be NULL either if it is the head of the list,
    // or if |state| is freed before it was added to the list at all.
    // Compare against the head of the list to distinguish these cases.
    *thread_states_list_bss_get() = state->next;
  }

  if (state->next != NULL) {
    state->next->prev = state->prev;
  }

  CRYPTO_MUTEX_unlock_write(thread_states_list_lock_bss_get());

  CTR_DRBG_clear(&state->drbg);
#endif

  OPENSSL_free(state);
}

#if defined(OPENSSL_X86_64) && !defined(OPENSSL_NO_ASM) && \
    !defined(FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION)
// rdrand should only be called if either |have_rdrand| or |have_fast_rdrand|
// returned true.
static int rdrand(uint8_t *buf, const size_t len) {
  const size_t len_multiple8 = len & ~7;
  if (!CRYPTO_rdrand_multiple8_buf(buf, len_multiple8)) {
    return 0;
  }
  const size_t remainder = len - len_multiple8;

  if (remainder != 0) {
    assert(remainder < 8);

    uint8_t rand_buf[8];
    if (!CRYPTO_rdrand(rand_buf)) {
      return 0;
    }
    OPENSSL_memcpy(buf + len_multiple8, rand_buf, remainder);
  }

  return 1;
}

#else

static int rdrand(uint8_t *buf, size_t len) { return 0; }

#endif

bcm_status BCM_rand_bytes_hwrng(uint8_t *buf, const size_t len) {
  if (!have_rdrand()) {
    return bcm_status::failure;
  }
  if (rdrand(buf, len)) {
    return bcm_status::not_approved;
  }
  return bcm_status::failure;
}

#if defined(BORINGSSL_FIPS)

// In passive entropy mode, entropy is supplied from outside of the module via
// |BCM_rand_load_entropy| and is stored in global instance of the following
// structure.

struct entropy_buffer {
  // bytes contains entropy suitable for seeding a DRBG.
  uint8_t
      bytes[CRNGT_BLOCK_SIZE + CTR_DRBG_ENTROPY_LEN * BORINGSSL_FIPS_OVERREAD];
  // bytes_valid indicates the number of bytes of |bytes| that contain valid
  // data.
  size_t bytes_valid;
  // want_additional_input is true if any of the contents of |bytes| were
  // obtained via a method other than from the kernel. In these cases entropy
  // from the kernel is also provided via an additional input to the DRBG.
  int want_additional_input;
};

DEFINE_BSS_GET(struct entropy_buffer, entropy_buffer, {})
DEFINE_STATIC_MUTEX(entropy_buffer_lock)

bcm_infallible BCM_rand_load_entropy(const uint8_t *entropy, size_t entropy_len,
                                     int want_additional_input) {
  struct entropy_buffer *const buffer = entropy_buffer_bss_get();

  CRYPTO_MUTEX_lock_write(entropy_buffer_lock_bss_get());
  const size_t space = sizeof(buffer->bytes) - buffer->bytes_valid;
  if (entropy_len > space) {
    entropy_len = space;
  }

  OPENSSL_memcpy(&buffer->bytes[buffer->bytes_valid], entropy, entropy_len);
  buffer->bytes_valid += entropy_len;
  buffer->want_additional_input |= want_additional_input && (entropy_len != 0);
  CRYPTO_MUTEX_unlock_write(entropy_buffer_lock_bss_get());
  return bcm_infallible::not_approved;
}

// get_seed_entropy fills |out_entropy_len| bytes of |out_entropy| from the
// global |entropy_buffer|.
static void get_seed_entropy(uint8_t *out_entropy, size_t out_entropy_len,
                             int *out_want_additional_input) {
  struct entropy_buffer *const buffer = entropy_buffer_bss_get();
  if (out_entropy_len > sizeof(buffer->bytes)) {
    abort();
  }

  CRYPTO_MUTEX_lock_write(entropy_buffer_lock_bss_get());
  while (buffer->bytes_valid < out_entropy_len) {
    CRYPTO_MUTEX_unlock_write(entropy_buffer_lock_bss_get());
    RAND_need_entropy(out_entropy_len - buffer->bytes_valid);
    CRYPTO_MUTEX_lock_write(entropy_buffer_lock_bss_get());
  }

  *out_want_additional_input = buffer->want_additional_input;
  OPENSSL_memcpy(out_entropy, buffer->bytes, out_entropy_len);
  OPENSSL_memmove(buffer->bytes, &buffer->bytes[out_entropy_len],
                  buffer->bytes_valid - out_entropy_len);
  buffer->bytes_valid -= out_entropy_len;
  if (buffer->bytes_valid == 0) {
    buffer->want_additional_input = 0;
  }

  CRYPTO_MUTEX_unlock_write(entropy_buffer_lock_bss_get());
}

// rand_get_seed fills |seed| with entropy. In some cases, it will additionally
// fill |additional_input| with entropy to supplement |seed|. It sets
// |*out_additional_input_len| to the number of extra bytes.
static void rand_get_seed(struct rand_thread_state *state,
                          uint8_t seed[CTR_DRBG_ENTROPY_LEN],
                          uint8_t additional_input[CTR_DRBG_ENTROPY_LEN],
                          size_t *out_additional_input_len) {
  uint8_t entropy_bytes[sizeof(state->last_block) +
                        CTR_DRBG_ENTROPY_LEN * BORINGSSL_FIPS_OVERREAD];
  uint8_t *entropy = entropy_bytes;
  size_t entropy_len = sizeof(entropy_bytes);

  if (state->last_block_valid) {
    // No need to fill |state->last_block| with entropy from the read.
    entropy += sizeof(state->last_block);
    entropy_len -= sizeof(state->last_block);
  }

  int want_additional_input;
  get_seed_entropy(entropy, entropy_len, &want_additional_input);

  if (!state->last_block_valid) {
    OPENSSL_memcpy(state->last_block, entropy, sizeof(state->last_block));
    entropy += sizeof(state->last_block);
    entropy_len -= sizeof(state->last_block);
  }

  // See FIPS 140-2, section 4.9.2. This is the “continuous random number
  // generator test” which causes the program to randomly abort. Hopefully the
  // rate of failure is small enough not to be a problem in practice.
  if (CRYPTO_memcmp(state->last_block, entropy, sizeof(state->last_block)) ==
      0) {
    fprintf(CRYPTO_get_stderr(), "CRNGT failed.\n");
    BORINGSSL_FIPS_abort();
  }

  assert(entropy_len % CRNGT_BLOCK_SIZE == 0);
  for (size_t i = CRNGT_BLOCK_SIZE; i < entropy_len; i += CRNGT_BLOCK_SIZE) {
    if (CRYPTO_memcmp(entropy + i - CRNGT_BLOCK_SIZE, entropy + i,
                      CRNGT_BLOCK_SIZE) == 0) {
      fprintf(CRYPTO_get_stderr(), "CRNGT failed.\n");
      BORINGSSL_FIPS_abort();
    }
  }
  OPENSSL_memcpy(state->last_block, entropy + entropy_len - CRNGT_BLOCK_SIZE,
                 CRNGT_BLOCK_SIZE);

  assert(entropy_len == BORINGSSL_FIPS_OVERREAD * CTR_DRBG_ENTROPY_LEN);
  OPENSSL_memcpy(seed, entropy, CTR_DRBG_ENTROPY_LEN);

  for (size_t i = 1; i < BORINGSSL_FIPS_OVERREAD; i++) {
    for (size_t j = 0; j < CTR_DRBG_ENTROPY_LEN; j++) {
      seed[j] ^= entropy[CTR_DRBG_ENTROPY_LEN * i + j];
    }
  }

  // If we used something other than system entropy then also
  // opportunistically read from the system. This avoids solely relying on the
  // hardware once the entropy pool has been initialized.
  *out_additional_input_len = 0;
  if (want_additional_input &&
      CRYPTO_sysrand_if_available(additional_input, CTR_DRBG_ENTROPY_LEN)) {
    *out_additional_input_len = CTR_DRBG_ENTROPY_LEN;
  }
}

#else

// rand_get_seed fills |seed| with entropy. In some cases, it will additionally
// fill |additional_input| with entropy to supplement |seed|. It sets
// |*out_additional_input_len| to the number of extra bytes.
static void rand_get_seed(struct rand_thread_state *state,
                          uint8_t seed[CTR_DRBG_ENTROPY_LEN],
                          uint8_t additional_input[CTR_DRBG_ENTROPY_LEN],
                          size_t *out_additional_input_len) {
  // If not in FIPS mode, we don't overread from the system entropy source and
  // we don't depend only on the hardware RDRAND.
  CRYPTO_sysrand_for_seed(seed, CTR_DRBG_ENTROPY_LEN);
  *out_additional_input_len = 0;
}

#endif

bcm_infallible BCM_rand_bytes_with_additional_data(
    uint8_t *out, size_t out_len, const uint8_t user_additional_data[32]) {
  if (out_len == 0) {
    return bcm_infallible::approved;
  }

  const uint64_t fork_generation = CRYPTO_get_fork_generation();
  const int fork_unsafe_buffering = rand_fork_unsafe_buffering_enabled();

  // Additional data is mixed into every CTR-DRBG call to protect, as best we
  // can, against forks & VM clones. We do not over-read this information and
  // don't reseed with it so, from the point of view of FIPS, this doesn't
  // provide “prediction resistance”. But, in practice, it does.
  uint8_t additional_data[32];
  // Intel chips have fast RDRAND instructions while, in other cases, RDRAND can
  // be _slower_ than a system call.
  if (!have_fast_rdrand() ||
      !rdrand(additional_data, sizeof(additional_data))) {
    // Without a hardware RNG to save us from address-space duplication, the OS
    // entropy is used. This can be expensive (one read per |RAND_bytes| call)
    // and so is disabled when we have fork detection, or if the application has
    // promised not to fork.
    if (fork_generation != 0 || fork_unsafe_buffering) {
      OPENSSL_memset(additional_data, 0, sizeof(additional_data));
    } else if (!have_rdrand()) {
      // No alternative so block for OS entropy.
      CRYPTO_sysrand(additional_data, sizeof(additional_data));
    } else if (!CRYPTO_sysrand_if_available(additional_data,
                                            sizeof(additional_data)) &&
               !rdrand(additional_data, sizeof(additional_data))) {
      // RDRAND failed: block for OS entropy.
      CRYPTO_sysrand(additional_data, sizeof(additional_data));
    }
  }

  for (size_t i = 0; i < sizeof(additional_data); i++) {
    additional_data[i] ^= user_additional_data[i];
  }

  struct rand_thread_state stack_state;
  struct rand_thread_state *state = reinterpret_cast<rand_thread_state *>(
      CRYPTO_get_thread_local(OPENSSL_THREAD_LOCAL_RAND));

  if (state == NULL) {
    state = reinterpret_cast<rand_thread_state *>(
        OPENSSL_zalloc(sizeof(struct rand_thread_state)));
    if (state == NULL ||
        !CRYPTO_set_thread_local(OPENSSL_THREAD_LOCAL_RAND, state,
                                 rand_thread_state_free)) {
      // If the system is out of memory, use an ephemeral state on the
      // stack.
      state = &stack_state;
    }

    state->last_block_valid = 0;
    uint8_t seed[CTR_DRBG_ENTROPY_LEN];
    uint8_t personalization[CTR_DRBG_ENTROPY_LEN] = {0};
    size_t personalization_len = 0;
    rand_get_seed(state, seed, personalization, &personalization_len);

    if (!CTR_DRBG_init(&state->drbg, seed, personalization,
                       personalization_len)) {
      abort();
    }
    state->calls = 0;
    state->fork_generation = fork_generation;
    state->fork_unsafe_buffering = fork_unsafe_buffering;

#if defined(BORINGSSL_FIPS)
    CRYPTO_MUTEX_init(&state->clear_drbg_lock);
    if (state != &stack_state) {
      CRYPTO_MUTEX_lock_write(thread_states_list_lock_bss_get());
      struct rand_thread_state **states_list = thread_states_list_bss_get();
      state->next = *states_list;
      if (state->next != NULL) {
        state->next->prev = state;
      }
      state->prev = NULL;
      *states_list = state;
      CRYPTO_MUTEX_unlock_write(thread_states_list_lock_bss_get());
    }
#endif
  }

  if (state->calls >= kReseedInterval ||
      // If we've forked since |state| was last seeded, reseed.
      state->fork_generation != fork_generation ||
      // If |state| was seeded from a state with different fork-safety
      // preferences, reseed. Suppose |state| was fork-safe, then forked into
      // two children, but each of the children never fork and disable fork
      // safety. The children must reseed to avoid working from the same PRNG
      // state.
      state->fork_unsafe_buffering != fork_unsafe_buffering) {
    uint8_t seed[CTR_DRBG_ENTROPY_LEN];
    uint8_t reseed_additional_data[CTR_DRBG_ENTROPY_LEN] = {0};
    size_t reseed_additional_data_len = 0;
    rand_get_seed(state, seed, reseed_additional_data,
                  &reseed_additional_data_len);
#if defined(BORINGSSL_FIPS)
    // Take a read lock around accesses to |state->drbg|. This is needed to
    // avoid returning bad entropy if we race with
    // |rand_thread_state_clear_all|.
    CRYPTO_MUTEX_lock_read(&state->clear_drbg_lock);
#endif
    if (!CTR_DRBG_reseed(&state->drbg, seed, reseed_additional_data,
                         reseed_additional_data_len)) {
      abort();
    }
    state->calls = 0;
    state->fork_generation = fork_generation;
    state->fork_unsafe_buffering = fork_unsafe_buffering;
  } else {
#if defined(BORINGSSL_FIPS)
    CRYPTO_MUTEX_lock_read(&state->clear_drbg_lock);
#endif
  }

  int first_call = 1;
  while (out_len > 0) {
    size_t todo = out_len;
    if (todo > CTR_DRBG_MAX_GENERATE_LENGTH) {
      todo = CTR_DRBG_MAX_GENERATE_LENGTH;
    }

    if (!CTR_DRBG_generate(&state->drbg, out, todo, additional_data,
                           first_call ? sizeof(additional_data) : 0)) {
      abort();
    }

    out += todo;
    out_len -= todo;
    // Though we only check before entering the loop, this cannot add enough to
    // overflow a |size_t|.
    state->calls++;
    first_call = 0;
  }

  if (state == &stack_state) {
    CTR_DRBG_clear(&state->drbg);
  }

#if defined(BORINGSSL_FIPS)
  CRYPTO_MUTEX_unlock_read(&state->clear_drbg_lock);
#endif
  return bcm_infallible::approved;
}

bcm_infallible BCM_rand_bytes(uint8_t *out, size_t out_len) {
  static const uint8_t kZeroAdditionalData[32] = {0};
  BCM_rand_bytes_with_additional_data(out, out_len, kZeroAdditionalData);
  return bcm_infallible::approved;
}
