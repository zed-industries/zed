#!/usr/bin/env python3

import urllib.request
import os

URL = 'https://www.unicode.org/Public/vertical/revision-17/VerticalOrientation-17.txt'
FILE_NAME = 'VerticalOrientation-17.txt'

if not os.path.exists(FILE_NAME):
    urllib.request.urlretrieve(URL, FILE_NAME)

ranges = []
with open(FILE_NAME) as f:
    for line in f:
        line = line.strip()

        if not line:
            continue

        if line.startswith('#'):
            continue

        range, type = line.split(' ; ')

        if type == 'R':
            continue
        elif type == 'U':
            rust_type = 'Orientation::Upright'
        elif type == 'Tu':
            rust_type = 'Orientation::TransformedOrUpright'
        elif type == 'Tr':
            rust_type = 'Orientation::TransformedOrRotated'

        if '..' in range:
            start, end = range.split('..')
            ranges.append([start, end, rust_type])
        else:
            ranges.append([range, None, rust_type])

merged_ranges = []
idx = 0
while idx < len(ranges):
    range = ranges[idx]

    merged_ranges.append(range)
    idx += 1

    if range[1] != None:
        continue

    last_char = int(range[0], 16) + 1

    while idx < len(ranges):
        next_range = ranges[idx]

        if next_range[1] != None:
            break

        if last_char == int(next_range[0], 16) and range[2] == next_range[2]:
            merged_ranges[-1][1] = next_range[0]
            last_char += 1
            idx += 1
        else:
            break

for range in merged_ranges:
    if range[1]:
        print('0x{}...0x{} => {},'.format(range[0], range[1], range[2]))
    else:
        print('0x{} => {},'.format(range[0], range[2]))
