//! Shaping support for autohinting.

use super::style::{GlyphStyle, StyleClass};
use crate::{charmap::Charmap, collections::SmallVec, FontRef, GlyphId, MetadataProvider};
use core::ops::Range;
use raw::{
    tables::{
        gsub::{
            ChainedSequenceContext, Gsub, SequenceContext, SingleSubst, SubstitutionLookupList,
            SubstitutionSubtables,
        },
        layout::{Feature, ScriptTags},
        varc::CoverageTable,
    },
    types::Tag,
    ReadError, TableProvider,
};

// To prevent infinite recursion in contextual lookups. Matches HB
// <https://github.com/harfbuzz/harfbuzz/blob/c7ef6a2ed58ae8ec108ee0962bef46f42c73a60c/src/hb-limits.hh#L53>
const MAX_NESTING_DEPTH: usize = 64;

/// Determines the fidelity with which we apply shaping in the
/// autohinter.
///
/// Shaping only affects glyph style classification and the glyphs that
/// are chosen for metrics computations. We keep the `Nominal` mode around
/// to enable validation of internal algorithms against a configuration that
/// is known to match FreeType. The `BestEffort` mode should always be
/// used for actual rendering.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum ShaperMode {
    /// Characters are mapped to nominal glyph identifiers and layout tables
    /// are not used for style coverage.
    ///
    /// This matches FreeType when HarfBuzz support is not enabled.
    Nominal,
    /// Simple substitutions are applied according to script rules and layout
    /// tables are used to extend style coverage beyond the character map.
    #[allow(unused)]
    BestEffort,
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct ShapedGlyph {
    pub id: GlyphId,
    /// This may be used for computing vertical alignment zones, particularly
    /// for glyphs like super/subscripts which might have adjustments in GPOS.
    ///
    /// Note that we don't do the same in the horizontal direction which
    /// means that we don't care about the x-offset.
    pub y_offset: i32,
}

/// Arbitrarily chosen to cover our max input size plus some extra to account
/// for expansion from multiple substitution tables.
const SHAPED_CLUSTER_INLINE_SIZE: usize = 16;

/// Container for storing the result of shaping a cluster.
///
/// Some of our input "characters" for metrics computations are actually
/// multi-character [grapheme clusters](https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)
/// that may expand to multiple glyphs.
pub(crate) type ShapedCluster = SmallVec<ShapedGlyph, SHAPED_CLUSTER_INLINE_SIZE>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum ShaperCoverageKind {
    /// Shaper coverage that traverses a specific script.
    Script,
    /// Shaper coverage that also includes the `Dflt` script.
    ///
    /// This is used as a catch all after all styles are processed.
    Default,
}

/// Maps characters to glyphs and handles extended style coverage beyond
/// glyphs that are available in the character map.
///
/// Roughly covers the functionality in <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afshaper.c>.
pub(crate) struct Shaper<'a> {
    font: FontRef<'a>,
    #[allow(unused)]
    mode: ShaperMode,
    charmap: Charmap<'a>,
    gsub: Option<Gsub<'a>>,
}

impl<'a> Shaper<'a> {
    pub fn new(font: &FontRef<'a>, mode: ShaperMode) -> Self {
        let charmap = font.charmap();
        let gsub = (mode != ShaperMode::Nominal)
            .then(|| font.gsub().ok())
            .flatten();
        Self {
            font: font.clone(),
            mode,
            charmap,
            gsub,
        }
    }

