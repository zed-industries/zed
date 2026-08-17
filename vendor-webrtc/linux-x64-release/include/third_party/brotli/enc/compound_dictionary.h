/* Copyright 2017 Google Inc. All Rights Reserved.

   Distributed under MIT license.
   See file LICENSE for detail or copy at https://opensource.org/licenses/MIT
*/

#ifndef BROTLI_ENC_PREPARED_DICTIONARY_H_
#define BROTLI_ENC_PREPARED_DICTIONARY_H_

#include "../common/platform.h"
#include "../common/constants.h"
#include <brotli/shared_dictionary.h>
#include <brotli/types.h>
#include "memory.h"

static const uint32_t kPreparedDictionaryMagic = 0xDEBCEDE0;
static const uint64_t kPreparedDictionaryHashMul64Long =
    BROTLI_MAKE_UINT64_T(0x1FE35A7Bu, 0xD3579BD3u);

typedef struct PreparedDictionary {
  uint32_t magic;
  uint32_t source_offset;
  uint32_t source_size;
  uint32_t hash_bits;
  uint32_t bucket_bits;
  uint32_t slot_bits;

  /* --- Dynamic size members --- */

  /* uint32_t slot_offsets[1 << slot_bits]; */
  /* uint16_t heads[1 << bucket_bits]; */
  /* uint32_t items[variable]; */

  /* uint8_t source[source_size] */
} PreparedDictionary;

BROTLI_INTERNAL PreparedDictionary* CreatePreparedDictionary(MemoryManager* m,
    const uint8_t* source, size_t source_size);

BROTLI_INTERNAL void DestroyPreparedDictionary(MemoryManager* m,
    PreparedDictionary* dictionary);

typedef struct CompoundDictionary {
  /* LZ77 prefix, compound dictionary */
  size_t num_chunks;
  size_t total_size;
  /* Client instances. */
  const PreparedDictionary* chunks[SHARED_BROTLI_MAX_COMPOUND_DICTS + 1];
  const uint8_t* chunk_source[SHARED_BROTLI_MAX_COMPOUND_DICTS + 1];
  size_t chunk_offsets[SHARED_BROTLI_MAX_COMPOUND_DICTS + 1];

  size_t num_prepared_instances_;
  /* Owned instances. */
  PreparedDictionary* prepared_instances_[SHARED_BROTLI_MAX_COMPOUND_DICTS + 1];
} CompoundDictionary;

BROTLI_INTERNAL BROTLI_BOOL AttachPreparedDictionary(
    CompoundDictionary* compound, const PreparedDictionary* dictionary);

#endif /* BROTLI_ENC_PREPARED_DICTIONARY */
