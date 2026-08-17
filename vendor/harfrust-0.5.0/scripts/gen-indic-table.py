#!/usr/bin/env python3

# Based on harfbuzz/src/gen-indic-table.py

import io
import os
import urllib.request

DEPENDENCIES = [
    'IndicSyllabicCategory.txt',
    'IndicPositionalCategory.txt',
    'Blocks.txt',
]

for dep in DEPENDENCIES:
    if not os.path.exists(dep):
        urllib.request.urlretrieve('https://unicode.org/Public/UCD/latest/ucd/' + dep, dep)

ALLOWED_SINGLES = [0x00A0, 0x25CC]
ALLOWED_BLOCKS = [
    'Basic Latin',
    'Latin-1 Supplement',
    'Devanagari',
    'Bengali',
    'Gurmukhi',
    'Gujarati',
    'Oriya',
    'Tamil',
    'Telugu',
    'Kannada',
    'Malayalam',
    'Myanmar',
    'Khmer',
    'Vedic Extensions',
    'General Punctuation',
    'Superscripts and Subscripts',
    'Devanagari Extended',
    'Myanmar Extended-B',
    'Myanmar Extended-A',
    'Myanmar Extended-C',
]

files = [io.open(x, encoding='utf-8') for x in DEPENDENCIES]

headers = [[f.readline() for i in range(2)] for f in files]

unicode_data = [{} for f in files]
for i, f in enumerate(files):
    for line in f:
        j = line.find('#')
        if j >= 0:
            line = line[:j]

        fields = [x.strip() for x in line.split(';')]
        if len(fields) == 1:
            continue

        uu = fields[0].split('..')
        start = int(uu[0], 16)
        if len(uu) == 1:
            end = start
        else:
            end = int(uu[1], 16)

        t = fields[1]

        for u in range(start, end + 1):
            unicode_data[i][u] = t

# Merge data into one dict:
defaults = ('Other', 'Not_Applicable', 'No_Block')
combined = {}
for i, d in enumerate(unicode_data):
    for u, v in d.items():
        if i == 2 and u not in combined:
            continue
        if u not in combined:
            combined[u] = list(defaults)
        combined[u][i] = v
combined = {k: v for k, v in combined.items() if k in ALLOWED_SINGLES or v[2] in ALLOWED_BLOCKS}

# Convert categories & positions types

category_map = {
  'Other'			: 'X',
  'Avagraha'			: 'Symbol',
  'Bindu'			: 'SM',
  'Brahmi_Joining_Number'	: 'PLACEHOLDER', # Don't care.
  'Cantillation_Mark'		: 'A',
  'Consonant'			: 'C',
  'Consonant_Dead'		: 'C',
  'Consonant_Final'		: 'CM',
  'Consonant_Head_Letter'	: 'C',
  'Consonant_Initial_Postfixed'	: 'C', # TODO
  'Consonant_Killer'		: 'M', # U+17CD only.
  'Consonant_Medial'		: 'CM',
  'Consonant_Placeholder'	: 'PLACEHOLDER',
  'Consonant_Preceding_Repha'	: 'Repha',
  'Consonant_Prefixed'		: 'X', # Don't care.
  'Consonant_Subjoined'		: 'CM',
  'Consonant_Succeeding_Repha'	: 'CM',
  'Consonant_With_Stacker'	: 'CS',
  'Gemination_Mark'		: 'SM', # https://github.com/harfbuzz/harfbuzz/issues/552
  'Invisible_Stacker'		: 'H',
  'Joiner'			: 'ZWJ',
  'Modifying_Letter'		: 'X',
  'Non_Joiner'			: 'ZWNJ',
  'Nukta'			: 'N',
  'Number'			: 'PLACEHOLDER',
  'Number_Joiner'		: 'PLACEHOLDER', # Don't care.
  'Pure_Killer'			: 'M', # Is like a vowel matra.
  'Register_Shifter'		: 'RS',
  'Syllable_Modifier'		: 'SM',
  'Tone_Letter'			: 'X',
  'Tone_Mark'			: 'N',
  'Virama'			: 'H',
  'Visarga'			: 'SM',
  'Vowel'			: 'V',
  'Vowel_Dependent'		: 'M',
  'Vowel_Independent'		: 'V',
}