    pub fn font(&self) -> &FontRef<'a> {
        &self.font
    }

    pub fn charmap(&self) -> &Charmap<'a> {
        &self.charmap
    }

    pub fn lookup_count(&self) -> u16 {
        self.gsub
            .as_ref()
            .and_then(|gsub| gsub.lookup_list().ok())
            .map(|list| list.lookup_count())
            .unwrap_or_default()
    }

    pub fn cluster_shaper(&'a self, style: &StyleClass) -> ClusterShaper<'a> {
        if self.mode == ShaperMode::BestEffort {
            // For now, only apply substitutions for styles with an associated
            // feature
            if let Some(feature_tag) = style.feature {
                if let Some((lookup_list, feature)) = self.gsub.as_ref().and_then(|gsub| {
                    let script_list = gsub.script_list().ok()?;
                    let selected_script =
                        script_list.select(&ScriptTags::from_unicode(style.script.tag))?;
                    let script = script_list.get(selected_script.index).ok()?;
                    let lang_sys = script.default_lang_sys()?.ok()?;
                    let feature_list = gsub.feature_list().ok()?;
                    let feature_ix = lang_sys.feature_index_for_tag(&feature_list, feature_tag)?;
                    let feature = feature_list.get(feature_ix).ok()?.element;
                    let lookup_list = gsub.lookup_list().ok()?;
                    Some((lookup_list, feature))
                }) {
                    return ClusterShaper {
                        shaper: self,
                        lookup_list: Some(lookup_list),
                        kind: ClusterShaperKind::SingleFeature(feature),
                    };
                }
            }
        }
        ClusterShaper {
            shaper: self,
            lookup_list: None,
            kind: ClusterShaperKind::Nominal,
        }
    }

    /// Uses layout tables to compute coverage for the given style.
    ///
    /// Returns `true` if any glyph styles were updated for this style.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afshaper.c#L99>
    pub(crate) fn compute_coverage(
        &self,
        style: &StyleClass,
        coverage_kind: ShaperCoverageKind,
        glyph_styles: &mut [GlyphStyle],
        visited_set: &mut VisitedLookupSet<'_>,
    ) -> bool {
        let Some(gsub) = self.gsub.as_ref() else {
            return false;
        };
        let (Ok(script_list), Ok(feature_list), Ok(lookup_list)) =
            (gsub.script_list(), gsub.feature_list(), gsub.lookup_list())
        else {
            return false;
        };
        let mut script_tags: [Option<Tag>; 3] = [None; 3];
        for (a, b) in script_tags
            .iter_mut()
            .zip(ScriptTags::from_unicode(style.script.tag).iter())
        {
            *a = Some(*b);
        }
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afshaper.c#L153>
        const DEFAULT_SCRIPT: Tag = Tag::new(b"Dflt");
        if coverage_kind == ShaperCoverageKind::Default {
            if script_tags[0].is_none() {
                script_tags[0] = Some(DEFAULT_SCRIPT);
            } else if script_tags[1].is_none() {
                script_tags[1] = Some(DEFAULT_SCRIPT);
            } else if script_tags[1] != Some(DEFAULT_SCRIPT) {
                script_tags[2] = Some(DEFAULT_SCRIPT);
            }
        } else {
            // Script classes contain some non-standard tags used for special
            // purposes. We ignore these
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afshaper.c#L167>
            const NON_STANDARD_TAGS: &[Option<Tag>] = &[
                // Khmer symbols
                Some(Tag::new(b"Khms")),
                // Latin subscript fallbacks
                Some(Tag::new(b"Latb")),
                // Latin superscript fallbacks
                Some(Tag::new(b"Latp")),
            ];
            if NON_STANDARD_TAGS.contains(&script_tags[0]) {
                return false;
            }
        }
        // Check each requested script that is available in GSUB
        let mut gsub_handler = GsubHandler::new(
            &self.charmap,
            &lookup_list,
            style,
            glyph_styles,
            visited_set,
        );
        for script in script_tags.iter().filter_map(|tag| {
            tag.and_then(|tag| script_list.index_for_tag(tag))
                .and_then(|ix| script_list.script_records().get(ix as usize))
                .and_then(|rec| rec.script(script_list.offset_data()).ok())
        }) {
            // And all language systems for each script
            for langsys in script
                .lang_sys_records()
                .iter()
                .filter_map(|rec| rec.lang_sys(script.offset_data()).ok())
                .chain(script.default_lang_sys().transpose().ok().flatten())
            {
                for feature_ix in langsys.feature_indices() {
                    let Some(feature) = feature_list
                        .feature_records()
                        .get(feature_ix.get() as usize)
                        .and_then(|rec| {
                            // If our style has a feature tag, we only look at that specific
                            // feature; otherwise, handle all of them
                            if style.feature == Some(rec.feature_tag()) || style.feature.is_none() {
                                rec.feature(feature_list.offset_data()).ok()
                            } else {
                                None
                            }
                        })
                    else {
                        continue;
                    };
                    // And now process associated lookups
                    for index in feature.lookup_list_indices().iter() {
                        // We only care about errors here for testing
                        let _ = gsub_handler.process_lookup(index.get());
                    }
                }
            }
        }
        if let Some(range) = gsub_handler.finish() {
            // If we get a range then we captured at least some glyphs so
            // let's try to assign our current style
            let mut result = false;
            for glyph_style in &mut glyph_styles[range] {
                // We only want to return true here if we actually assign the
                // style to avoid computing unnecessary metrics
                result |= glyph_style.maybe_assign_gsub_output_style(style);
            }
            result
        } else {
            false
        }
    }
}

