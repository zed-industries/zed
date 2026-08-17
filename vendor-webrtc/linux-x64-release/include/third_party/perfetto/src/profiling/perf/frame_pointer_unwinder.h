/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#ifndef SRC_PROFILING_PERF_FRAME_POINTER_UNWINDER_H_
#define SRC_PROFILING_PERF_FRAME_POINTER_UNWINDER_H_

#include <stdint.h>
#include <memory>
#include <vector>

#include <unwindstack/Error.h>
#include <unwindstack/MachineArm64.h>
#include <unwindstack/MachineRiscv64.h>
#include <unwindstack/MachineX86_64.h>
#include <unwindstack/Unwinder.h>

namespace perfetto {
namespace profiling {

class FramePointerUnwinder {
 public:
  FramePointerUnwinder(size_t max_frames,
                       unwindstack::Maps* maps,
                       unwindstack::Regs* regs,
                       std::shared_ptr<unwindstack::Memory> process_memory,
                       size_t stack_size)
      : max_frames_(max_frames),
        maps_(maps),
        regs_(regs),
        process_memory_(process_memory),
        stack_size_(stack_size),
        arch_(regs->Arch()) {
    stack_end_ = regs->sp() + stack_size;
  }

  FramePointerUnwinder(const FramePointerUnwinder&) = delete;
  FramePointerUnwinder& operator=(const FramePointerUnwinder&) = delete;

  void Unwind();

  // Disabling the resolving of names results in the function name being
  // set to an empty string and the function offset being set to zero.
  void SetResolveNames(bool resolve) { resolve_names_ = resolve; }

  unwindstack::ErrorCode LastErrorCode() const { return last_error_.code; }
  uint64_t warnings() const { return warnings_; }

  std::vector<unwindstack::FrameData> ConsumeFrames() {
    std::vector<unwindstack::FrameData> frames = std::move(frames_);
    frames_.clear();
    return frames;
  }

  bool IsArchSupported() const {
    return arch_ == unwindstack::ARCH_ARM64 ||
           arch_ == unwindstack::ARCH_X86_64;
  }

  void ClearErrors() {
    warnings_ = unwindstack::WARNING_NONE;
    last_error_.code = unwindstack::ERROR_NONE;
    last_error_.address = 0;
  }

 protected:
  const size_t max_frames_;
  unwindstack::Maps* maps_;
  unwindstack::Regs* regs_;
  std::vector<unwindstack::FrameData> frames_;
  std::shared_ptr<unwindstack::Memory> process_memory_;
  const size_t stack_size_;
  unwindstack::ArchEnum arch_ = unwindstack::ARCH_UNKNOWN;
  bool resolve_names_ = false;
  size_t stack_end_;

  unwindstack::ErrorData last_error_;
  uint64_t warnings_ = 0;

 private:
  void TryUnwind();
  // Given a frame pointer, returns the frame pointer of the calling stack
  // frame, places the return address of the calling stack frame into
  // `ret_addr` and stack pointer into `sp`.
  uint64_t DecodeFrame(uint64_t fp, uint64_t* ret_addr, uint64_t* sp);
  bool IsFrameValid(uint64_t fp, uint64_t sp);
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_PERF_FRAME_POINTER_UNWINDER_H_