position_map = {
  'Not_Applicable'		: 'END',

  'Left'			: 'PRE_C',
  'Top'				: 'ABOVE_C',
  'Bottom'			: 'BELOW_C',
  'Right'			: 'POST_C',

  # These should resolve to the position of the last part of the split sequence.
  'Bottom_And_Right'		: 'POST_C',
  'Left_And_Right'		: 'POST_C',
  'Top_And_Bottom'		: 'BELOW_C',
  'Top_And_Bottom_And_Left'	: 'BELOW_C',
  'Top_And_Bottom_And_Right'	: 'POST_C',
  'Top_And_Left'		: 'ABOVE_C',
  'Top_And_Left_And_Right'	: 'POST_C',
  'Top_And_Right'		: 'POST_C',

  'Overstruck'			: 'AFTER_MAIN',
  'Visual_order_left'		: 'PRE_M',
}

category_overrides = {

  # These are the variation-selectors. They only appear in the Myanmar grammar
  # but are not Myanmar-specific
  0xFE00: 'VS',
  0xFE01: 'VS',
  0xFE02: 'VS',
  0xFE03: 'VS',
  0xFE04: 'VS',
  0xFE05: 'VS',
  0xFE06: 'VS',
  0xFE07: 'VS',
  0xFE08: 'VS',
  0xFE09: 'VS',
  0xFE0A: 'VS',
  0xFE0B: 'VS',
  0xFE0C: 'VS',
  0xFE0D: 'VS',
  0xFE0E: 'VS',
  0xFE0F: 'VS',

  # These appear in the OT Myanmar spec, but are not Myanmar-specific
  0x2015: 'PLACEHOLDER',
  0x2022: 'PLACEHOLDER',
  0x25FB: 'PLACEHOLDER',
  0x25FC: 'PLACEHOLDER',
  0x25FD: 'PLACEHOLDER',
  0x25FE: 'PLACEHOLDER',

  # Indic

  0x0930: 'Ra', # Devanagari
  0x09B0: 'Ra', # Bengali
  0x09F0: 'Ra', # Bengali
  0x0A30: 'Ra', # Gurmukhi 	No Reph
  0x0AB0: 'Ra', # Gujarati
  0x0B30: 'Ra', # Oriya
  0x0BB0: 'Ra', # Tamil 	No Reph
  0x0C30: 'Ra', # Telugu 	Reph formed only with ZWJ
  0x0CB0: 'Ra', # Kannada
  0x0D30: 'Ra', # Malayalam 	No Reph, Logical Repha

  # The following act more like the Bindus.
  0x0953: 'SM',
  0x0954: 'SM',

  # U+0A40 GURMUKHI VOWEL SIGN II may be preceded by U+0A02 GURMUKHI SIGN BINDI.
  0x0A40: 'MPst',

  # The following act like consonants.
  0x0A72: 'C',
  0x0A73: 'C',
  0x1CF5: 'C',
  0x1CF6: 'C',

  # TODO: The following should only be allowed after a Visarga.
  # For now, just treat them like regular tone marks.
  0x1CE2: 'A',
  0x1CE3: 'A',
  0x1CE4: 'A',
  0x1CE5: 'A',
  0x1CE6: 'A',
  0x1CE7: 'A',
  0x1CE8: 'A',

  # TODO: The following should only be allowed after some of
  # the nasalization marks, maybe only for U+1CE9..U+1CF1.
  # For now, just treat them like tone marks.
  0x1CED: 'A',

  # The following take marks in standalone clusters, similar to Avagraha.
  0xA8F2: 'Symbol',
  0xA8F3: 'Symbol',
  0xA8F4: 'Symbol',
  0xA8F5: 'Symbol',
  0xA8F6: 'Symbol',
  0xA8F7: 'Symbol',
  0x1CE9: 'Symbol',
  0x1CEA: 'Symbol',
  0x1CEB: 'Symbol',
  0x1CEC: 'Symbol',
  0x1CEE: 'Symbol',
  0x1CEF: 'Symbol',
  0x1CF0: 'Symbol',
  0x1CF1: 'Symbol',

  0x0A51: 'M', # https://github.com/harfbuzz/harfbuzz/issues/524

  # According to ScriptExtensions.txt, these Grantha marks may also be used in Tamil,
  # so the Indic shaper needs to know their categories.
  0x11301: 'SM',
  0x11302: 'SM',
  0x11303: 'SM',
  0x1133B: 'N',
  0x1133C: 'N',

  0x0AFB: 'N', # https://github.com/harfbuzz/harfbuzz/issues/552
  0x0B55: 'N', # https://github.com/harfbuzz/harfbuzz/issues/2849

  0x09FC: 'PLACEHOLDER', # https://github.com/harfbuzz/harfbuzz/pull/1613
  0x0C80: 'PLACEHOLDER', # https://github.com/harfbuzz/harfbuzz/pull/623
  0x0D04: 'PLACEHOLDER', # https://github.com/harfbuzz/harfbuzz/pull/3511

  0x25CC: 'DOTTEDCIRCLE',


  # Khmer

  0x179A: 'Ra',

  0x17CC: 'Robatic',
  0x17C9: 'Robatic',
  0x17CA: 'Robatic',

  0x17C6: 'Xgroup',
  0x17CB: 'Xgroup',
  0x17CD: 'Xgroup',
  0x17CE: 'Xgroup',
  0x17CF: 'Xgroup',
  0x17D0: 'Xgroup',
  0x17D1: 'Xgroup',

  0x17C7: 'Ygroup',
  0x17C8: 'Ygroup',
  0x17DD: 'Ygroup',
  0x17D3: 'Ygroup', # Just guessing. Uniscribe doesn't categorize it.

  0x17D9: 'PLACEHOLDER', # https://github.com/harfbuzz/harfbuzz/issues/2384


  # Myanmar

  # https://docs.microsoft.com/en-us/typography/script-development/myanmar#analyze

  0x104E: 'C', # The spec says C, IndicSyllableCategory says Consonant_Placeholder

  0x1004: 'Ra',
  0x101B: 'Ra',
  0x105A: 'Ra',

  0x1032: 'A',
  0x1036: 'A',

  0x103A: 'As',

  #0x1040: 'D0', # XXX The spec says D0, but Uniscribe doesn't seem to do.

  0x103E: 'MH',
  0x1060: 'ML',
  0x103C: 'MR',
  0x103D: 'MW',
  0x1082: 'MW',
  0x103B: 'MY',
  0x105E: 'MY',
  0x105F: 'MY',

  0x1063: 'PT',
  0x1064: 'PT',
  0x1069: 'PT',
  0x106A: 'PT',
  0x106B: 'PT',
  0x106C: 'PT',
  0x106D: 'PT',
  0xAA7B: 'PT',

  0x1038: 'SM',
  0x1087: 'SM',
  0x1088: 'SM',
  0x1089: 'SM',
  0x108A: 'SM',
  0x108B: 'SM',
  0x108C: 'SM',
  0x108D: 'SM',
  0x108F: 'SM',
  0x109A: 'SM',
  0x109B: 'SM',
  0x109C: 'SM',

  0x104A: 'PLACEHOLDER',
}

