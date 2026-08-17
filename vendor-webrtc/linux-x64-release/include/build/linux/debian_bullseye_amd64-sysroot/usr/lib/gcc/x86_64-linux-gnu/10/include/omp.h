/* Copyright (C) 2005-2020 Free Software Foundation, Inc.
   Contributed by Richard Henderson <rth@redhat.com>.

   This file is part of the GNU Offloading and Multi Processing Library
   (libgomp).

   Libgomp is free software; you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   Libgomp is distributed in the hope that it will be useful, but WITHOUT ANY
   WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
   FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
   more details.

   Under Section 7 of GPL version 3, you are granted additional
   permissions described in the GCC Runtime Library Exception, version
   3.1, as published by the Free Software Foundation.

   You should have received a copy of the GNU General Public License and
   a copy of the GCC Runtime Library Exception along with this program;
   see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
   <http://www.gnu.org/licenses/>.  */

#ifndef _OMP_H
#define _OMP_H 1

#ifndef _LIBGOMP_OMP_LOCK_DEFINED
#define _LIBGOMP_OMP_LOCK_DEFINED 1
/* These two structures get edited by the libgomp build process to 
   reflect the shape of the two types.  Their internals are private
   to the library.  */

typedef struct
{
  unsigned char _x[4] 
    __attribute__((__aligned__(4)));
} omp_lock_t;

typedef struct
{
  /*
    Derive OMP_NEST_LOCK_SIZE and OMP_NEST_LOCK_ALIGN, don't hard
    code the values because the header is used for all multilibs.
    OMP_NEST_LOCK_SIZE  = 16
    OMP_NEST_LOCK_ALIGN = 8
  */
#if defined(__linux__) && !(defined(__hppa__) || defined(__alpha__))
  unsigned char _x[8 + sizeof (void *)] 
    __attribute__((__aligned__(sizeof (void *))));
#else
  unsigned char _x[16] 
    __attribute__((__aligned__(8)));
#endif
} omp_nest_lock_t;
#endif

typedef enum omp_sched_t
{
  omp_sched_static = 1,
  omp_sched_dynamic = 2,
  omp_sched_guided = 3,
  omp_sched_auto = 4,
  omp_sched_monotonic = 0x80000000U
} omp_sched_t;

typedef enum omp_proc_bind_t
{
  omp_proc_bind_false = 0,
  omp_proc_bind_true = 1,
  omp_proc_bind_master = 2,
  omp_proc_bind_close = 3,
  omp_proc_bind_spread = 4
} omp_proc_bind_t;

typedef enum omp_sync_hint_t
{
  omp_sync_hint_none = 0,
  omp_lock_hint_none = omp_sync_hint_none,
  omp_sync_hint_uncontended = 1,
  omp_lock_hint_uncontended = omp_sync_hint_uncontended,
  omp_sync_hint_contended = 2,
  omp_lock_hint_contended = omp_sync_hint_contended,
  omp_sync_hint_nonspeculative = 4,
  omp_lock_hint_nonspeculative = omp_sync_hint_nonspeculative,
  omp_sync_hint_speculative = 8,
  omp_lock_hint_speculative = omp_sync_hint_speculative
} omp_sync_hint_t;

typedef omp_sync_hint_t omp_lock_hint_t;

typedef struct __attribute__((__aligned__ (sizeof (void *)))) omp_depend_t
{
  char __omp_depend_t__[2 * sizeof (void *)];
} omp_depend_t;

typedef enum omp_pause_resource_t
{
  omp_pause_soft = 1,
  omp_pause_hard = 2
} omp_pause_resource_t;

