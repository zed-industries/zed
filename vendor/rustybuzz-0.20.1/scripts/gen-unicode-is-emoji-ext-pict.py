#!/usr/bin/env python3

import urllib.request
import os

URL = 'https://www.unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt'
FILE_NAME = 'emoji-data.txt'

if not os.path.exists(FILE_NAME):
    urllib.request.urlretrieve(URL, FILE_NAME)

is_ext_pict_section = False
ranges = []
with open(FILE_NAME) as f:
    for line in f:
        line = line.strip()

        if not is_ext_pict_section:
            if line == '# All omitted code points have Extended_Pictographic=No':
                is_ext_pict_section = True
            continue

        if not line:
            continue

        if line.startswith('#'):
            continue

        range, _ = line.split(' ; ')
        range = range.strip()

        if '..' in range:
            start, end = range.split('..')
            ranges.append([start, end])
        else:
            ranges.append([range, range])

# Merge ranges.
idx = 0
while idx < len(ranges)-1:
    if int(ranges[idx][1], 16) + 1 == int(ranges[idx+1][0], 16):
        ranges[idx][1] = ranges[idx+1][1]
        del ranges[idx+1]
    else:
        idx += 1;

for range in ranges:
    if range[0] == range[1]:
        print('0x{} => true,'.format(range[0], range[1]))
    else:
        print('0x{}..=0x{} => true,'.format(range[0], range[1]))