position_overrides = {

  0x0A51: 'BELOW_C', # https://github.com/harfbuzz/harfbuzz/issues/524

  0x0B01: 'BEFORE_SUB', # Oriya Bindu is BeforeSub in the spec.
}

def matra_pos_left(u, block):
  return "PRE_M"
def matra_pos_right(u, block):
  if block == 'Devanagari':	return  'AFTER_SUB'
  if block == 'Bengali':	return  'AFTER_POST'
  if block == 'Gurmukhi':	return  'AFTER_POST'
  if block == 'Gujarati':	return  'AFTER_POST'
  if block == 'Oriya':		return  'AFTER_POST'
  if block == 'Tamil':		return  'AFTER_POST'
  if block == 'Telugu':		return  'BEFORE_SUB' if u <= 0x0C42 else 'AFTER_SUB'
  if block == 'Kannada':	return  'BEFORE_SUB' if u < 0x0CC3 or u > 0x0CD6 else 'AFTER_SUB'
  if block == 'Malayalam':	return  'AFTER_POST'
  return 'AFTER_SUB'
def matra_pos_top(u, block):
  # BENG and MLYM don't have top matras.
  if block == 'Devanagari':	return  'AFTER_SUB'
  if block == 'Gurmukhi':	return  'AFTER_POST' # Deviate from spec
  if block == 'Gujarati':	return  'AFTER_SUB'
  if block == 'Oriya':		return  'AFTER_MAIN'
  if block == 'Tamil':		return  'AFTER_SUB'
  if block == 'Telugu':		return  'BEFORE_SUB'
  if block == 'Kannada':	return  'BEFORE_SUB'
  return 'AFTER_SUB'
