/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TEXT_AUTOSIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TEXT_AUTOSIZER_H_

#include <unicode/uchar.h>

#include "base/dcheck_is_on.h"
#include "third_party/blink/public/mojom/frame/text_autosizer_page_info.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace gfx {
class Size;
}

namespace blink {

class Document;
class Frame;
class LayoutBlock;
class LayoutBox;
class LayoutObject;
class LayoutTable;
class LayoutText;
class LocalFrame;
class Page;

inline bool operator==(const mojom::blink::TextAutosizerPageInfo& lhs,
                       const mojom::blink::TextAutosizerPageInfo& rhs) {
  return lhs.main_frame_width == rhs.main_frame_width &&
         lhs.main_frame_layout_width == rhs.main_frame_layout_width &&
         lhs.device_scale_adjustment == rhs.device_scale_adjustment;
}

inline bool operator!=(const mojom::blink::TextAutosizerPageInfo& lhs,
                       const mojom::blink::TextAutosizerPageInfo& rhs) {
  return !(lhs == rhs);
}

// Single-pass text autosizer. Documentation at:
// http://tinyurl.com/TextAutosizer

class CORE_EXPORT TextAutosizer final : public GarbageCollected<TextAutosizer> {
 public:
  explicit TextAutosizer(const Document*);
  TextAutosizer(const TextAutosizer&) = delete;
  TextAutosizer& operator=(const TextAutosizer&) = delete;
  ~TextAutosizer();

  // computed_size should include zoom.
  static float ComputeAutosizedFontSize(float computed_size,
                                        float multiplier,
                                        float effective_zoom);
  // Static to allow starting updates from the frame tree root when it's a
  // remote frame, though this function is called for all main frames, local
  // or remote.
  static void UpdatePageInfoInAllFrames(Frame* root_frame);

  bool HasLayoutInlineSizeChanged() const;
  void UpdatePageInfo();
  void Record(LayoutBlock*);
  void Record(LayoutText*);
  void Destroy(LayoutObject*);

  bool PageNeedsAutosizing() const;

  // Register the specified |inline_size| for |ng_block| if the document has
  // a TextAutosizer instance and it should handle layout.
  static void MaybeRegisterInlineSize(const LayoutBlock& ng_block,
                                      LayoutUnit inline_size);

  void Trace(Visitor*) const;

  class LayoutScope {
    STACK_ALLOCATED();

   public:
    explicit LayoutScope(LayoutBlock*);
    ~LayoutScope();

   protected:
    TextAutosizer* text_autosizer_;
    LayoutBlock* block_;
  };

  class TableLayoutScope : LayoutScope {
    STACK_ALLOCATED();

   public:
    explicit TableLayoutScope(LayoutTable*);
  };

  class NGLayoutScope {
    STACK_ALLOCATED();

   public:
    explicit NGLayoutScope(LayoutBox*, LayoutUnit inline_size);
    ~NGLayoutScope();

   protected:
    TextAutosizer* text_autosizer_;
    LayoutBlock* block_;
  };

  class CORE_EXPORT DeferUpdatePageInfo {
    STACK_ALLOCATED();

   public:
    explicit DeferUpdatePageInfo(Page*);
    ~DeferUpdatePageInfo();

   private:
    LocalFrame* main_frame_;
  };

 private:
  using BlockSet = GCedHeapHashSet<Member<LayoutBlock>>;
  using ConstBlockSet = GCedHeapHashSet<Member<const LayoutBlock>>;

  enum HasEnoughTextToAutosize {
    kUnknownAmountOfText,
    kHasEnoughText,
    kNotEnoughText
  };

  enum RelayoutBehavior {
    kAlreadyInLayout,  // The default; appropriate if we are already in layout.
    kLayoutNeeded      // Use this if changing a multiplier outside of layout.
  };

  enum BeginLayoutBehavior { kStopLayout, kContinueLayout };

  enum InflateBehavior { kThisBlockOnly, kDescendToInnerBlocks };

  enum BlockFlag {
    // A block that is evaluated for becoming a cluster root.
    POTENTIAL_ROOT = 1 << 0,
    // A cluster root that establishes an independent multiplier.
    INDEPENDENT = 1 << 1,
    // A cluster root with an explicit width. These are likely to be
    // independent.
    EXPLICIT_WIDTH = 1 << 2,
    // A cluster that is wider or narrower than its parent. These also create an
    // independent multiplier, but this state cannot be determined until layout.
    WIDER_OR_NARROWER = 1 << 3,
    // A cluster that suppresses autosizing.
    SUPPRESSING = 1 << 4
  };

