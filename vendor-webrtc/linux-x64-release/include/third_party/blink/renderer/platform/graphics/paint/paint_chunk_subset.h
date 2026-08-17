// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CHUNK_SUBSET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CHUNK_SUBSET_H_

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_artifact.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// Provides access to a subset of paint chunks in a PaintArtifact.
class PaintChunkSubset {
  DISALLOW_NEW();

 public:
  // An empty subset.
  PaintChunkSubset() = default;

  // A subset containing a single paint chunk initially.
  PaintChunkSubset(const PaintArtifact& paint_artifact, const PaintChunk& chunk)
      : paint_artifact_(&paint_artifact) {
    wtf_size_t chunk_index = base::checked_cast<wtf_size_t>(
        &chunk - &paint_artifact_->GetPaintChunks().front());
    CHECK_LT(chunk_index, paint_artifact_->GetPaintChunks().size());
    subset_indices_.push_back(chunk_index);
  }

  // A subset containing the whole PaintArtifact. This is less efficient than
  // directly iterating paint_artifact->GetPaintChunks(), so should be rarely
  // used in production code.
  explicit PaintChunkSubset(const PaintArtifact& paint_artifact)
      : paint_artifact_(&paint_artifact) {
    for (wtf_size_t i = 0; i < paint_artifact_->GetPaintChunks().size(); ++i) {
      subset_indices_.push_back(i);
    }
  }

  void Trace(Visitor* visitor) const { visitor->Trace(paint_artifact_); }

  class Iterator {
    STACK_ALLOCATED();

   public:
    const PaintChunk& operator*() const { return GetChunk(); }
    const PaintChunk* operator->() const { return &GetChunk(); }
    bool operator==(const Iterator& other) const {
      DCHECK_EQ(subset_, other.subset_);
      return subset_index_ == other.subset_index_;
    }
    bool operator!=(const Iterator& other) const { return !(*this == other); }
    const Iterator& operator++() {
      ++subset_index_;
      return *this;
    }
    const Iterator& operator--() {
      --subset_index_;
      return *this;
    }
    Iterator operator+(wtf_size_t offset) const {
      DCHECK_LE(subset_index_ + offset, subset_->end().subset_index_);
      return Iterator(*subset_, subset_index_ + offset);
    }
    Iterator& operator+=(wtf_size_t offset) {
      DCHECK_LE(subset_index_ + offset, subset_->end().subset_index_);
      subset_index_ += offset;
      return *this;
    }

    // Returns the index of the current PaintChunk in the PaintArtifact.
    wtf_size_t IndexInPaintArtifact() const {
      return subset_->subset_indices_[subset_index_];
    }

    DisplayItemRange DisplayItems() const {
      auto& chunk = GetChunk();
      return UNSAFE_TODO(
          subset_->paint_artifact_->GetDisplayItemList().ItemsInRange(
              chunk.begin_index, chunk.end_index));
    }

   private:
    friend class PaintChunkSubset;

    Iterator(const PaintChunkSubset& subset, wtf_size_t subset_index)
        : subset_(&subset), subset_index_(subset_index) {}

    const PaintChunk& GetChunk() const {
      DCHECK_LT(subset_index_, subset_->end().subset_index_);
      return subset_->paint_artifact_->GetPaintChunks()[IndexInPaintArtifact()];
    }

    const PaintChunkSubset* subset_;
    wtf_size_t subset_index_;
  };

  using value_type = PaintChunk;
  using const_iterator = Iterator;

  Iterator begin() const { return Iterator(*this, 0); }
  Iterator end() const { return Iterator(*this, subset_indices_.size()); }

  const PaintChunk& operator[](wtf_size_t i) const {
    return paint_artifact_->GetPaintChunks()[subset_indices_[i]];
  }

  bool IsEmpty() const { return subset_indices_.empty(); }

  wtf_size_t size() const { return subset_indices_.size(); }

  const PaintArtifact& GetPaintArtifact() const { return *paint_artifact_; }

  // This can be used to swap in an updated artifact but care should be taken
  // because the PaintChunk indices into the new artifact must still be valid.
  void SetPaintArtifact(const PaintArtifact& paint_artifact) {
    // Existing paint chunk indices would be invalid if the sizes change.
    DCHECK_EQ(paint_artifact.GetPaintChunks().size(),
              paint_artifact_->GetPaintChunks().size());
    paint_artifact_ = &paint_artifact;
  }

  void Merge(const PaintChunkSubset& other) {
    DCHECK_EQ(paint_artifact_, other.paint_artifact_);
    subset_indices_.AppendVector(other.subset_indices_);
  }

  size_t ApproximateUnsharedMemoryUsage() const {
    return sizeof(*this) + subset_indices_.CapacityInBytes();
  }

  std::unique_ptr<JSONArray> ToJSON() const;

 private:
  Member<const PaintArtifact> paint_artifact_;
  Vector<wtf_size_t> subset_indices_;
};

using PaintChunkIterator = PaintChunkSubset::Iterator;

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&,
                                         const PaintChunkSubset&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CHUNK_SUBSET_H_
