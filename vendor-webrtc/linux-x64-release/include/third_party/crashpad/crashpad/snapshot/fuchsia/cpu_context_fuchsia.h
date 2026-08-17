// Copyright 2018 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_SNAPSHOT_FUCHSIA_CPU_CONTEXT_FUCHSIA_H_
#define CRASHPAD_SNAPSHOT_FUCHSIA_CPU_CONTEXT_FUCHSIA_H_

#include <zircon/syscalls/debug.h>

#include "build/build_config.h"
#include "snapshot/cpu_context.h"
#include "snapshot/fuchsia/process_reader_fuchsia.h"

namespace crashpad {
namespace internal {

#if defined(ARCH_CPU_X86_64) || DOXYGEN

//! \brief Initializes a CPUContextX86_64 structure from native context
//!     structures on Fuchsia.
//!
//! Segment registers are currently initialized to zero.
//!
//! \param[in] thread_context The native thread context.
//! \param[in] float_context The native floating point context.
//! \param[out] context The CPUContextX86_64 structure to initialize.
void InitializeCPUContextX86_64(
    const zx_thread_state_general_regs_t& thread_context,
    const zx_thread_state_fp_regs_t& float_context,
    CPUContextX86_64* context);

#endif  // ARCH_CPU_X86_64 || DOXYGEN

#if defined(ARCH_CPU_ARM64) || DOXYGEN

//! \brief Initializes a CPUContextARM64 structure from native context
//!     structures on Fuchsia.
//!
//! \param[in] thread_context The native thread context.
//! \param[in] vector_context The native vector context that also contains the
//!                           floating point registers.
//! \param[out] context The CPUContextARM64 structure to initialize.
void InitializeCPUContextARM64(
    const zx_thread_state_general_regs_t& thread_context,
    const zx_thread_state_vector_regs_t& vector_context,
    CPUContextARM64* context);

#endif  // ARCH_CPU_ARM64 || DOXYGEN

#if defined(ARCH_CPU_RISCV64) || DOXYGEN

//! \brief Initializes a CPUContextRISCV64 structure from native context
//!     structures on Fuchsia.
//!
//! \param[in] thread_context The native thread context.
//! \param[in] float_context The native floating point context.
//! \param[out] context The CPUContextRISCV64 structure to initialize.
void InitializeCPUContextRISCV64(
    const zx_thread_state_general_regs_t& thread_context,
    const zx_thread_state_fp_regs_t& float_context,
    CPUContextRISCV64* context);

#endif  // ARCH_CPU_RISCV64 || DOXYGEN

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_FUCHSIA_CPU_CONTEXT_FUCHSIA_H_