def matra_pos_bottom(u, block):
  if block == 'Devanagari':	return  'AFTER_SUB'
  if block == 'Bengali':	return  'AFTER_SUB'
  if block == 'Gurmukhi':	return  'AFTER_POST'
  if block == 'Gujarati':	return  'AFTER_POST'
  if block == 'Oriya':		return  'AFTER_SUB'
  if block == 'Tamil':		return  'AFTER_POST'
  if block == 'Telugu':		return  'BEFORE_SUB'
  if block == 'Kannada':	return  'BEFORE_SUB'
  if block == 'Malayalam':	return  'AFTER_POST'
  return "AFTER_SUB"
def indic_matra_position(u, pos, block): # Reposition matra
  if pos == 'PRE_C':	return matra_pos_left(u, block)
  if pos == 'POST_C':	return matra_pos_right(u, block)
  if pos == 'ABOVE_C':	return matra_pos_top(u, block)
  if pos == 'BELOW_C':	return matra_pos_bottom(u, block)
  assert (False)

def position_to_category(pos):
  if pos == 'PRE_C':	return 'VPre'
  if pos == 'ABOVE_C':	return 'VAbv'
  if pos == 'BELOW_C':	return 'VBlw'
  if pos == 'POST_C':	return 'VPst'
  assert(False)



defaults = (category_map[defaults[0]], position_map[defaults[1]], defaults[2])

indic_data = {}
for k, (cat, pos, block) in combined.items():
  cat = category_map[cat]
  if cat == 'SM' and pos == 'Not_Applicable':
    cat = 'SMPst'
  pos = position_map[pos]
  indic_data[k] = (cat, pos, block)

for k,new_cat in category_overrides.items():
  (cat, pos, _) = indic_data.get(k, defaults)
  indic_data[k] = (new_cat, pos, unicode_data[2][k])

# We only expect position for certain types
positioned_categories = ('CM', 'SM', 'RS', 'H', 'M', 'MPst')
for k, (cat, pos, block) in indic_data.items():
  if cat not in positioned_categories:
    pos = 'END'
    indic_data[k] = (cat, pos, block)

# Position overrides are more complicated

# Keep in sync with CONSONANT_FLAGS in the shaper
consonant_categories = ('C', 'CS', 'Ra','CM', 'V', 'PLACEHOLDER', 'DOTTEDCIRCLE')
matra_categories = ('M', 'MPst')
smvd_categories = ('SM', 'SMPst', 'VD', 'A', 'Symbol')
for k, (cat, pos, block) in indic_data.items():
  if cat in consonant_categories:
    pos = 'BASE_C'
  elif cat in matra_categories:
    if block.startswith('Khmer') or block.startswith('Myanmar'):
      cat = position_to_category(pos)
    else:
      pos = indic_matra_position(k, pos, block)
  elif cat in smvd_categories:
    pos = 'SMVD'
  indic_data[k] = (cat, pos, block)

for k,new_pos in position_overrides.items():
  (cat, pos, _) = indic_data.get(k, defaults)
  indic_data[k] = (cat, new_pos, unicode_data[2][k])


values = [{_: 1} for _ in defaults]
for vv in indic_data.values():
  for i,v in enumerate(vv):
    values[i][v] = values[i].get (v, 0) + 1

# Move the outliers NO-BREAK SPACE and DOTTED CIRCLE out
singles = {}
for u in ALLOWED_SINGLES:
    singles[u] = indic_data[u]
    del indic_data[u]

print('// WARNING: this file was generated by scripts/gen-indic-table.py')
print()
print('#![allow(non_camel_case_types)]')
print('#![allow(unused_imports)]')
print()
print('use super::ot_shaper_indic::ot_category_t::*;')
print('use super::ot_shaper_indic::ot_position_t::*;')

# Shorten values
short = [{
	"Repha":		'Rf',
	"PLACEHOLDER":		'GB',
	"DOTTEDCIRCLE":		'DC',
    "SMPst":        'SP',
    "VPst":			'VR',
    "VPre":			'VL',
    "Robatic":		'Rt',
    "Xgroup":		'Xg',
    "Ygroup":		'Yg',
    "As":			'As',
},{
	"END":			'X',
	"BASE_C":		'C',
	"ABOVE_C":		'T',
	"BELOW_C":		'B',
	"POST_C":		'R',
	"PRE_C":		'L',
	"PRE_M":		'LM',
	"AFTER_MAIN":		'A',
	"AFTER_SUB":		'AS',
	"BEFORE_SUB":		'BS',
	"AFTER_POST":		'AP',
	"SMVD":			'SM',
}]
all_shorts = [{},{}]