pub(crate) struct ClusterShaper<'a> {
    shaper: &'a Shaper<'a>,
    lookup_list: Option<SubstitutionLookupList<'a>>,
    kind: ClusterShaperKind<'a>,
}

impl ClusterShaper<'_> {
    pub(crate) fn shape(&mut self, input: &str, output: &mut ShapedCluster) {
        // First fill the output cluster with the nominal character
        // to glyph id mapping
        output.clear();
        for ch in input.chars() {
            output.push(ShapedGlyph {
                id: self.shaper.charmap.map(ch).unwrap_or_default(),
                y_offset: 0,
            });
        }
        match self.kind.clone() {
            ClusterShaperKind::Nominal => {
                // In nominal mode, reject clusters with multiple glyphs
                // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afshaper.c#L639>
                if self.shaper.mode == ShaperMode::Nominal && output.len() > 1 {
                    output.clear();
                }
            }
            ClusterShaperKind::SingleFeature(feature) => {
                let mut did_subst = false;
                for lookup_ix in feature.lookup_list_indices() {
                    let mut glyph_ix = 0;
                    while glyph_ix < output.len() {
                        did_subst |= self.apply_lookup(lookup_ix.get(), output, glyph_ix, 0);
                        glyph_ix += 1;
                    }
                }
                // Reject clusters that weren't modified by the feature.
                // FreeType detects this by shaping twice and comparing gids
                // but we just track substitutions
                // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afshaper.c#L528>
                if !did_subst {
                    output.clear();
                }
            }
        }
    }

    fn apply_lookup(
        &self,
        lookup_index: u16,
        cluster: &mut ShapedCluster,
        glyph_ix: usize,
        nesting_depth: usize,
    ) -> bool {
        if nesting_depth > MAX_NESTING_DEPTH {
            return false;
        }
        let Some(glyph) = cluster.get_mut(glyph_ix) else {
            return false;
        };
        let Some(subtables) = self
            .lookup_list
            .as_ref()
            .and_then(|list| list.lookups().get(lookup_index as usize).ok())
            .and_then(|lookup| lookup.subtables().ok())
        else {
            return false;
        };
        match subtables {
            // For now, just applying single substitutions because we're
            // currently only handling shaping for "feature" styles like
            // c2sc (caps to small caps) which are (almost?) always
            // single substs
            SubstitutionSubtables::Single(tables) => {
                for table in tables.iter().filter_map(|table| table.ok()) {
                    match table {
                        SingleSubst::Format1(table) => {
                            let Some(_) = table.coverage().ok().and_then(|cov| cov.get(glyph.id))
                            else {
                                continue;
                            };
                            let delta = table.delta_glyph_id() as i32;
                            glyph.id = GlyphId::from((glyph.id.to_u32() as i32 + delta) as u16);
                            return true;
                        }
                        SingleSubst::Format2(table) => {
                            let Some(cov_ix) =
                                table.coverage().ok().and_then(|cov| cov.get(glyph.id))
                            else {
                                continue;
                            };
                            let Some(subst) = table.substitute_glyph_ids().get(cov_ix as usize)
                            else {
                                continue;
                            };
                            glyph.id = subst.get().into();
                            return true;
                        }
                    }
                }
            }
            SubstitutionSubtables::Multiple(_tables) => {}
            SubstitutionSubtables::Ligature(_tables) => {}
            SubstitutionSubtables::Alternate(_tables) => {}
            SubstitutionSubtables::Contextual(_tables) => {}
            SubstitutionSubtables::ChainContextual(_tables) => {}
            SubstitutionSubtables::Reverse(_tables) => {}
        }
        false
    }
}

