// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_INFO_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_INFO_LIST_H_

#include "third_party/blink/renderer/core/layout/inline/inline_break_token.h"
#include "third_party/blink/renderer/core/layout/inline/line_info.h"

namespace blink {

//
// A `Vector` or `Deque`-like class for `LineInfo`, with a fixed maximum
// capacity.
//
// Use `LineInfoListOf` to instantiate. Algorithms can use this class to
// handle different capacities.
//
class LineInfoList {
  STACK_ALLOCATED();

 public:
  wtf_size_t Size() const { return size_; }
  bool IsEmpty() const { return !size_; }
  wtf_size_t MaxLines() const { return max_lines_; }

  // Out-of-bounds `index` will hit `DCHECK` and returns the value at `index %
  // max_lines_`.
  LineInfo& operator[](wtf_size_t index) {
    DCHECK_LT(index, Size());
    // SAFETY: The size of line_infos_ is max_lines_.
    return UNSAFE_BUFFERS(line_infos_[(start_index_ + index) % max_lines_]);
  }
  // Out-of-bounds `index` will hit `DCHECK` and returns the value at `index %
  // max_lines_`.
  const LineInfo& operator[](wtf_size_t index) const {
    DCHECK_LT(index, Size());
    // SAFETY: The size of line_infos_ is max_lines_.
    return UNSAFE_BUFFERS(line_infos_[(start_index_ + index) % max_lines_]);
  }
  // If empty, this will hit `DCHECK`.
  LineInfo& Front() { return (*this)[0]; }
  // If empty, this will hit `DCHECK`.
  const LineInfo& Front() const { return (*this)[0]; }
  // If empty, this will hit `DCHECK`.
  LineInfo& Back() { return (*this)[Size() - 1]; }
  // If empty, this will hit `DCHECK`.
  const LineInfo& Back() const { return (*this)[Size() - 1]; }

  void Shrink(wtf_size_t size) {
    DCHECK_LT(size, size_);
    size_ = size;
  }
  void Clear() { size_ = start_index_ = 0; }

  LineInfo& Append() {
    DCHECK_LT(size_, max_lines_);
    ++size_;
    return Back();
  }

  void RemoveFront() {
    DCHECK_GT(size_, 0u);
    --size_;
    start_index_ = (start_index_ + 1) % max_lines_;
  }

  // Get the cached `LineInfo` for the `break_token`, remove it from this
  // list, and set `is_cached_out` to `true`. If it doesn't exist, returns an
  // unused instance. The unused instance may be a new instance or a used
  // instance. Callsites are expected to call `LineInfo::Reset()`.
  LineInfo& Get(const InlineBreakToken* break_token, bool& is_cached_out);

 protected:
  LineInfoList(LineInfo* line_infos_instance, wtf_size_t max_lines)
      : max_lines_(max_lines) {
    CHECK_EQ(line_infos_instance, line_infos_);
  }

 private:
  LineInfo& UnusedInstance() {
    DCHECK(IsEmpty());
    return line_infos_[0];
  }

  wtf_size_t size_ = 0;
  wtf_size_t start_index_ = 0;
  const wtf_size_t max_lines_;
  LineInfo line_infos_[0];
};

inline LineInfo& LineInfoList::Get(const InlineBreakToken* break_token,
                                   bool& is_cached_out) {
  DCHECK(!is_cached_out);
  if (IsEmpty()) {
    return UnusedInstance();
  }

  LineInfo& line_info = Front();
  if (break_token ? line_info.Start() == break_token->Start()
                  : line_info.Start().IsZero()) {
    RemoveFront();
    is_cached_out = true;
    return line_info;
  }

  Clear();
  return UnusedInstance();
}

//
// Instantiate `LineInfo` with the given `capacity`.
//
template <wtf_size_t max_lines>
class LineInfoListOf : public LineInfoList {
 public:
  LineInfoListOf() : LineInfoList(line_infos_instance_, max_lines) {}

 private:
  LineInfo line_infos_instance_[max_lines];
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_INFO_LIST_H_
