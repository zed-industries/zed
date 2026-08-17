/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_DESKTOP_REGION_H_
#define MODULES_DESKTOP_CAPTURE_DESKTOP_REGION_H_

#include <stdint.h>

#include <map>
#include <vector>

#include "modules/desktop_capture/desktop_geometry.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// DesktopRegion represents a region of the screen or window.
//
// Internally each region is stored as a set of rows where each row contains one
// or more rectangles aligned vertically.
class RTC_EXPORT DesktopRegion {
 private:
  // The following private types need to be declared first because they are used
  // in the public Iterator.

  // RowSpan represents a horizontal span withing a single row.
  struct RowSpan {
    RowSpan(int32_t left, int32_t right);

    // Used by std::vector<>.
    bool operator==(const RowSpan& that) const {
      return left == that.left && right == that.right;
    }

    int32_t left;
    int32_t right;
  };

  typedef std::vector<RowSpan> RowSpanSet;

  // Row represents a single row of a region. A row is set of rectangles that
  // have the same vertical position.
  struct Row {
    Row(const Row&);
    Row(Row&&);
    Row(int32_t top, int32_t bottom);
    ~Row();

    int32_t top;
    int32_t bottom;

    RowSpanSet spans;
  };

  // Type used to store list of rows in the region. The bottom position of row
  // is used as the key so that rows are always ordered by their position. The
  // map stores pointers to make Translate() more efficient.
  typedef std::map<int, Row*> Rows;

 public:
  // Iterator that can be used to iterate over rectangles of a DesktopRegion.
  // The region must not be mutated while the iterator is used.
  class RTC_EXPORT Iterator {
   public:
    explicit Iterator(const DesktopRegion& target);
    ~Iterator();

    bool IsAtEnd() const;
    void Advance();

    const DesktopRect& rect() const { return rect_; }

   private:
    const DesktopRegion& region_;

    // Updates `rect_` based on the current `row_` and `row_span_`. If
    // `row_span_` matches spans on consecutive rows then they are also merged
    // into `rect_`, to generate more efficient output.
    void UpdateCurrentRect();

    Rows::const_iterator row_;
    Rows::const_iterator previous_row_;
    RowSpanSet::const_iterator row_span_;
    DesktopRect rect_;
  };

  DesktopRegion();
  explicit DesktopRegion(const DesktopRect& rect);
  DesktopRegion(const DesktopRect* rects, int count);
  DesktopRegion(const DesktopRegion& other);
  ~DesktopRegion();

  DesktopRegion& operator=(const DesktopRegion& other);

  bool is_empty() const { return rows_.empty(); }

  bool Equals(const DesktopRegion& region) const;

  // Reset the region to be empty.
  void Clear();

  // Reset region to contain just `rect`.
  void SetRect(const DesktopRect& rect);

  // Adds specified rect(s) or region to the region.
  void AddRect(const DesktopRect& rect);
  void AddRects(const DesktopRect* rects, int count);
  void AddRegion(const DesktopRegion& region);

  // Finds intersection of two regions and stores them in the current region.
  void Intersect(const DesktopRegion& region1, const DesktopRegion& region2);

  // Same as above but intersects content of the current region with `region`.
  void IntersectWith(const DesktopRegion& region);

  // Clips the region by the `rect`.
  void IntersectWith(const DesktopRect& rect);

  // Subtracts `region` from the current content of the region.
  void Subtract(const DesktopRegion& region);

  // Subtracts `rect` from the current content of the region.
  void Subtract(const DesktopRect& rect);

  // Adds (dx, dy) to the position of the region.
  void Translate(int32_t dx, int32_t dy);

  void Swap(DesktopRegion* region);

 private:
  // Comparison functions used for std::lower_bound(). Compare left or right
  // edges withs a given `value`.
  static bool CompareSpanLeft(const RowSpan& r, int32_t value);
  static bool CompareSpanRight(const RowSpan& r, int32_t value);

  // Adds a new span to the row, coalescing spans if necessary.
  static void AddSpanToRow(Row* row, int32_t left, int32_t right);

  // Returns true if the `span` exists in the given `row`.
  static bool IsSpanInRow(const Row& row, const RowSpan& rect);

  // Calculates the intersection of two sets of spans.
  static void IntersectRows(const RowSpanSet& set1,
                            const RowSpanSet& set2,
                            RowSpanSet* output);

  static void SubtractRows(const RowSpanSet& set_a,
                           const RowSpanSet& set_b,
                           RowSpanSet* output);

  // Merges `row` with the row above it if they contain the same spans. Doesn't
  // do anything if called with `row` set to rows_.begin() (i.e. first row of
  // the region). If the rows were merged `row` remains a valid iterator to the
  // merged row.
  void MergeWithPrecedingRow(Rows::iterator row);

  Rows rows_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DESKTOP_REGION_H_