#[derive(Clone)]
enum ClusterShaperKind<'a> {
    Nominal,
    SingleFeature(Feature<'a>),
}

/// Captures glyphs from the GSUB table that aren't present in cmap.
///
/// FreeType does this in a few phases:
/// 1. Collect all lookups for a given set of scripts and features.
///    <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afshaper.c#L174>
/// 2. For each lookup, collect all _output_ glyphs.
///    <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afshaper.c#L201>
/// 3. If the style represents a specific feature, make sure at least one of
///    the characters in the associated blue string would be substituted by
///    those lookups. If none would be substituted, then we don't assign the
///    style to any glyphs because we don't have any modified alignment
///    zones.
///    <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afshaper.c#L264>
///
/// We roll these into one pass over the lookups below so that we don't have
/// to allocate a lookup set or iterate them twice. Note that since
/// substitutions are checked for individual characters, we ignore ligatures
/// and contextual lookups (and alternates since they aren't applicable).
struct GsubHandler<'a, 'b> {
    charmap: &'a Charmap<'a>,
    lookup_list: &'a SubstitutionLookupList<'a>,
    style: &'a StyleClass,
    glyph_styles: &'a mut [GlyphStyle],
    // Set to true when we need to check if any substitutions are available
    // for our blue strings. This is the case when style.feature != None
    need_blue_substs: bool,
    // Keep track of our range of touched gids in the style list
    min_gid: usize,
    max_gid: usize,
    lookup_depth: usize,
    visited_set: &'a mut VisitedLookupSet<'b>,
}

impl<'a, 'b> GsubHandler<'a, 'b> {
    fn new(
        charmap: &'a Charmap<'a>,
        lookup_list: &'a SubstitutionLookupList,
        style: &'a StyleClass,
        glyph_styles: &'a mut [GlyphStyle],
        visited_set: &'a mut VisitedLookupSet<'b>,
    ) -> Self {
        let min_gid = glyph_styles.len();
        // If we have a feature, then we need to check the blue string to see
        // if any substitutions are available. If not, we don't enable this
        // style because it won't have any affect on alignment zones
        let need_blue_substs = style.feature.is_some();
        Self {
            charmap,
            lookup_list,
            style,
            glyph_styles,
            need_blue_substs,
            min_gid,
            max_gid: 0,
            lookup_depth: 0,
            visited_set,
        }
    }

    fn process_lookup(&mut self, lookup_index: u16) -> Result<(), ProcessLookupError> {
        // General protection against stack overflows
        if self.lookup_depth == MAX_NESTING_DEPTH {
            return Err(ProcessLookupError::ExceededMaxDepth);
        }
        // Skip lookups that have already been processed
        if !self.visited_set.insert(lookup_index) {
            return Ok(());
        }
        self.lookup_depth += 1;
        // Actually process the lookup
        let result = self.process_lookup_inner(lookup_index);
        // Out we go again
        self.lookup_depth -= 1;
        result
    }

