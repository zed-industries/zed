// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_PRIVACY_BUDGET_FONT_INDEXER_FONT_INDEXER_H_
#define TOOLS_PRIVACY_BUDGET_FONT_INDEXER_FONT_INDEXER_H_

#include <string>
#include <utility>

#include "base/functional/callback.h"
#include "base/values.h"
#include "third_party/blink/renderer/platform/fonts/font_cache.h"
#include "third_party/blink/renderer/platform/fonts/font_description.h"
#include "third_party/blink/renderer/platform/fonts/font_selection_types.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace privacy_budget {

// Note that in the following constants, we prioritize the more common values as
// we later associate each unique digest with the first set of settings for
// which that digest is found.
extern const std::pair<blink::FontSelectionValue, std::string> kFontWeights[];
extern const std::pair<blink::FontSelectionValue, std::string> kFontWidths[];

// Not as thorough as above, given its rarity and to reduce the speed impact.
extern const std::pair<blink::FontSelectionValue, std::string> kFontSlopes[];

// Following only used if |more_slope_checks_|
extern const std::pair<blink::FontSelectionValue, std::string>
    kAdditionalFontSlopes[];

extern const char kOutputHeader[];
extern const char kOutputSeparator[];

// FontIndexer allows for enumerating all locally installed fonts and computing
// identifiability digests of each typeface.
class FontIndexer {
 public:
  FontIndexer();
  ~FontIndexer();

  // The main function that enumerates all fonts and prints a tab-separated file
  // containing the fonts' details to stdout.
  void PrintAllFonts();

  // By default, this tool attempts to determine whether the fonts vary along
  // each axis (i.e. width, weight and slope), skipping checks along the axes
  // with no variation. This call disables this optimization, slowing the tool
  // substantially, but possibly being more thorough (if the determinations are
  // incorrect).
  void SetNoSmartSkipping() { smart_skipping_ = false; }

  // By default, the tool only checks a limited number of slope values as
  // substantial slope variation is rare and slow to check for. This call adds
  // more granularity when slopes are varied. This will slow down the tool, but
  // will give more results if a font with many slope variations is available.
  void SetMoreSlopeChecks() { more_slope_checks_ = true; }

 private:
  void FontListHasLoaded(base::Value::List list);
  void WaitForFontListToLoad();

  // Determines whether the fonts with |name| appear to vary along the specified
  // axis, by comparing font digests at extreme values to |default_font_digest|.
  bool DoFontsWithNameHaveVaryingWeights(WTF::AtomicString name,
                                         int64_t default_font_digest);
  bool DoFontsWithNameHaveVaryingWidths(WTF::AtomicString name,
                                        int64_t default_font_digest);
  bool DoFontsWithNameHaveVaryingSlopes(WTF::AtomicString name,
                                        int64_t default_font_digest);

  // Determines whether a font lookup for |name| with |font_description| results
  // in a typeface with |digest|.
  bool DoesFontHaveDigest(WTF::AtomicString name,
                          blink::FontDescription font_description,
                          int64_t digest);

  // Enumerates fonts with |name| and prints tab-separated lines with each
  // font's details.
  void PrintAllFontsWithName(WTF::AtomicString name);

  blink::FontCache* font_cache_;
  bool has_font_list_loaded_ = false;
  bool smart_skipping_ = true;
  bool more_slope_checks_ = false;
  base::OnceClosure quit_closure_;
};

}  // namespace privacy_budget

#endif  // TOOLS_PRIVACY_BUDGET_FONT_INDEXER_FONT_INDEXER_H_
