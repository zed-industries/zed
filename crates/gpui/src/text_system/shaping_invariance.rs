use collections::FxHashSet;
use ttf_parser::{
    Face, GlyphId, RawFaceTables, Tag,
    gpos::{PairAdjustment, PositioningSubtable, SingleAdjustment},
    gsub::{SingleSubstitution, SubstitutionSubtable},
    morx,
    opentype_layout::{ChainedContextLookup, ContextLookup, Coverage, LayoutTable, LookupIndex},
};

/// Raw font tables that decide how a platform text system shapes a font.
#[derive(Default)]
pub struct ShapingTables<'a> {
    /// The `head` table.
    pub head: &'a [u8],
    /// The `hhea` table.
    pub hhea: &'a [u8],
    /// The `maxp` table.
    pub maxp: &'a [u8],
    /// The `hmtx` table.
    pub hmtx: Option<&'a [u8]>,
    /// The `GSUB` table.
    pub gsub: Option<&'a [u8]>,
    /// The `GPOS` table.
    pub gpos: Option<&'a [u8]>,
    /// The `morx` table.
    pub morx: Option<&'a [u8]>,
    /// The `kern` table.
    pub kern: Option<&'a [u8]>,
    /// The `kerx` table.
    pub kerx: Option<&'a [u8]>,
}

const DEFAULT_OFF_FEATURES: &[[u8; 4]] = &[
    *b"aalt", *b"afrc", *b"c2pc", *b"c2sc", *b"case", *b"cpsp", *b"cswh", *b"dlig", *b"dnom",
    *b"expt", *b"falt", *b"frac", *b"fwid", *b"halt", *b"hist", *b"hkna", *b"hlig", *b"hngl",
    *b"hojo", *b"hwid", *b"ital", *b"jalt", *b"jp04", *b"jp78", *b"jp83", *b"jp90", *b"lfbd",
    *b"lnum", *b"mgrk", *b"nalt", *b"nlck", *b"numr", *b"onum", *b"opbd", *b"ordn", *b"ornm",
    *b"palt", *b"pcap", *b"pkna", *b"pnum", *b"pwid", *b"qwid", *b"rand", *b"rtbd", *b"ruby",
    *b"salt", *b"sinf", *b"size", *b"smcp", *b"smpl", *b"subs", *b"sups", *b"swsh", *b"titl",
    *b"tnam", *b"tnum", *b"trad", *b"twid", *b"unic", *b"vert", *b"vhal", *b"vkna", *b"vpal",
    *b"vrt2", *b"vrtr", *b"zero",
];

const AAT_DELETED_GLYPH: u16 = 0xFFFF;
const AAT_NO_INDEX: u16 = 0xFFFF;
const AAT_START_STATES: [u16; 2] = [0, 1];
const AAT_END_OF_TEXT_CLASS: u16 = 0;
const AAT_OUT_OF_BOUNDS_CLASS: u16 = 1;