    #[inline(always)]
    fn process_lookup_inner(&mut self, lookup_index: u16) -> Result<(), ProcessLookupError> {
        let Ok(subtables) = self
            .lookup_list
            .lookups()
            .get(lookup_index as usize)
            .and_then(|lookup| lookup.subtables())
        else {
            return Ok(());
        };
        match subtables {
            SubstitutionSubtables::Single(tables) => {
                for table in tables.iter().filter_map(|table| table.ok()) {
                    match table {
                        SingleSubst::Format1(table) => {
                            let Ok(coverage) = table.coverage() else {
                                continue;
                            };
                            let delta = table.delta_glyph_id() as i32;
                            for gid in coverage.iter() {
                                self.capture_glyph((gid.to_u32() as i32 + delta) as u16 as u32);
                            }
                            // Check input coverage for blue strings if
                            // required and if we're not under a contextual
                            // lookup
                            if self.need_blue_substs && self.lookup_depth == 1 {
                                self.check_blue_coverage(Ok(coverage));
                            }
                        }
                        SingleSubst::Format2(table) => {
                            for gid in table.substitute_glyph_ids() {
                                self.capture_glyph(gid.get().to_u32());
                            }
                            // See above
                            if self.need_blue_substs && self.lookup_depth == 1 {
                                self.check_blue_coverage(table.coverage());
                            }
                        }
                    }
                }
            }
            SubstitutionSubtables::Multiple(tables) => {
                for table in tables.iter().filter_map(|table| table.ok()) {
                    for seq in table.sequences().iter().filter_map(|seq| seq.ok()) {
                        for gid in seq.substitute_glyph_ids() {
                            self.capture_glyph(gid.get().to_u32());
                        }
                    }
                    // See above
                    if self.need_blue_substs && self.lookup_depth == 1 {
                        self.check_blue_coverage(table.coverage());
                    }
                }
            }
            SubstitutionSubtables::Ligature(tables) => {
                for table in tables.iter().filter_map(|table| table.ok()) {
                    for set in table.ligature_sets().iter().filter_map(|set| set.ok()) {
                        for lig in set.ligatures().iter().filter_map(|lig| lig.ok()) {
                            self.capture_glyph(lig.ligature_glyph().to_u32());
                        }
                    }
                }
            }
            SubstitutionSubtables::Alternate(tables) => {
                for table in tables.iter().filter_map(|table| table.ok()) {
                    for set in table.alternate_sets().iter().filter_map(|set| set.ok()) {
                        for gid in set.alternate_glyph_ids() {
                            self.capture_glyph(gid.get().to_u32());
                        }
                    }
                }
            }
            SubstitutionSubtables::Contextual(tables) => {
                for table in tables.iter().filter_map(|table| table.ok()) {
                    match table {
                        SequenceContext::Format1(table) => {
                            for set in table
                                .seq_rule_sets()
                                .iter()
                                .filter_map(|set| set.transpose().ok().flatten())
                            {
                                for rule in set.seq_rules().iter().filter_map(|rule| rule.ok()) {
                                    for rec in rule.seq_lookup_records() {
                                        self.process_lookup(rec.lookup_list_index())?;
                                    }
                                }
                            }
                        }
                        SequenceContext::Format2(table) => {
                            for set in table
                                .class_seq_rule_sets()
                                .iter()
                                .filter_map(|set| set.transpose().ok().flatten())
                            {
                                for rule in
                                    set.class_seq_rules().iter().filter_map(|rule| rule.ok())
                                {
                                    for rec in rule.seq_lookup_records() {
                                        self.process_lookup(rec.lookup_list_index())?;
                                    }
                                }
                            }
                        }
                        SequenceContext::Format3(table) => {
                            for rec in table.seq_lookup_records() {
                                self.process_lookup(rec.lookup_list_index())?;
                            }
                        }
                    }
                }
            }
            SubstitutionSubtables::ChainContextual(tables) => {
                for table in tables.iter().filter_map(|table| table.ok()) {
                    match table {
                        ChainedSequenceContext::Format1(table) => {
                            for set in table
                                .chained_seq_rule_sets()
                                .iter()
                                .filter_map(|set| set.transpose().ok().flatten())
                            {
                                for rule in
                                    set.chained_seq_rules().iter().filter_map(|rule| rule.ok())
                                {
                                    for rec in rule.seq_lookup_records() {
                                        self.process_lookup(rec.lookup_list_index())?;
                                    }
                                }
                            }
                        }
                        ChainedSequenceContext::Format2(table) => {
                            for set in table
                                .chained_class_seq_rule_sets()
                                .iter()
                                .filter_map(|set| set.transpose().ok().flatten())
                            {
                                for rule in set
                                    .chained_class_seq_rules()
                                    .iter()
                                    .filter_map(|rule| rule.ok())
                                {
                                    for rec in rule.seq_lookup_records() {
                                        self.process_lookup(rec.lookup_list_index())?;
                                    }
                                }
                            }
                        }
                        ChainedSequenceContext::Format3(table) => {
                            for rec in table.seq_lookup_records() {
                                self.process_lookup(rec.lookup_list_index())?;
                            }
                        }
                    }
                }
            }
            SubstitutionSubtables::Reverse(tables) => {
                for table in tables.iter().filter_map(|table| table.ok()) {
                    for gid in table.substitute_glyph_ids() {
                        self.capture_glyph(gid.get().to_u32());
                    }
                }
            }
        }
        Ok(())
    }