  enum InheritParentMultiplier {
    kUnknown,
    kInheritMultiplier,
    kDontInheritMultiplier
  };

  typedef unsigned BlockFlags;

  // A supercluster represents autosizing information about a set of two or
  // more blocks that all have the same fingerprint. Clusters whose roots
  // belong to a supercluster will share a common multiplier and
  // text-length-based autosizing status.
  struct Supercluster : public GarbageCollected<Supercluster> {
   public:
    explicit Supercluster(const BlockSet* roots)
        : roots_(roots),
          has_enough_text_to_autosize_(kUnknownAmountOfText),
          multiplier_(0),
          inherit_parent_multiplier_(kUnknown) {}
    void Trace(Visitor*) const;

    Member<const BlockSet> roots_;
    HasEnoughTextToAutosize has_enough_text_to_autosize_;
    float multiplier_;
    InheritParentMultiplier inherit_parent_multiplier_;
  };

  struct Cluster : public GarbageCollected<Cluster> {
   public:
    explicit Cluster(const LayoutBlock* root,
                     BlockFlags,
                     Cluster* parent,
                     Supercluster* = nullptr);

    void Trace(Visitor*) const;

    Member<const LayoutBlock> const root_;
    BlockFlags flags_;
    // The deepest block containing all text is computed lazily (see:
    // deepestBlockContainingAllText). A value of 0 indicates the value has not
    // been computed yet.
    Member<const LayoutBlock> deepest_block_containing_all_text_;
    Member<Cluster> parent_;
    // The multiplier is computed lazily (see: clusterMultiplier) because it
    // must be calculated after the lowest block containing all text has entered
    // layout (the m_blocksThatHaveBegunLayout assertions cover this). Note: the
    // multiplier is still calculated when m_autosize is false because child
    // clusters may depend on this multiplier.
    float multiplier_;
    HasEnoughTextToAutosize has_enough_text_to_autosize_;
    // A set of blocks that are similar to this block.
    Member<Supercluster> supercluster_;
    bool has_table_ancestor_;
  };

  enum TextLeafSearch { kFirst, kLast };

  typedef unsigned Fingerprint;
  typedef HeapVector<Member<Cluster>> ClusterStack;

  // Fingerprints are computed during style recalc, for (some subset of)
  // blocks that will become cluster roots.
  // Clusters whose roots share the same fingerprint use the same multiplier
  class FingerprintMapper {
    DISALLOW_NEW();

   public:
    void Add(LayoutObject*, Fingerprint);
    void AddTentativeClusterRoot(LayoutBlock*, Fingerprint);
    // Returns true if any BlockSet was modified or freed by the removal.
    bool Remove(LayoutObject*);
    Fingerprint Get(const LayoutObject*);
    BlockSet* GetTentativeClusterRoots(Fingerprint);
    Supercluster* CreateSuperclusterIfNeeded(LayoutBlock*, bool& is_new_entry);
    bool HasFingerprints() const { return !fingerprints_.empty(); }
    HeapHashSet<Member<Supercluster>>&
    GetPotentiallyInconsistentSuperclusters() {
      return potentially_inconsistent_superclusters_;
    }
    void Trace(Visitor* visitor) const;

   private:
    typedef HeapHashMap<Member<const LayoutObject>, Fingerprint> FingerprintMap;
    typedef HeapHashMap<Fingerprint, Member<BlockSet>> ReverseFingerprintMap;
    typedef HeapHashMap<Fingerprint, Member<Supercluster>> SuperclusterMap;

    FingerprintMap fingerprints_;
    ReverseFingerprintMap blocks_for_fingerprint_;
    // Maps fingerprints to superclusters. Superclusters persist across layouts.
    SuperclusterMap superclusters_;
    // Superclusters that need to be checked for consistency at the start of the
    // next layout.
    HeapHashSet<Member<Supercluster>> potentially_inconsistent_superclusters_;
#if DCHECK_IS_ON()
    void AssertMapsAreConsistent();
#endif
  };

  struct PageInfo {
    DISALLOW_NEW();
    PageInfo() = default;

    mojom::blink::TextAutosizerPageInfo shared_info_;
    float accessibility_font_scale_factor_;
    bool page_needs_autosizing_;
    bool has_autosized_;
    bool setting_enabled_;
  };