# Add some of the values, to make them more readable, and to avoid duplicates

for i in range(2):
    for v, s in short[i].items():
        all_shorts[i][s] = v

what = ["OT", "POS"]
what_short = ["_OT", "_POS"]
cat_defs = []
for i in range(2):
    vv = sorted(values[i].keys())
    for v in vv:
        v_no_and = v.replace('_And_', '_')
        if v in short[i]:
            s = short[i][v]
        else:
            s = ''.join([c for c in v_no_and if ord('A') <= ord(c) <= ord('Z')])
            if s in all_shorts[i]:
                raise Exception('Duplicate short value alias', v, all_shorts[i][s])
            all_shorts[i][s] = v
            short[i][v] = s
        cat_defs.append ((what_short[i] + '_' + s, what[i] + '_' + (v.upper () if i else v), str (values[i][v]), v))

maxlen_s = max([len(c[0]) for c in cat_defs])
maxlen_l = max([len(c[1]) for c in cat_defs])
maxlen_n = max([len(c[2]) for c in cat_defs])
for s in what_short:
    print()
    for c in [c for c in cat_defs if s in c[0]]:
        print ("use %s as %s /* %s chars; %s */" %
        			(c[1].ljust (maxlen_s), (c[0] + ";").ljust (maxlen_l), c[2].rjust (maxlen_n), c[3]))
print()
print()

total = 0
used = 0
last_block = None


def print_block(block, start, end, data):
    global total, used, last_block
    if block and block != last_block:
        print()
        print()
        print('  /* %s */' % block)
    num = 0
    assert start % 8 == 0
    assert (end + 1) % 8 == 0
    for u in range(start, end + 1):
        if u % 8 == 0:
            print()
            print('  /* %04X */' % u, end='')
        if u in data:
            num += 1
        d = data.get(u, defaults)
        print('%16s' % ('(_OT_%s,_POS_%s),' % (short[0][d[0]], short[1][d[1]])), end='')

    total += end - start + 1
    used += num
    if block:
        last_block = block


uu = sorted(indic_data)

last = -100000
num = 0
offset = 0
starts = []
ends = []
print('pub type SyllabicCategory = u8;')
print('pub type MatraCategory = u8;')
print('')
print('#[rustfmt::skip]')
print('static TABLE: &[(SyllabicCategory, MatraCategory)] = &[')
offsets = []
for u in uu:
    if u <= last:
        continue
    block = indic_data[u][2]

    start = u // 8 * 8
    end = start + 1
    while end in uu and block == indic_data[end][2]:
        end += 1
    end = (end - 1) // 8 * 8 + 7

    if start != last + 1:
        if start - last <= 1 + 16 * 2:
            print_block(None, last + 1, start - 1, indic_data)
            last = start - 1
        else:
            if last >= 0:
                ends.append(last + 1)
                offset += ends[-1] - starts[-1]
            # print()
            # print()
            offsets.append('const OFFSET_0X%04X: usize = %d;' % (start, offset))
            starts.append(start)

    print_block(block, start, end, indic_data)
    last = end
ends.append(last + 1)
offset += ends[-1] - starts[-1]
print()
print()
occupancy = used * 100. / total
page_bits = 12
print('];')
print()
for o in offsets:
    print(o)
print()
print('#[rustfmt::skip]')
print('pub fn get_categories(u: u32) -> (SyllabicCategory, MatraCategory) {')
print('    match u >> %d {' % page_bits)
pages = set([u >> page_bits for u in starts + ends + list(singles.keys())])
for p in sorted(pages):
    print('        0x%0X => {' % p)
    for u, d in singles.items():
        if p != u >> page_bits: continue
        print('            if u == 0x%04X { return (_OT_%s, _POS_%s); }' % (u, short[0][d[0]], short[1][d[1]]))
    for (start, end) in zip(starts, ends):
        if p not in [start >> page_bits, end >> page_bits]: continue
        offset = 'OFFSET_0X%04X' % start
        print('            if (0x%04X..=0x%04X).contains(&u) { return TABLE[u as usize - 0x%04X + %s]; }' % (start, end - 1, start, offset))
    print('        }')
print('        _ => {}')
print('    }')
print()
print('    (_OT_X, _POS_X)')
print('}')

# Maintain at least 50% occupancy in the table */
if occupancy < 50:
    raise Exception('Table too sparse, please investigate: ', occupancy)