    /// Finishes processing for this set of GSUB lookups and
    /// returns the range of touched glyphs.
    fn finish(self) -> Option<Range<usize>> {
        self.visited_set.clear();
        if self.min_gid > self.max_gid {
            // We didn't touch any glyphs
            return None;
        }
        let range = self.min_gid..self.max_gid + 1;
        if self.need_blue_substs {
            // We didn't find any substitutions for our blue strings so
            // we ignore the style. Clear the GSUB marker for any touched
            // glyphs
            for glyph in &mut self.glyph_styles[range] {
                glyph.clear_from_gsub();
            }
            None
        } else {
            Some(range)
        }
    }

    /// Checks the given coverage table for any characters in the blue
    /// strings associated with our current style.
    fn check_blue_coverage(&mut self, coverage: Result<CoverageTable<'a>, ReadError>) {
        let Ok(coverage) = coverage else {
            return;
        };
        for (blue_str, _) in self.style.script.blues {
            if blue_str
                .chars()
                .filter_map(|ch| self.charmap.map(ch))
                .filter_map(|gid| coverage.get(gid))
                .next()
                .is_some()
            {
                // Condition satisfied, so don't check any further subtables
                self.need_blue_substs = false;
                return;
            }
        }
    }

    fn capture_glyph(&mut self, gid: u32) {
        let gid = gid as usize;
        if let Some(style) = self.glyph_styles.get_mut(gid) {
            style.set_from_gsub_output();
            self.min_gid = gid.min(self.min_gid);
            self.max_gid = gid.max(self.max_gid);
        }
    }
}

pub(crate) struct VisitedLookupSet<'a>(&'a mut [u8]);

impl<'a> VisitedLookupSet<'a> {
    pub fn new(storage: &'a mut [u8]) -> Self {
        Self(storage)
    }