/// Returns whether shaping any sequence of printable ASCII characters in this font yields glyph
/// runs whose total advance equals the sum of the characters' own advances.
///
/// `ascii_glyphs` are the glyphs the platform maps U+0020..=U+007E to, `requested_features` are
/// the user's OpenType feature settings, and `advance` reports a glyph's horizontal advance.
pub fn ascii_shaping_preserves_advances(
    tables: ShapingTables<'_>,
    ascii_glyphs: &[u16],
    requested_features: &[(String, u32)],
    advance: impl Fn(u16) -> Option<f32>,
) -> bool {
    if tables.kerx.is_some() {
        return false;
    }
    let Ok(face) = Face::from_raw_tables(RawFaceTables {
        head: tables.head,
        hhea: tables.hhea,
        maxp: tables.maxp,
        hmtx: tables.hmtx,
        gsub: tables.gsub,
        gpos: tables.gpos,
        morx: tables.morx,
        kern: tables.kern,
        ..RawFaceTables::default()
    }) else {
        return false;
    };
    let face_tables = face.tables();
    if face_tables.morx.is_some() && requested_features.iter().any(|(_, value)| *value != 0) {
        return false;
    }
    let advance = |glyph: GlyphId| advance(glyph.0);
    let mut closure = ascii_glyphs
        .iter()
        .map(|glyph| GlyphId(*glyph))
        .collect::<FxHashSet<_>>();
    let gsub_lookups = face_tables
        .gsub
        .as_ref()
        .map(|gsub| enabled_lookups(gsub, requested_features, nested_substitution_lookups));
    let gpos_lookups = face_tables
        .gpos
        .as_ref()
        .map(|gpos| enabled_lookups(gpos, requested_features, nested_positioning_lookups));

    let glyph_limit = face.number_of_glyphs() as usize;
    for _ in 0..=glyph_limit {
        let before = closure.len();
        if let (Some(gsub), Some(lookups)) = (&face_tables.gsub, &gsub_lookups) {
            for index in lookups {
                let Some(lookup) = gsub.lookups.get(*index) else {
                    return false;
                };
                for subtable in lookup.subtables.into_iter::<SubstitutionSubtable>() {
                    if !substitution_preserves_advances(&subtable, &mut closure, &advance) {
                        return false;
                    }
                }
            }
        }
        if let Some(morx) = &face_tables.morx {
            for chain in morx.chains {
                for subtable in chain.subtables {
                    if chain.default_flags & subtable.feature_flags == 0
                        || subtable.coverage.is_vertical()
                    {
                        continue;
                    }
                    if !morx_subtable_is_inert(&subtable.kind, &mut closure, &advance) {
                        return false;
                    }
                }
            }
        }
        if closure.len() == before {
            break;
        }
    }

    if let (Some(gpos), Some(lookups)) = (&face_tables.gpos, &gpos_lookups) {
        for index in lookups {
            let Some(lookup) = gpos.lookups.get(*index) else {
                return false;
            };
            for subtable in lookup.subtables.into_iter::<PositioningSubtable>() {
                if !positioning_preserves_advances(&subtable, &closure) {
                    return false;
                }
            }
        }
    }
    if let Some(kern) = &face_tables.kern {
        for subtable in kern.subtables {
            if !subtable.horizontal {
                continue;
            }
            if subtable.has_state_machine || subtable.variable || subtable.has_cross_stream {
                return false;
            }
            for left in &closure {
                for right in &closure {
                    if subtable
                        .glyphs_kerning(*left, *right)
                        .is_some_and(|kerning| kerning != 0)
                    {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn feature_is_enabled(tag: Tag, requested_features: &[(String, u32)]) -> bool {
    let bytes = tag.to_bytes();
    if let Some((_, value)) = requested_features
        .iter()
        .find(|(requested, _)| requested.as_bytes() == bytes)
    {
        return *value != 0;
    }
    let numbered_variant =
        (&bytes[..2] == b"cv" || &bytes[..2] == b"ss") && bytes[2..].iter().all(u8::is_ascii_digit);
    !numbered_variant && !DEFAULT_OFF_FEATURES.contains(&bytes)
}

fn enabled_lookups(
    table: &LayoutTable<'_>,
    requested_features: &[(String, u32)],
    nested: impl Fn(&LayoutTable<'_>, LookupIndex) -> Option<Vec<LookupIndex>>,
) -> Vec<LookupIndex> {
    let mut lookups = FxHashSet::default();
    if table.variations.is_some() {
        lookups.extend(0..table.lookups.len());
    } else {
        let mut required = FxHashSet::default();
        for script in table.scripts {
            for language in script.default_language.into_iter().chain(script.languages) {
                required.extend(language.required_feature);
            }
        }
        for (index, feature) in table.features.into_iter().enumerate() {
            if required.contains(&(index as u16))
                || feature_is_enabled(feature.tag, requested_features)
            {
                lookups.extend(feature.lookup_indices);
            }
        }
    }
    let mut pending = lookups.iter().copied().collect::<Vec<_>>();
    while let Some(index) = pending.pop() {
        for nested_index in nested(table, index).unwrap_or_default() {
            if lookups.insert(nested_index) {
                pending.push(nested_index);
            }
        }
    }
    let mut lookups = lookups.into_iter().collect::<Vec<_>>();
    lookups.sort_unstable();
    lookups
}

fn nested_substitution_lookups(
    table: &LayoutTable<'_>,
    index: LookupIndex,
) -> Option<Vec<LookupIndex>> {
    let mut nested = Vec::new();
    for subtable in table
        .lookups
        .get(index)?
        .subtables
        .into_iter::<SubstitutionSubtable>()
    {
        match subtable {
            SubstitutionSubtable::Context(context) => nested.extend(context_lookups(&context)),
            SubstitutionSubtable::ChainContext(context) => {
                nested.extend(chained_context_lookups(&context))
            }
            _ => {}
        }
    }
    Some(nested)
}

fn nested_positioning_lookups(
    table: &LayoutTable<'_>,
    index: LookupIndex,
) -> Option<Vec<LookupIndex>> {
    let mut nested = Vec::new();
    for subtable in table
        .lookups
        .get(index)?
        .subtables
        .into_iter::<PositioningSubtable>()
    {
        match subtable {
            PositioningSubtable::Context(context) => nested.extend(context_lookups(&context)),
            PositioningSubtable::ChainContext(context) => {
                nested.extend(chained_context_lookups(&context))
            }
            _ => {}
        }
    }
    Some(nested)
}

fn context_lookups(context: &ContextLookup<'_>) -> Vec<LookupIndex> {
    match context {
        ContextLookup::Format1 { sets, .. } | ContextLookup::Format2 { sets, .. } => (0..sets
            .len())
            .filter_map(|set| sets.get(set))
            .flat_map(|set| (0..set.len()).filter_map(move |rule| set.get(rule)))
            .flat_map(|rule| rule.lookups.into_iter())
            .map(|record| record.lookup_list_index)
            .collect(),
        ContextLookup::Format3 { lookups, .. } => lookups
            .into_iter()
            .map(|record| record.lookup_list_index)
            .collect(),
    }
}

fn chained_context_lookups(context: &ChainedContextLookup<'_>) -> Vec<LookupIndex> {
    match context {
        ChainedContextLookup::Format1 { sets, .. } | ChainedContextLookup::Format2 { sets, .. } => {
            (0..sets.len())
                .filter_map(|set| sets.get(set))
                .flat_map(|set| (0..set.len()).filter_map(move |rule| set.get(rule)))
                .flat_map(|rule| rule.lookups.into_iter())
                .map(|record| record.lookup_list_index)
                .collect()
        }
        ChainedContextLookup::Format3 { lookups, .. } => lookups
            .into_iter()
            .map(|record| record.lookup_list_index)
            .collect(),
    }
}

fn covered_closure_glyphs(coverage: &Coverage<'_>, closure: &FxHashSet<GlyphId>) -> Vec<GlyphId> {
    let mut glyphs = match coverage {
        Coverage::Format1 { glyphs } => glyphs
            .into_iter()
            .filter(|glyph| closure.contains(glyph))
            .collect::<Vec<_>>(),
        Coverage::Format2 { records } => records
            .into_iter()
            .flat_map(|record| record.start.0..=record.end.0)
            .map(GlyphId)
            .filter(|glyph| closure.contains(glyph))
            .collect(),
    };
    glyphs.sort_unstable();
    glyphs
}

fn outputs_preserve_advance(
    input: f32,
    outputs: &[GlyphId],
    closure: &mut FxHashSet<GlyphId>,
    advance: &impl Fn(GlyphId) -> Option<f32>,
) -> bool {
    let mut total = 0.;
    for output in outputs {
        let Some(output_advance) = advance(*output) else {
            return false;
        };
        total += output_advance;
    }
    if total != input {
        return false;
    }
    closure.extend(outputs.iter().copied());
    true
}

fn substitution_preserves_advances(
    subtable: &SubstitutionSubtable<'_>,
    closure: &mut FxHashSet<GlyphId>,
    advance: &impl Fn(GlyphId) -> Option<f32>,
) -> bool {
    match subtable {
        SubstitutionSubtable::Single(single) => {
            let coverage = single.coverage();
            for glyph in covered_closure_glyphs(&coverage, closure) {
                let output = match single {
                    SingleSubstitution::Format1 { delta, .. } => {
                        GlyphId((i32::from(glyph.0) + i32::from(*delta)) as u16)
                    }
                    SingleSubstitution::Format2 { substitutes, .. } => {
                        let Some(output) =
                            coverage.get(glyph).and_then(|index| substitutes.get(index))
                        else {
                            return false;
                        };
                        output
                    }
                };
                let Some(input) = advance(glyph) else {
                    return false;
                };
                if !outputs_preserve_advance(input, &[output], closure, advance) {
                    return false;
                }
            }
            true
        }
        SubstitutionSubtable::Multiple(multiple) => {
            for glyph in covered_closure_glyphs(&multiple.coverage, closure) {
                let Some((input, sequence)) = advance(glyph).zip(
                    multiple
                        .coverage
                        .get(glyph)
                        .and_then(|index| multiple.sequences.get(index)),
                ) else {
                    return false;
                };
                let outputs = sequence.substitutes.into_iter().collect::<Vec<_>>();
                if !outputs_preserve_advance(input, &outputs, closure, advance) {
                    return false;
                }
            }
            true
        }
        SubstitutionSubtable::Alternate(alternate) => {
            for glyph in covered_closure_glyphs(&alternate.coverage, closure) {
                let Some((input, set)) = advance(glyph).zip(
                    alternate
                        .coverage
                        .get(glyph)
                        .and_then(|index| alternate.alternate_sets.get(index)),
                ) else {
                    return false;
                };
                for output in set.alternates {
                    if !outputs_preserve_advance(input, &[output], closure, advance) {
                        return false;
                    }
                }
            }
            true
        }
        SubstitutionSubtable::Ligature(ligature) => {
            for first in covered_closure_glyphs(&ligature.coverage, closure) {
                let Some((first_advance, set)) = advance(first).zip(
                    ligature
                        .coverage
                        .get(first)
                        .and_then(|index| ligature.ligature_sets.get(index)),
                ) else {
                    return false;
                };
                for candidate in (0..set.len()).filter_map(|index| set.get(index)) {
                    if !candidate
                        .components
                        .into_iter()
                        .all(|component| closure.contains(&component))
                    {
                        continue;
                    }
                    let mut input = first_advance;
                    for component in candidate.components {
                        let Some(component_advance) = advance(component) else {
                            return false;
                        };
                        input += component_advance;
                    }
                    if !outputs_preserve_advance(input, &[candidate.glyph], closure, advance) {
                        return false;
                    }
                }
            }
            true
        }
        SubstitutionSubtable::ReverseChainSingle(reverse) => {
            for glyph in covered_closure_glyphs(&reverse.coverage, closure) {
                let Some((input, output)) = advance(glyph).zip(
                    reverse
                        .coverage
                        .get(glyph)
                        .and_then(|index| reverse.substitutes.get(index)),
                ) else {
                    return false;
                };
                if !outputs_preserve_advance(input, &[output], closure, advance) {
                    return false;
                }
            }
            true
        }
        SubstitutionSubtable::Context(_) | SubstitutionSubtable::ChainContext(_) => true,
    }
}

fn positioning_preserves_advances(
    subtable: &PositioningSubtable<'_>,
    closure: &FxHashSet<GlyphId>,
) -> bool {
    match subtable {
        PositioningSubtable::Single(single) => {
            let coverage = single.coverage();
            covered_closure_glyphs(&coverage, closure)
                .into_iter()
                .all(|glyph| match single {
                    SingleAdjustment::Format1 { value, .. } => value.x_advance == 0,
                    SingleAdjustment::Format2 { values, .. } => coverage
                        .get(glyph)
                        .and_then(|index| values.get(index))
                        .is_some_and(|value| value.x_advance == 0),
                })
        }
        PositioningSubtable::Pair(pair) => {
            let coverage = pair.coverage();
            for first in covered_closure_glyphs(&coverage, closure) {
                for second in closure {
                    let records = match pair {
                        PairAdjustment::Format1 { sets, .. } => coverage
                            .get(first)
                            .and_then(|index| sets.get(index))
                            .and_then(|set| set.get(*second)),
                        PairAdjustment::Format2 {
                            classes, matrix, ..
                        } => matrix.get((classes.0.get(first), classes.1.get(*second))),
                    };
                    if records
                        .is_some_and(|(left, right)| left.x_advance != 0 || right.x_advance != 0)
                    {
                        return false;
                    }
                }
            }
            true
        }
        PositioningSubtable::Cursive(cursive) => {
            covered_closure_glyphs(&cursive.coverage, closure).is_empty()
        }
        PositioningSubtable::MarkToBase(_)
        | PositioningSubtable::MarkToLigature(_)
        | PositioningSubtable::MarkToMark(_)
        | PositioningSubtable::Context(_)
        | PositioningSubtable::ChainContext(_) => true,
    }
}

fn morx_subtable_is_inert(
    kind: &morx::SubtableKind<'_>,
    closure: &mut FxHashSet<GlyphId>,
    advance: &impl Fn(GlyphId) -> Option<f32>,
) -> bool {
    match kind {
        morx::SubtableKind::NonContextual(lookup) => {
            let mut outputs = Vec::new();
            for glyph in closure.iter() {
                let Some(output) = lookup.value(*glyph) else {
                    continue;
                };
                if output == AAT_DELETED_GLYPH {
                    return false;
                }
                let output = GlyphId(output);
                if output != *glyph {
                    match (advance(*glyph), advance(output)) {
                        (Some(input), Some(replaced)) if input == replaced => outputs.push(output),
                        _ => return false,
                    }
                }
            }
            closure.extend(outputs);
            true
        }
        morx::SubtableKind::Rearrangement(state) => aat_machine_is_inert(
            closure,
            |glyph| state.class(glyph),
            |state_index, class| {
                state
                    .entry(state_index, class)
                    .filter(|entry| entry.flags & 0x000F == 0)
                    .map(|entry| entry.new_state)
            },
        ),
        morx::SubtableKind::Contextual(contextual) => aat_machine_is_inert(
            closure,
            |glyph| contextual.state.class(glyph),
            |state_index, class| {
                contextual
                    .state
                    .entry(state_index, class)
                    .filter(|entry| {
                        entry.extra.mark_index == AAT_NO_INDEX
                            && entry.extra.current_index == AAT_NO_INDEX
                    })
                    .map(|entry| entry.new_state)
            },
        ),
        morx::SubtableKind::Ligature(ligature) => aat_machine_is_inert(
            closure,
            |glyph| ligature.state.class(glyph),
            |state_index, class| {
                ligature
                    .state
                    .entry(state_index, class)
                    .filter(|entry| entry.flags & 0x2000 == 0)
                    .map(|entry| entry.new_state)
            },
        ),
        morx::SubtableKind::Insertion(insertion) => aat_machine_is_inert(
            closure,
            |glyph| insertion.state.class(glyph),
            |state_index, class| {
                insertion
                    .state
                    .entry(state_index, class)
                    .filter(|entry| {
                        entry.extra.current_insert_index == AAT_NO_INDEX
                            && entry.extra.marked_insert_index == AAT_NO_INDEX
                    })
                    .map(|entry| entry.new_state)
            },
        ),
    }
}

fn aat_machine_is_inert(
    closure: &FxHashSet<GlyphId>,
    class_of: impl Fn(GlyphId) -> Option<u16>,
    inert_transition: impl Fn(u16, u16) -> Option<u16>,
) -> bool {
    let mut classes = closure
        .iter()
        .map(|glyph| class_of(*glyph).unwrap_or(AAT_OUT_OF_BOUNDS_CLASS))
        .collect::<FxHashSet<_>>();
    classes.insert(AAT_END_OF_TEXT_CLASS);
    let mut visited = AAT_START_STATES.iter().copied().collect::<FxHashSet<_>>();
    let mut pending = AAT_START_STATES.to_vec();
    while let Some(state) = pending.pop() {
        for class in &classes {
            let Some(next) = inert_transition(state, *class) else {
                return false;
            };
            if visited.insert(next) {
                pending.push(next);
            }
        }
    }
    true
}
