// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PHYSICAL_FRAGMENT_RARE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PHYSICAL_FRAGMENT_RARE_DATA_H_

#include <climits>

#include "third_party/blink/renderer/core/layout/gap_fragment_data.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_rect.h"
#include "third_party/blink/renderer/core/layout/table/table_fragment_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class BoxFragmentBuilder;
class TableBorders;
struct FrameSetLayoutData;
struct MathMLPaintInfo;

// This class manages rare data of PhysicalBoxFragment.
// Only PhysicalBoxFragment should use this class.
//
// How to add a new field:
//  * Add a new enum member to FieldId.
//    If the new one has the maximum value, update kMaxValue too.
//  * Add a new union member of RareField.
//    The size of a member should be smaller than or same as LayoutUnit[4].
//    If it's larger, it should be pointed by a pointer such as
//    std::unique_ptr<>.
//
//  * Add field initialization code to two PhysicalFragmentRareData constructors
//  * Update DISPATCH_BY_MEMBER_TYPE macro.
class PhysicalFragmentRareData
    : public GarbageCollected<PhysicalFragmentRareData> {
 public:
  explicit PhysicalFragmentRareData(wtf_size_t num_fields);
  PhysicalFragmentRareData(const PhysicalRect* scrollable_overflow,
                           const PhysicalBoxStrut* borders,
                           const PhysicalBoxStrut* scrollbar,
                           const PhysicalBoxStrut* padding,
                           std::optional<PhysicalRect> inflow_bounds,
                           BoxFragmentBuilder& builder,
                           wtf_size_t num_fields);
  PhysicalFragmentRareData(const PhysicalFragmentRareData& other);
  ~PhysicalFragmentRareData();

  void Trace(Visitor* visitor) const {
    visitor->Trace(table_collapsed_borders_);
    visitor->Trace(table_column_geometries_);
    visitor->Trace(mathml_paint_info_);
    visitor->Trace(reading_flow_nodes_);
    visitor->Trace(gap_geometry_);
  }

 private:
  friend PhysicalBoxFragment;

  using RareBitFieldType = uint32_t;

  // In ARM, the size of a shift amount operand of shift instructions is same
  // as the size of shifted data. So FieldId should be RareBitFieldType.
  enum class FieldId : RareBitFieldType {
    kScrollableOverflow = 0,
    kBorders,
    kScrollbar,
    kPadding,
    kInflowBounds,
    kFrameSetLayoutData,
    kTableGridRect,
    kTableCollapsedBordersGeometry,
    kTableCellColumnIndex,
    kTableSectionStartRowIndex,
    kTableSectionRowOffsets,
    kPageName,
    kMargins,

    kMaxValue = kMargins,
  };
  static_assert(sizeof(RareBitFieldType) * CHAR_BIT >
                    static_cast<unsigned>(FieldId::kMaxValue),
                "RareBitFieldType is not big enough for FieldId.");

  struct RareField {
    union {
      PhysicalRect scrollable_overflow;
      PhysicalBoxStrut borders;
      PhysicalBoxStrut scrollbar;
      PhysicalBoxStrut padding;
      PhysicalRect inflow_bounds;
      std::unique_ptr<const FrameSetLayoutData> frame_set_layout_data;
      LogicalRect table_grid_rect;
      scoped_refptr<const TableBorders> table_collapsed_borders;
      std::unique_ptr<CollapsedTableBordersGeometry>
          table_collapsed_borders_geometry;
      wtf_size_t table_cell_column_index;
      wtf_size_t table_section_start_row_index;
      Vector<LayoutUnit> table_section_row_offsets;
      AtomicString page_name;
      PhysicalBoxStrut margins;
    };
    const FieldId type;

    explicit RareField(FieldId field_id);
    RareField(RareField&& other);
    ~RareField();
  };

  constexpr static RareBitFieldType FieldIdBit(FieldId field_id) {
    return static_cast<RareBitFieldType>(1) << static_cast<unsigned>(field_id);
  }

  constexpr static RareBitFieldType FieldIdLowerMask(FieldId field_id) {
    return ~(~static_cast<RareBitFieldType>(0)
             << static_cast<unsigned>(field_id));
  }

  ALWAYS_INLINE wtf_size_t GetFieldIndex(FieldId field_id) const {
    DCHECK(bit_field_ & FieldIdBit(field_id));
    return std::popcount(bit_field_ & FieldIdLowerMask(field_id));
  }

  ALWAYS_INLINE const RareField* GetField(FieldId field_id) const {
    if (bit_field_ & FieldIdBit(field_id)) {
      return &field_list_[GetFieldIndex(field_id)];
    }
    return nullptr;
  }

  template <bool allow_overwrite>
  RareField& EnsureField(FieldId field_id) {
    RareBitFieldType field_id_bit = FieldIdBit(field_id);
    if (allow_overwrite) {
      if (bit_field_ & field_id_bit) {
        return field_list_[GetFieldIndex(field_id)];
      }
    } else {
      DCHECK(!(bit_field_ & field_id_bit));
    }
    bit_field_ = bit_field_ | field_id_bit;
    wtf_size_t index = GetFieldIndex(field_id);
    field_list_.insert(index, RareField(field_id));
    return field_list_[index];
  }

  // We should not call this for a unique `field_id` multiple times.
  RareField& SetField(FieldId field_id) { return EnsureField<false>(field_id); }

  // We may call this for a unique `field_id` multiple times.
  RareField& EnsureField(FieldId field_id) {
    return EnsureField<true>(field_id);
  }

  // This should be called only if this has an element for `field_id`.
  void RemoveField(FieldId field_id) {
    field_list_.EraseAt(GetFieldIndex(field_id));
    bit_field_ = bit_field_ & ~FieldIdBit(field_id);
  }

  Vector<RareField, 1> field_list_;
  RareBitFieldType bit_field_ = 0u;
  // A garbage-collected field is not stored in the Vector in order to avoid
  // troublesome conditional tracing.
  Member<const TableBorders> table_collapsed_borders_;
  Member<const TableColumnGeometries> table_column_geometries_;
  Member<const MathMLPaintInfo> mathml_paint_info_;
  Member<const HeapVector<Member<Node>>> reading_flow_nodes_;
  Member<const GapGeometry> gap_geometry_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PHYSICAL_FRAGMENT_RARE_DATA_H_