    /// If the given lookup index is not already in the set, adds it and
    /// returns `true`. Returns `false` otherwise.
    ///
    /// This follows the behavior of `HashSet::insert`.
    fn insert(&mut self, lookup_index: u16) -> bool {
        let byte_ix = lookup_index as usize / 8;
        let bit_mask = 1 << (lookup_index % 8) as u8;
        if let Some(byte) = self.0.get_mut(byte_ix) {
            if *byte & bit_mask == 0 {
                *byte |= bit_mask;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn clear(&mut self) {
        self.0.fill(0);
    }
}

#[derive(PartialEq, Debug)]
enum ProcessLookupError {
    ExceededMaxDepth,
}

#[cfg(test)]
mod tests {
    use super::{super::style, *};
    use font_test_data::bebuffer::BeBuffer;
    use raw::{FontData, FontRead};

    #[test]
    fn small_caps_subst() {
        let font = FontRef::new(font_test_data::NOTOSERIF_AUTOHINT_SHAPING).unwrap();
        let shaper = Shaper::new(&font, ShaperMode::BestEffort);
        let style = &style::STYLE_CLASSES[style::StyleClass::LATN_C2SC];
        let mut cluster_shaper = shaper.cluster_shaper(style);
        let mut cluster = ShapedCluster::new();
        cluster_shaper.shape("H", &mut cluster);
        assert_eq!(cluster.len(), 1);
        // from ttx, gid 8 is small caps "H"
        assert_eq!(cluster[0].id, GlyphId::new(8));
    }

    #[test]
    fn small_caps_nominal() {
        let font = FontRef::new(font_test_data::NOTOSERIF_AUTOHINT_SHAPING).unwrap();
        let shaper = Shaper::new(&font, ShaperMode::Nominal);
        let style = &style::STYLE_CLASSES[style::StyleClass::LATN_C2SC];
        let mut cluster_shaper = shaper.cluster_shaper(style);
        let mut cluster = ShapedCluster::new();
        cluster_shaper.shape("H", &mut cluster);
        assert_eq!(cluster.len(), 1);
        // from ttx, gid 1 is "H"
        assert_eq!(cluster[0].id, GlyphId::new(1));
    }

    #[test]
    fn exceed_max_depth() {
        let font = FontRef::new(font_test_data::NOTOSERIF_AUTOHINT_SHAPING).unwrap();
        let shaper = Shaper::new(&font, ShaperMode::BestEffort);
        let style = &style::STYLE_CLASSES[style::StyleClass::LATN];
        // Build a lookup chain exceeding our max depth
        let mut bad_lookup_builder = BadLookupBuilder::default();
        for i in 0..MAX_NESTING_DEPTH {
            // each lookup calls the next
            bad_lookup_builder.lookups.push(i as u16 + 1);
        }
        let lookup_list_buf = bad_lookup_builder.build();
        let lookup_list = SubstitutionLookupList::read(FontData::new(&lookup_list_buf)).unwrap();
        let mut set_buf = [0u8; 8192];
        let mut visited_set = VisitedLookupSet(&mut set_buf);
        let mut gsub_handler = GsubHandler::new(
            &shaper.charmap,
            &lookup_list,
            style,
            &mut [],
            &mut visited_set,
        );
        assert_eq!(
            gsub_handler.process_lookup(0),
            Err(ProcessLookupError::ExceededMaxDepth)
        );
    }

    #[test]
    fn dont_cycle_forever() {
        let font = FontRef::new(font_test_data::NOTOSERIF_AUTOHINT_SHAPING).unwrap();
        let shaper = Shaper::new(&font, ShaperMode::BestEffort);
        let style = &style::STYLE_CLASSES[style::StyleClass::LATN];
        // Build a lookup chain that cycles; 0 calls 1 which calls 0
        let mut bad_lookup_builder = BadLookupBuilder::default();
        bad_lookup_builder.lookups.push(1);
        bad_lookup_builder.lookups.push(0);
        let lookup_list_buf = bad_lookup_builder.build();
        let lookup_list = SubstitutionLookupList::read(FontData::new(&lookup_list_buf)).unwrap();
        let mut set_buf = [0u8; 8192];
        let mut visited_set = VisitedLookupSet(&mut set_buf);
        let mut gsub_handler = GsubHandler::new(
            &shaper.charmap,
            &lookup_list,
            style,
            &mut [],
            &mut visited_set,
        );
        gsub_handler.process_lookup(0).unwrap();
    }

    #[test]
    fn visited_set() {
        let count = 2341u16;
        let n_bytes = (count as usize).div_ceil(8);
        let mut set_buf = vec![0u8; n_bytes];
        let mut set = VisitedLookupSet::new(&mut set_buf);
        for i in 0..count {
            assert!(set.insert(i));
            assert!(!set.insert(i));
        }
        for byte in &set_buf[0..set_buf.len() - 1] {
            assert_eq!(*byte, 0xFF);
        }
        assert_eq!(*set_buf.last().unwrap(), 0b00011111);
    }

    #[derive(Default)]
    struct BadLookupBuilder {
        /// Just a list of nested lookup indices for each generated lookup
        lookups: Vec<u16>,
    }

    impl BadLookupBuilder {
        fn build(&self) -> Vec<u8> {
            // Full byte size of a contextual format 3 lookup with one
            // subtable and one nested lookup
            const CONTEXT3_FULL_SIZE: usize = 18;
            let mut buf = BeBuffer::default();
            // LookupList table
            // count
            buf = buf.push(self.lookups.len() as u16);
            // offsets for each lookup
            let base_offset = 2 + 2 * self.lookups.len();
            for i in 0..self.lookups.len() {
                buf = buf.push((base_offset + i * CONTEXT3_FULL_SIZE) as u16);
            }
            // now the actual lookups
            for nested_ix in &self.lookups {
                // lookup type: GSUB contextual substitution
                buf = buf.push(5u16);
                // lookup flag
                buf = buf.push(0u16);
                // subtable count
                buf = buf.push(1u16);
                // offset to single subtable (always 8 bytes from start of lookup)
                buf = buf.push(8u16);
                // start of subtable, format == 3
                buf = buf.push(3u16);
                // number of glyphs in sequence
                buf = buf.push(0u16);
                // sequence lookup count
                buf = buf.push(1u16);
                // (no coverage offsets)
                // sequence lookup (sequence index, lookup index)
                buf = buf.push(0u16).push(*nested_ix);
            }
            buf.to_vec()
        }
    }
}
