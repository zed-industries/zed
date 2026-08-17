// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DIFF_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DIFF_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

using InspectorIndexMap =
    HashMap<unsigned, unsigned, WTF::IntWithZeroKeyHashTraits<unsigned>>;

// A general-purpose comparator between 2 arrays for the inspector
class CORE_EXPORT InspectorDiff {
 public:
  // Holds 2 arrays of some elements allowing to compare any pair of
  // element from the first array and element from the second array.
  class Input {
   public:
    virtual int GetLength1() = 0;
    virtual int GetLength2() = 0;
    virtual bool Equals(int index1, int index2) = 0;

   protected:
    virtual ~Input() = default;
  };

  // Receives compare result as a series of chunks.
  class Output {
   public:
    // Called whenever a match is reported between two lists
    virtual void AddMatch(int pos1, int pos2) = 0;

   protected:
    virtual ~Output() = default;
  };

  // Finds the LCS between two arrays and reports matching positions.
  static void CalculateMatches(Input* input, Output* result_writer);

  // Finds the mapping between 2 arrays of elements so that the
  // mapped elements form the longest common subsequence of the arrays.
  static void FindLCSMapping(const Vector<String>& list_a,
                             const Vector<String>& list_b,
                             InspectorIndexMap* a_to_b,
                             InspectorIndexMap* b_to_a);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DIFF_H_
