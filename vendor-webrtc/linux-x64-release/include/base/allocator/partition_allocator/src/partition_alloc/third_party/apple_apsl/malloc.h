/*
 * Copyright (c) 1999 Apple Computer, Inc. All rights reserved.
 *
 * @APPLE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this
 * file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_LICENSE_HEADER_END@
 */

#ifndef PARTITION_ALLOC_THIRD_PARTY_APPLE_APSL_MALLOC_H_
#define PARTITION_ALLOC_THIRD_PARTY_APPLE_APSL_MALLOC_H_

#include <mach/boolean.h>

typedef struct _ChromeMallocZone {
  /* Only zone implementors should depend on the layout of this structure;
  Regular callers should use the access functions below */
  void* reserved1; /* RESERVED FOR CFAllocator DO NOT USE */
  void* reserved2; /* RESERVED FOR CFAllocator DO NOT USE */
  size_t (*size)(
      struct _malloc_zone_t* zone,
      const void* ptr); /* returns the size of a block or 0 if not in this zone;
                           must be fast, especially for negative answers */
  void* (*malloc)(struct _malloc_zone_t* zone, size_t size);
  void* (*calloc)(
      struct _malloc_zone_t* zone,
      size_t num_items,
      size_t size); /* same as malloc, but block returned is set to zero */
  void* (*valloc)(struct _malloc_zone_t* zone,
                  size_t size); /* same as malloc, but block returned is set to
                                   zero and is guaranteed to be page aligned */
  void (*free)(struct _malloc_zone_t* zone, void* ptr);
  void* (*realloc)(struct _malloc_zone_t* zone, void* ptr, size_t size);
  void (*destroy)(struct _malloc_zone_t*
                      zone); /* zone is destroyed and all memory reclaimed */
  const char* zone_name;

  /* Optional batch callbacks; these may be NULL */
  unsigned (*batch_malloc)(
      struct _malloc_zone_t* zone,
      size_t size,
      void** results,
      unsigned
          num_requested); /* given a size, returns pointers capable of holding
                             that size; returns the number of pointers allocated
                             (maybe 0 or less than num_requested) */
  void (*batch_free)(
      struct _malloc_zone_t* zone,
      void** to_be_freed,
      unsigned num_to_be_freed); /* frees all the pointers in to_be_freed; note
                                    that to_be_freed may be overwritten during
                                    the process */

  struct malloc_introspection_t* introspect;
  unsigned version;

  /* aligned memory allocation. The callback may be NULL. Present in version
   * >= 5. */
  void* (*memalign)(struct _malloc_zone_t* zone, size_t alignment, size_t size);

  /* free a pointer known to be in zone and known to have the given size. The
   * callback may be NULL. Present in version >= 6.*/
  void (*free_definite_size)(struct _malloc_zone_t* zone,
                             void* ptr,
                             size_t size);

  /* Empty out caches in the face of memory pressure. The callback may be NULL.
   * Present in version >= 8. */
  size_t (*pressure_relief)(struct _malloc_zone_t* zone, size_t goal);

  /*
   * Checks whether an address might belong to the zone. May be NULL. Present in
   * version >= 10. False positives are allowed (e.g. the pointer was freed, or
   * it's in zone space that has not yet been allocated. False negatives are not
   * allowed.
   */
  boolean_t (*claimed_address)(struct _malloc_zone_t* zone, void* ptr);

  /* For zone 0 implementations: try to free ptr, promising to call
   * find_zone_and_free if it turns out not to belong to us */
  void (*try_free_default)(struct _malloc_zone_t* zone, void* ptr);
} ChromeMallocZone;

/*********  Zone version summary ************/
// Version 0, but optional:
//   malloc_zone_t::batch_malloc
//   malloc_zone_t::batch_free
// Version 5:
//   malloc_zone_t::memalign
// Version 6:
//   malloc_zone_t::free_definite_size
// Version 7:
//   malloc_introspection_t::enable_discharge_checking
//   malloc_introspection_t::disable_discharge_checking
//   malloc_introspection_t::discharge
// Version 8:
//   malloc_zone_t::pressure_relief
// Version 9:
//   malloc_introspection_t::reinit_lock
// Version 10:
//   malloc_zone_t::claimed_address
// Version 11:
//   malloc_introspection_t::print_task
// Version 12:
//   malloc_introspection_t::task_statistics
// Version 13:
//   - malloc_zone_t::malloc and malloc_zone_t::calloc assume responsibility for
//     setting errno to ENOMEM on failure
//   - malloc_zone_t::try_free_default

// These functions are optional and calling them requires two checks:
//  * Check zone version to ensure zone struct is large enough to include the
//  member.
//  * Check that the function pointer is not null.

#endif  // PARTITION_ALLOC_THIRD_PARTY_APPLE_APSL_MALLOC_H_