  void BeginLayout(LayoutBlock*);
  void EndLayout(LayoutBlock*);
  void RegisterInlineSize(const LayoutBlock& ng_block, LayoutUnit inline_size);
  void UnregisterInlineSize(const LayoutBlock& ng_block);
  void InflateAutoTable(LayoutTable*);
  float Inflate(LayoutObject*,
                InflateBehavior = kThisBlockOnly,
                float multiplier = 0);
  bool ShouldHandleLayout() const;
  gfx::Size WindowSize() const;
  void SetAllTextNeedsLayout(LayoutBlock* container = nullptr);
  void ResetMultipliers();
  BeginLayoutBehavior PrepareForLayout(LayoutBlock*);
  void PrepareClusterStack(LayoutObject*);
  bool ClusterHasEnoughTextToAutosize(
      Cluster*,
      const LayoutBlock* width_provider = nullptr);
  bool SuperclusterHasEnoughTextToAutosize(
      Supercluster*,
      const LayoutBlock* width_provider = nullptr,
      bool skip_layouted_nodes = false);
  bool ClusterWouldHaveEnoughTextToAutosize(
      const LayoutBlock* root,
      const LayoutBlock* width_provider = nullptr);
  Fingerprint GetFingerprint(LayoutObject*);
  Fingerprint ComputeFingerprint(const LayoutObject*);
  Cluster* MaybeCreateCluster(LayoutBlock*);
  float ClusterMultiplier(Cluster*);
  float SuperclusterMultiplier(Cluster*);
  // A cluster's width provider is typically the deepest block containing all
  // text. There are exceptions, such as tables and table cells which use the
  // table itself for width.
  const LayoutBlock* ClusterWidthProvider(const LayoutBlock*) const;
  const LayoutBlock* MaxClusterWidthProvider(
      Supercluster*,
      const LayoutBlock* current_root) const;
  // Typically this returns a block's computed width. In the case of tables
  // layout, this width is not yet known so the fixed width is used if it's
  // available, or the containing block's width otherwise.
  float WidthFromBlock(const LayoutBlock*) const;
  float MultiplierFromBlock(const LayoutBlock*);
  void ApplyMultiplier(LayoutObject*,
                       float,
                       RelayoutBehavior = kAlreadyInLayout);
  bool IsWiderOrNarrowerDescendant(Cluster*);
  Cluster* CurrentCluster() const;
  const LayoutBlock* DeepestBlockContainingAllText(Cluster*);
  const LayoutBlock* DeepestBlockContainingAllText(const LayoutBlock*) const;
  // Returns the first text leaf that is in the current cluster. We attempt to
  // not include text from descendant clusters but because descendant clusters
  // may not exist, this is only an approximation.  The TraversalDirection
  // controls whether we return the first or the last text leaf.
  const LayoutObject* FindTextLeaf(const LayoutObject*,
                                   size_t&,
                                   TextLeafSearch) const;
  BlockFlags ClassifyBlock(const LayoutObject*,
                           BlockFlags mask = UINT_MAX) const;
  // Must be called at the start of layout.
  void CheckSuperclusterConsistency();
  // Mark the nearest non-inheritance supercluser
  void MarkSuperclusterForConsistencyCheck(LayoutObject*);

  void ReportIfCrossSiteFrame();

  float ContentInlineSize(const LayoutBlock* block) const;

  Member<const Document> document_;
  Member<const LayoutBlock> first_block_to_begin_layout_;
  // WeakMember because we don't call UnregisterInlineSize() for
  // LayoutMultiColumnFlowThread.
  HeapHashMap<WeakMember<const LayoutBlock>, LayoutUnit> inline_size_map_;

#if DCHECK_IS_ON()
  // Used to ensure we don't compute properties of a block before beginLayout()
  // is called on it.
  HeapHashSet<Member<const LayoutBlock>> blocks_that_have_begun_layout_;
#endif

  // Clusters are created and destroyed during layout
  ClusterStack cluster_stack_;
  FingerprintMapper fingerprint_mapper_;
  // FIXME: All frames should share the same m_pageInfo instance.
  PageInfo page_info_;
  bool update_page_info_deferred_;

  // Inflate reports a use counter if we're autosizing a cross site iframe.
  // This flag makes sure we only check it once per layout pass.
  bool did_check_cross_site_use_count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TEXT_AUTOSIZER_H_