#ifdef __cplusplus
extern "C" {
# define __GOMP_NOTHROW throw ()
#else
# define __GOMP_NOTHROW __attribute__((__nothrow__))
#endif

extern void omp_set_num_threads (int) __GOMP_NOTHROW;
extern int omp_get_num_threads (void) __GOMP_NOTHROW;
extern int omp_get_max_threads (void) __GOMP_NOTHROW;
extern int omp_get_thread_num (void) __GOMP_NOTHROW;
extern int omp_get_num_procs (void) __GOMP_NOTHROW;

extern int omp_in_parallel (void) __GOMP_NOTHROW;

extern void omp_set_dynamic (int) __GOMP_NOTHROW;
extern int omp_get_dynamic (void) __GOMP_NOTHROW;

extern void omp_set_nested (int) __GOMP_NOTHROW;
extern int omp_get_nested (void) __GOMP_NOTHROW;

extern void omp_init_lock (omp_lock_t *) __GOMP_NOTHROW;
extern void omp_init_lock_with_hint (omp_lock_t *, omp_sync_hint_t)
  __GOMP_NOTHROW;
extern void omp_destroy_lock (omp_lock_t *) __GOMP_NOTHROW;
extern void omp_set_lock (omp_lock_t *) __GOMP_NOTHROW;
extern void omp_unset_lock (omp_lock_t *) __GOMP_NOTHROW;
extern int omp_test_lock (omp_lock_t *) __GOMP_NOTHROW;

extern void omp_init_nest_lock (omp_nest_lock_t *) __GOMP_NOTHROW;
extern void omp_init_nest_lock_with_hint (omp_nest_lock_t *, omp_sync_hint_t)
  __GOMP_NOTHROW;
extern void omp_destroy_nest_lock (omp_nest_lock_t *) __GOMP_NOTHROW;
extern void omp_set_nest_lock (omp_nest_lock_t *) __GOMP_NOTHROW;
extern void omp_unset_nest_lock (omp_nest_lock_t *) __GOMP_NOTHROW;
extern int omp_test_nest_lock (omp_nest_lock_t *) __GOMP_NOTHROW;

extern double omp_get_wtime (void) __GOMP_NOTHROW;
extern double omp_get_wtick (void) __GOMP_NOTHROW;

extern void omp_set_schedule (omp_sched_t, int) __GOMP_NOTHROW;
extern void omp_get_schedule (omp_sched_t *, int *) __GOMP_NOTHROW;
extern int omp_get_thread_limit (void) __GOMP_NOTHROW;
extern void omp_set_max_active_levels (int) __GOMP_NOTHROW;
extern int omp_get_max_active_levels (void) __GOMP_NOTHROW;
extern int omp_get_level (void) __GOMP_NOTHROW;
extern int omp_get_ancestor_thread_num (int) __GOMP_NOTHROW;
extern int omp_get_team_size (int) __GOMP_NOTHROW;
extern int omp_get_active_level (void) __GOMP_NOTHROW;

extern int omp_in_final (void) __GOMP_NOTHROW;

extern int omp_get_cancellation (void) __GOMP_NOTHROW;
extern omp_proc_bind_t omp_get_proc_bind (void) __GOMP_NOTHROW;
extern int omp_get_num_places (void) __GOMP_NOTHROW;
extern int omp_get_place_num_procs (int) __GOMP_NOTHROW;
extern void omp_get_place_proc_ids (int, int *) __GOMP_NOTHROW;
extern int omp_get_place_num (void) __GOMP_NOTHROW;
extern int omp_get_partition_num_places (void) __GOMP_NOTHROW;
extern void omp_get_partition_place_nums (int *) __GOMP_NOTHROW;

extern void omp_set_default_device (int) __GOMP_NOTHROW;
extern int omp_get_default_device (void) __GOMP_NOTHROW;
extern int omp_get_num_devices (void) __GOMP_NOTHROW;
extern int omp_get_num_teams (void) __GOMP_NOTHROW;
extern int omp_get_team_num (void) __GOMP_NOTHROW;

extern int omp_is_initial_device (void) __GOMP_NOTHROW;
extern int omp_get_initial_device (void) __GOMP_NOTHROW;
extern int omp_get_max_task_priority (void) __GOMP_NOTHROW;

extern void *omp_target_alloc (__SIZE_TYPE__, int) __GOMP_NOTHROW;
extern void omp_target_free (void *, int) __GOMP_NOTHROW;
extern int omp_target_is_present (const void *, int) __GOMP_NOTHROW;
extern int omp_target_memcpy (void *, const void *, __SIZE_TYPE__,
			      __SIZE_TYPE__, __SIZE_TYPE__, int, int)
  __GOMP_NOTHROW;
extern int omp_target_memcpy_rect (void *, const void *, __SIZE_TYPE__, int,
				   const __SIZE_TYPE__ *,
				   const __SIZE_TYPE__ *,
				   const __SIZE_TYPE__ *,
				   const __SIZE_TYPE__ *,
				   const __SIZE_TYPE__ *, int, int)
  __GOMP_NOTHROW;
extern int omp_target_associate_ptr (const void *, const void *, __SIZE_TYPE__,
				     __SIZE_TYPE__, int) __GOMP_NOTHROW;
extern int omp_target_disassociate_ptr (const void *, int) __GOMP_NOTHROW;

extern void omp_set_affinity_format (const char *) __GOMP_NOTHROW;
extern __SIZE_TYPE__ omp_get_affinity_format (char *, __SIZE_TYPE__)
  __GOMP_NOTHROW;
extern void omp_display_affinity (const char *) __GOMP_NOTHROW;
extern __SIZE_TYPE__ omp_capture_affinity (char *, __SIZE_TYPE__, const char *)
  __GOMP_NOTHROW;

extern int omp_pause_resource (omp_pause_resource_t, int) __GOMP_NOTHROW;
extern int omp_pause_resource_all (omp_pause_resource_t) __GOMP_NOTHROW;

#ifdef __cplusplus
}
#endif

#endif /* _OMP_H */
