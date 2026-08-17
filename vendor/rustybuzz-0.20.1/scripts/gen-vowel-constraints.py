#!/usr/bin/env python3

"""
Generator of the function to prohibit certain vowel sequences.

It creates ``preprocess_text_vowel_constraints``, which inserts dotted
circles into sequences prohibited by the USE script development spec.

Based on harfbuzz/src/gen-vowel-constraints.py
"""

import collections
import io
import os
import urllib.request

if not os.path.exists('Scripts.txt'):
    urllib.request.urlretrieve('https://unicode.org/Public/UCD/latest/ucd/Scripts.txt', 'Scripts.txt')

with io.open('Scripts.txt', encoding='utf-8') as f:
    scripts_header = [f.readline() for i in range(2)]
    scripts = {}
    script_order = {}
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
        script = fields[1]
        for u in range(start, end + 1):
            scripts[u] = script
        if script not in script_order:
            script_order[script] = start


class ConstraintSet(object):
    """A set of prohibited code point sequences.

    Args:
        constraint (List[int]): A prohibited code point sequence.

    """

    def __init__(self, constraint):
        # Either a list or a dictionary. As a list of code points, it
        # represents a prohibited code point sequence. As a dictionary,
        # it represents a set of prohibited sequences, where each item
        # represents the set of prohibited sequences starting with the
        # key (a code point) concatenated with any of the values
        # (ConstraintSets).
        self._c = constraint

    def add(self, constraint):
        """Add a constraint to this set."""
        if not constraint:
            return
        first = constraint[0]
        rest = constraint[1:]
        if isinstance(self._c, list):
            if constraint == self._c[:len(constraint)]:
                self._c = constraint
            elif self._c != constraint[:len(self._c)]:
                self._c = {self._c[0]: ConstraintSet(self._c[1:])}
        if isinstance(self._c, dict):
            if first in self._c:
                self._c[first].add(rest)
            else:
                self._c[first] = ConstraintSet(rest)

    def __str__(self, index=0, depth=4):
        s = []
        if isinstance(self._c, list):
            if len(self._c) == 0:
                assert index == 2, 'Cannot use `matched` for this constraint; the general case has not been implemented'
                s.append('matched = true;\n')
            elif len(self._c) == 1:
                assert index == 1, 'Cannot use `matched` for this constraint; the general case has not been implemented'
                s.append('matched = 0x{:04X} == buffer.cur({}).glyph_id;\n'.format(next(
                    iter(self._c)), index))
            else:
                s.append('if 0x{:04X} == buffer.cur({}).glyph_id &&\n'.format(self._c[0], index))
                if index:
                    s.append('buffer.idx + {} < buffer.len &&\n'.format(index + 1))
                for i, cp in enumerate(self._c[1:], start=1):
                    s.append('0x{:04X} == buffer.cur({}).glyph_id{}\n'.format(
                        cp, index + i, '' if i == len(self._c) - 1 else ' &&'))
                s.append('{\n')
                for i in range(index + 1):
                    s.append('buffer.next_glyph();\n')
                s.append('output_dotted_circle(buffer);\n')
                s.append('}\n')
        else:
            s.append('match buffer.cur({}).glyph_id {{\n'.format(index))
            cases = collections.defaultdict(set)
            for first, rest in sorted(self._c.items()):
                cases[rest.__str__(index + 1, depth + 2)].add(first)
            for ii, (body, labels) in enumerate(sorted(cases.items(), key=lambda b_ls: sorted(b_ls[1])[0])):
                for i, cp in enumerate(sorted(labels)):
                    if i == len(labels) - 1:
                        s.append(' 0x{:04X} => {{ {}'.format(cp, '\n' if i % 4 == 3 else ''))
                    else:
                        s.append(' 0x{:04X} | {}'.format(cp, '\n' if i % 4 == 3 else ''))
                s.append(body)
                s.append('}')
                if ii == len(cases.items()) - 1:
                    s.append('_ => {}')
            s.append('}\n')
        return ''.join(s)


constraints = {}
with io.open('ms-use/IndicShapingInvalidCluster.txt', encoding='utf-8') as f:
    constraints_header = []
    while True:
        line = f.readline().strip()
        if line == '#':
            break
        constraints_header.append(line)
    for line in f:
        j = line.find('#')
        if j >= 0:
            line = line[:j]
        constraint = [int(cp, 16) for cp in line.split(';')[0].split()]
        if not constraint:
            continue
        assert 2 <= len(constraint), 'Prohibited sequence is too short: {}'.format(constraint)
        script = scripts[constraint[0]]
        if script in constraints:
            constraints[script].add(constraint)
        else:
            constraints[script] = ConstraintSet(constraint)
        assert constraints, 'No constraints found'

print('// WARNING: this file was generated by scripts/gen-vowel-constraints.py')
print()
print('#![allow(clippy::single_match)]')
print()
print('use super::buffer::hb_buffer_t;')
print('use super::ot_layout::*;')
print('use super::script;')
print('use crate::BufferFlags;')
print()
print('fn output_dotted_circle(buffer: &mut hb_buffer_t) {')
print('    buffer.output_glyph(0x25CC);')
print('    {')
print('        let out_idx = buffer.out_len - 1;')
print('        _hb_glyph_info_reset_continuation(&mut buffer.out_info_mut()[out_idx]);')
print('    }')
print('}')
print()
print('fn output_with_dotted_circle(buffer: &mut hb_buffer_t) {')
print('    output_dotted_circle(buffer);')
print('    buffer.next_glyph();')
print('}')
print()
print('pub fn preprocess_text_vowel_constraints(buffer: &mut hb_buffer_t) {')
print('    if buffer.flags.contains(BufferFlags::DO_NOT_INSERT_DOTTED_CIRCLE) {')
print('        return;')
print('    }')
print()
print('    // UGLY UGLY UGLY business of adding dotted-circle in the middle of')
print('    // vowel-sequences that look like another vowel.  Data for each script')
print('    // collected from the USE script development spec.')
print('    //')
print('    // https://github.com/harfbuzz/harfbuzz/issues/1019')
print('    buffer.clear_output();')
print('    match buffer.script {')

for script, constraints in sorted(constraints.items(), key=lambda s_c: script_order[s_c[0]]):
    print('        Some(script::{}) => {{'.format(script.upper()))
    print('            buffer.idx = 0;')
    print('            while buffer.idx + 1 < buffer.len {')
    print('               #[allow(unused_mut)]')
    print('                let mut matched = false;')
    print(str(constraints), end='')
    print('                buffer.next_glyph();')
    print('                if matched { output_with_dotted_circle(buffer); }')
    print('      }')
    print('      }')
    print()

print('        _ => {}')
print('    }')
print('    buffer.sync();')
print('}')
print()
