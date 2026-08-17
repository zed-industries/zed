/* OpenACC Profiling Interface

   Copyright (C) 2019-2020 Free Software Foundation, Inc.

   Contributed by Mentor, a Siemens Business.

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

#ifndef _ACC_PROF_H
#define _ACC_PROF_H 1


/* The OpenACC specification doesn't say so explicitly, but as its Profiling
   Interface explicitly makes use of, for example, <openacc.h>'s
   'acc_device_t', we supposedly are to '#include' that file here.  */

#include <openacc.h>


#ifdef __cplusplus
extern "C" {
#endif


/* Events.  */

typedef enum acc_event_t
{
  acc_ev_none = 0,
  acc_ev_device_init_start,
  acc_ev_device_init_end,
  acc_ev_device_shutdown_start,
  acc_ev_device_shutdown_end,
  acc_ev_runtime_shutdown,
  acc_ev_create,
  acc_ev_delete,
  acc_ev_alloc,
  acc_ev_free,
  acc_ev_enter_data_start,
  acc_ev_enter_data_end,
  acc_ev_exit_data_start,
  acc_ev_exit_data_end,
  acc_ev_update_start,
  acc_ev_update_end,
  acc_ev_compute_construct_start,
  acc_ev_compute_construct_end,
  acc_ev_enqueue_launch_start,
  acc_ev_enqueue_launch_end,
  acc_ev_enqueue_upload_start,
  acc_ev_enqueue_upload_end,
  acc_ev_enqueue_download_start,
  acc_ev_enqueue_download_end,
  acc_ev_wait_start,
  acc_ev_wait_end,
  acc_ev_last
} acc_event_t;


/* Callbacks Signature.  */

/* "The datatype 'ssize_t' means a signed 32-bit integer for a 32-bit binary
   and a 64-bit integer for a 64-bit binary".  */
typedef signed long int _acc_prof_ssize_t;
/* "The datatype 'size_t' means an unsigned 32-bit integer for a 32-bit binary
   and a 64-bit integer for a 64-bit binary".  */
typedef unsigned long int _acc_prof_size_t;
/* "The datatype 'int' means a 32-bit integer for both 32-bit and 64-bit
   binaries".  */
typedef int _acc_prof_int_t;

/* Internal helpers: a struct's 'valid_bytes' may be less than its 'sizeof'.  */
#define _ACC_PROF_VALID_BYTES_STRUCT(_struct, _lastfield, _valid_bytes_lastfield) \
  offsetof (_struct, _lastfield) + (_valid_bytes_lastfield)
#if 0 /* Untested.  */
#define _ACC_PROF_VALID_BYTES_TYPE_N(_type, _n, _valid_bytes_type) \
  ((_n - 1) * sizeof (_type) + (_valid_bytes_type))
#endif
#define _ACC_PROF_VALID_BYTES_BASICTYPE(_basictype) \
  (sizeof (_basictype))

typedef struct acc_prof_info
{
  acc_event_t event_type;
  _acc_prof_int_t valid_bytes;
  _acc_prof_int_t version;
  acc_device_t device_type;
  _acc_prof_int_t device_number;
  _acc_prof_int_t thread_id;
  _acc_prof_ssize_t async;
  _acc_prof_ssize_t async_queue;
  const char *src_file;
  const char *func_name;
  _acc_prof_int_t line_no, end_line_no;
  _acc_prof_int_t func_line_no, func_end_line_no;
#define _ACC_PROF_INFO_VALID_BYTES \
  _ACC_PROF_VALID_BYTES_STRUCT (acc_prof_info, func_end_line_no, \
				_ACC_PROF_VALID_BYTES_BASICTYPE (_acc_prof_int_t))
} acc_prof_info;

/* We implement the OpenACC 2.6 Profiling Interface.  */

#define _ACC_PROF_INFO_VERSION 201711

typedef enum acc_construct_t
{
  acc_construct_parallel = 0,
  acc_construct_kernels,
  acc_construct_loop,
  acc_construct_data,
  acc_construct_enter_data,
  acc_construct_exit_data,
  acc_construct_host_data,
  acc_construct_atomic,
  acc_construct_declare,
  acc_construct_init,
  acc_construct_shutdown,
  acc_construct_set,
  acc_construct_update,
  acc_construct_routine,
  acc_construct_wait,
  acc_construct_runtime_api,
  acc_construct_serial
} acc_construct_t;

typedef struct acc_data_event_info
{
  acc_event_t event_type;
  _acc_prof_int_t valid_bytes;
  acc_construct_t parent_construct;
  _acc_prof_int_t implicit;
  void *tool_info;
  const char *var_name;
  _acc_prof_size_t bytes;
  const void *host_ptr;
  const void *device_ptr;
#define _ACC_DATA_EVENT_INFO_VALID_BYTES \
  _ACC_PROF_VALID_BYTES_STRUCT (acc_data_event_info, device_ptr, \
				_ACC_PROF_VALID_BYTES_BASICTYPE (void *))
} acc_data_event_info;

typedef struct acc_launch_event_info
{
  acc_event_t event_type;
  _acc_prof_int_t valid_bytes;
  acc_construct_t parent_construct;
  _acc_prof_int_t implicit;
  void *tool_info;
  const char *kernel_name;
  _acc_prof_size_t num_gangs, num_workers, vector_length;
#define _ACC_LAUNCH_EVENT_INFO_VALID_BYTES \
  _ACC_PROF_VALID_BYTES_STRUCT (acc_launch_event_info, vector_length, \
				_ACC_PROF_VALID_BYTES_BASICTYPE (_acc_prof_size_t))
} acc_launch_event_info;

typedef struct acc_other_event_info
{
  acc_event_t event_type;
  _acc_prof_int_t valid_bytes;
  acc_construct_t parent_construct;
  _acc_prof_int_t implicit;
  void *tool_info;
#define _ACC_OTHER_EVENT_INFO_VALID_BYTES \
  _ACC_PROF_VALID_BYTES_STRUCT (acc_other_event_info, tool_info, \
				_ACC_PROF_VALID_BYTES_BASICTYPE (void *))
} acc_other_event_info;

typedef union acc_event_info
{
  acc_event_t event_type;
  acc_data_event_info data_event;
  acc_launch_event_info launch_event;
  acc_other_event_info other_event;
} acc_event_info;

typedef enum acc_device_api
{
  acc_device_api_none = 0,
  acc_device_api_cuda,
  acc_device_api_opencl,
  acc_device_api_coi,
  acc_device_api_other
} acc_device_api;

typedef struct acc_api_info
{
  acc_device_api device_api;
  _acc_prof_int_t valid_bytes;
  acc_device_t device_type;
  _acc_prof_int_t vendor;
  const void *device_handle;
  const void *context_handle;
  const void *async_handle;
#define _ACC_API_INFO_VALID_BYTES \
  _ACC_PROF_VALID_BYTES_STRUCT (acc_api_info, async_handle, \
				_ACC_PROF_VALID_BYTES_BASICTYPE (void *))
} acc_api_info;

/* Don't tag 'acc_prof_callback' as '__GOACC_NOTHROW': these functions are
   provided by user code, and must be expected to do anything.  */
typedef void (*acc_prof_callback) (acc_prof_info *, acc_event_info *,
				   acc_api_info *);


/* Loading the Library.  */

typedef enum acc_register_t
{
  acc_reg = 0,
  acc_toggle = 1,
  acc_toggle_per_thread = 2
} acc_register_t;

typedef void (*acc_prof_reg) (acc_event_t, acc_prof_callback, acc_register_t);
extern void acc_prof_register (acc_event_t, acc_prof_callback,
			       acc_register_t) __GOACC_NOTHROW;
extern void acc_prof_unregister (acc_event_t, acc_prof_callback,
				 acc_register_t) __GOACC_NOTHROW;
typedef void (*acc_query_fn) ();
typedef acc_query_fn (*acc_prof_lookup_func) (const char *);
extern acc_query_fn acc_prof_lookup (const char *) __GOACC_NOTHROW;
/* Don't tag 'acc_register_library' as '__GOACC_NOTHROW': this function can be
   overridden by user code, and must be expected to do anything.  */
extern void acc_register_library (acc_prof_reg, acc_prof_reg,
				  acc_prof_lookup_func);


#ifdef __cplusplus
}
#endif


#endif /* _ACC_PROF_H */
