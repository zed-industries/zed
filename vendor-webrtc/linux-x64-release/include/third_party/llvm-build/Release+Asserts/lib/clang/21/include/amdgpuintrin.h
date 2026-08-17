//===-- amdgpuintrin.h - AMDPGU intrinsic functions -----------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef __AMDGPUINTRIN_H
#define __AMDGPUINTRIN_H

#ifndef __AMDGPU__
#error "This file is intended for AMDGPU targets or offloading to AMDGPU"
#endif

#ifndef __GPUINTRIN_H
#error "Never use <amdgpuintrin.h> directly; include <gpuintrin.h> instead"
#endif

_Pragma("omp begin declare target device_type(nohost)");
_Pragma("omp begin declare variant match(device = {arch(amdgcn)})");

// Type aliases to the address spaces used by the AMDGPU backend.
#define __gpu_private __attribute__((address_space(5)))
#define __gpu_constant __attribute__((address_space(4)))
#define __gpu_local __attribute__((address_space(3)))
#define __gpu_global __attribute__((address_space(1)))
#define __gpu_generic __attribute__((address_space(0)))

// Attribute to declare a function as a kernel.
#define __gpu_kernel __attribute__((amdgpu_kernel, visibility("protected")))

// Returns the number of workgroups in the 'x' dimension of the grid.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_num_blocks_x(void) {
  return __builtin_amdgcn_grid_size_x() / __builtin_amdgcn_workgroup_size_x();
}

// Returns the number of workgroups in the 'y' dimension of the grid.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_num_blocks_y(void) {
  return __builtin_amdgcn_grid_size_y() / __builtin_amdgcn_workgroup_size_y();
}

// Returns the number of workgroups in the 'z' dimension of the grid.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_num_blocks_z(void) {
  return __builtin_amdgcn_grid_size_z() / __builtin_amdgcn_workgroup_size_z();
}

// Returns the 'x' dimension of the current AMD workgroup's id.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_block_id_x(void) {
  return __builtin_amdgcn_workgroup_id_x();
}

// Returns the 'y' dimension of the current AMD workgroup's id.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_block_id_y(void) {
  return __builtin_amdgcn_workgroup_id_y();
}

// Returns the 'z' dimension of the current AMD workgroup's id.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_block_id_z(void) {
  return __builtin_amdgcn_workgroup_id_z();
}

// Returns the number of workitems in the 'x' dimension.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_num_threads_x(void) {
  return __builtin_amdgcn_workgroup_size_x();
}

// Returns the number of workitems in the 'y' dimension.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_num_threads_y(void) {
  return __builtin_amdgcn_workgroup_size_y();
}

// Returns the number of workitems in the 'z' dimension.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_num_threads_z(void) {
  return __builtin_amdgcn_workgroup_size_z();
}

// Returns the 'x' dimension id of the workitem in the current AMD workgroup.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_thread_id_x(void) {
  return __builtin_amdgcn_workitem_id_x();
}

// Returns the 'y' dimension id of the workitem in the current AMD workgroup.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_thread_id_y(void) {
  return __builtin_amdgcn_workitem_id_y();
}

// Returns the 'z' dimension id of the workitem in the current AMD workgroup.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_thread_id_z(void) {
  return __builtin_amdgcn_workitem_id_z();
}

// Returns the size of an AMD wavefront, either 32 or 64 depending on hardware
// and compilation options.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_num_lanes(void) {
  return __builtin_amdgcn_wavefrontsize();
}

// Returns the id of the thread inside of an AMD wavefront executing together.
_DEFAULT_FN_ATTRS static __inline__ uint32_t __gpu_lane_id(void) {
  return __builtin_amdgcn_mbcnt_hi(~0u, __builtin_amdgcn_mbcnt_lo(~0u, 0u));
}

// Returns the bit-mask of active threads in the current wavefront.
_DEFAULT_FN_ATTRS static __inline__ uint64_t __gpu_lane_mask(void) {
  return __builtin_amdgcn_read_exec();
}

