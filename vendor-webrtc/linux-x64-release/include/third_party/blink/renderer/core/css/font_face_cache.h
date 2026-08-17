/*
 * Copyright (C) 2007, 2008, 2011 Apple Inc. All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_CACHE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_segmented_font_face.h"
#include "third_party/blink/renderer/core/css/font_face.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/platform/fonts/font_selection_types.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/linked_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class FontDescription;

class CORE_EXPORT FontFaceCache final : public GarbageCollected<FontFaceCache> {
 public:
  FontFaceCache();

  void Add(const StyleRuleFontFace*, FontFace*);
  void Remove(const StyleRuleFontFace*);
  // Returns true if at least one font was removed.
  bool ClearCSSConnected();
  void ClearAll();
  void AddFontFace(FontFace*, bool css_connected);
  void RemoveFontFace(FontFace*, bool css_connected);

  size_t GetNumSegmentedFacesForTesting();

  // FIXME: It's sort of weird that add/remove uses StyleRuleFontFace* as key,
  // but this function uses FontDescription/family pair.
  CSSSegmentedFontFace* Get(const FontDescription&, const AtomicString& family);

  const HeapLinkedHashSet<Member<FontFace>>& CssConnectedFontFaces() const {
    return css_connected_font_faces_;
  }

  unsigned Version() const { return version_; }
  void IncrementVersion();

  void Trace(Visitor*) const;

 private:
  // Two lookup accelerating caches are needed: For the font selection
  // algorithm to work and not perform font fallback across
  // FontSelectionCapabilities, we need to bin the incoming @font-faces by same
  // FontSelectionCapabilities, then run the font selection algorithm only by
  // looking at the capabilities of each group. Group here means: Font faces of
  // identicaly capabilities, but differing unicode-range.
  //
  // A second lookup table caches the previously received FontSelectionRequest
  // queries, which is: HeapHashMap <String, HeapHashMap<FontSelectionRequest,
  // CSSSegmentedFontFace>>
  class CapabilitiesSet final : public GarbageCollected<CapabilitiesSet> {
    using Map =
        HeapHashMap<FontSelectionCapabilities, Member<CSSSegmentedFontFace>>;

   public:
    Map::const_iterator begin() const { return map_.begin(); }
    Map::const_iterator end() const { return map_.end(); }
    wtf_size_t size() const { return map_.size(); }

    void AddFontFace(FontFace* font_face, bool css_connected);
    bool IsEmpty() const { return map_.empty(); }

    // Returns true if associated |CSSSegmentedFontFace| is empty.
    bool RemoveFontFace(FontFace* font_face);

    void Trace(Visitor*) const;

   private:
    Map map_;
  };

  // The map from |FontSelectionRequestKey| to |CSSSegmentedFontFace|.
  class FontSelectionQueryResult final
      : public GarbageCollected<FontSelectionQueryResult> {
    using Map =
        HeapHashMap<FontSelectionRequestKey, Member<CSSSegmentedFontFace>>;

   public:
    CSSSegmentedFontFace* GetOrCreate(const FontSelectionRequest& request,
                                      const CapabilitiesSet& family_faces);

    void Trace(Visitor*) const;

   private:
    Map map_;
  };

  // The map from font family name to |FontSelectionQueryResult|.
  class FontSelectionQueryCache final {
    DISALLOW_NEW();

    using Map = HeapHashMap<String,
                            Member<FontSelectionQueryResult>,
                            CaseFoldingHashTraits<String>>;

   public:
    void Clear();
    CSSSegmentedFontFace* GetOrCreate(const FontSelectionRequest& request,
                                      const AtomicString& family,
                                      CapabilitiesSet* family_faces);
    void Remove(const AtomicString& family);

    void Trace(Visitor*) const;

   private:
    Map map_;
  };

  // The map from font family name to |CapabilitiesSet|.
  class SegmentedFacesByFamily final {
    DISALLOW_NEW();

   public:
    void AddFontFace(FontFace* font_face, bool css_connected);
    void Clear() { map_.clear(); }
    CapabilitiesSet* Find(const AtomicString& family) const;
    bool IsEmpty() const { return map_.empty(); }
    // Returns true if |font_face| is removed from |map_|.
    bool RemoveFontFace(FontFace* font_face);

    size_t GetNumSegmentedFacesForTesting() const;

    void Trace(Visitor*) const;

   private:
    using Map = HeapHashMap<String,
                            Member<CapabilitiesSet>,
                            CaseFoldingHashTraits<String>>;

    Map map_;
  };

  // All incoming faces added from JS or CSS, bucketed per family.
  SegmentedFacesByFamily segmented_faces_;
  // Previously determined font matching query results, bucketed per family and
  // FontSelectionRequest. A family bucket of this cache gets invalidated when a
  // new face of the same family is added or removed.
  FontSelectionQueryCache font_selection_query_cache_;

  // Used for removing font faces from the segmented_faces_ list when a CSS rule
  // is removed.
  using StyleRuleToFontFace =
      HeapHashMap<Member<const StyleRuleFontFace>, Member<FontFace>>;
  StyleRuleToFontFace style_rule_to_font_face_;

  // Needed for incoming ClearCSSConnected() requests coming in from
  // StyleEngine, which clears all those faces from the FontCache which are
  // originating from CSS, as opposed to those originating from JS.
  HeapLinkedHashSet<Member<FontFace>> css_connected_font_faces_;

  // FIXME: See if this could be ditched
  // Used to compare Font instances, and the usage seems suspect.
  unsigned version_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_CACHE_H_