// Copies the value from the first active thread in the wavefront to the rest.
_DEFAULT_FN_ATTRS static __inline__ uint32_t
__gpu_read_first_lane_u32(uint64_t __lane_mask, uint32_t __x) {
  return __builtin_amdgcn_readfirstlane(__x);
}

// Returns a bitmask of threads in the current lane for which \p x is true.
_DEFAULT_FN_ATTRS static __inline__ uint64_t __gpu_ballot(uint64_t __lane_mask,
                                                          bool __x) {
  // The lane_mask & gives the nvptx semantics when lane_mask is a subset of
  // the active threads
  return __lane_mask & __builtin_amdgcn_ballot_w64(__x);
}

// Waits for all the threads in the block to converge and issues a fence.
_DEFAULT_FN_ATTRS static __inline__ void __gpu_sync_threads(void) {
  __builtin_amdgcn_s_barrier();
  __builtin_amdgcn_fence(__ATOMIC_SEQ_CST, "workgroup");
}

// Wait for all threads in the wavefront to converge, this is a noop on AMDGPU.
_DEFAULT_FN_ATTRS static __inline__ void __gpu_sync_lane(uint64_t __lane_mask) {
  __builtin_amdgcn_wave_barrier();
}

// Shuffles the the lanes inside the wavefront according to the given index.
_DEFAULT_FN_ATTRS static __inline__ uint32_t
__gpu_shuffle_idx_u32(uint64_t __lane_mask, uint32_t __idx, uint32_t __x,
                      uint32_t __width) {
  uint32_t __lane = __idx + (__gpu_lane_id() & ~(__width - 1));
  return __builtin_amdgcn_ds_bpermute(__lane << 2, __x);
}

// Returns a bitmask marking all lanes that have the same value of __x.
_DEFAULT_FN_ATTRS static __inline__ uint64_t
__gpu_match_any_u32(uint64_t __lane_mask, uint32_t __x) {
  return __gpu_match_any_u32_impl(__lane_mask, __x);
}

// Returns a bitmask marking all lanes that have the same value of __x.
_DEFAULT_FN_ATTRS static __inline__ uint64_t
__gpu_match_any_u64(uint64_t __lane_mask, uint64_t __x) {
  return __gpu_match_any_u64_impl(__lane_mask, __x);
}

// Returns the current lane mask if every lane contains __x.
_DEFAULT_FN_ATTRS static __inline__ uint64_t
__gpu_match_all_u32(uint64_t __lane_mask, uint32_t __x) {
  return __gpu_match_all_u32_impl(__lane_mask, __x);
}

// Returns the current lane mask if every lane contains __x.
_DEFAULT_FN_ATTRS static __inline__ uint64_t
__gpu_match_all_u64(uint64_t __lane_mask, uint64_t __x) {
  return __gpu_match_all_u64_impl(__lane_mask, __x);
}

// Returns true if the flat pointer points to AMDGPU 'shared' memory.
_DEFAULT_FN_ATTRS static __inline__ bool __gpu_is_ptr_local(void *ptr) {
  return __builtin_amdgcn_is_shared((void [[clang::address_space(0)]] *)((
      void [[clang::opencl_generic]] *)ptr));
}

// Returns true if the flat pointer points to AMDGPU 'private' memory.
_DEFAULT_FN_ATTRS static __inline__ bool __gpu_is_ptr_private(void *ptr) {
  return __builtin_amdgcn_is_private((void [[clang::address_space(0)]] *)((
      void [[clang::opencl_generic]] *)ptr));
}

// Terminates execution of the associated wavefront.
_DEFAULT_FN_ATTRS [[noreturn]] static __inline__ void __gpu_exit(void) {
  __builtin_amdgcn_endpgm();
}

// Suspend the thread briefly to assist the scheduler during busy loops.
_DEFAULT_FN_ATTRS static __inline__ void __gpu_thread_suspend(void) {
  __builtin_amdgcn_s_sleep(2);
}

_Pragma("omp end declare variant");
_Pragma("omp end declare target");

#endif // __AMDGPUINTRIN_H
