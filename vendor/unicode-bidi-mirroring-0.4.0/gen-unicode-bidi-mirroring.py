#!/usr/bin/env python3

import urllib.request
import os

URL = 'https://www.unicode.org/Public/UNIDATA/BidiMirroring.txt'
FILE_NAME = 'BidiMirroring.txt'

if not os.path.exists(FILE_NAME):
    urllib.request.urlretrieve(URL, FILE_NAME)

chars_before = []
chars_after = []
with open(FILE_NAME) as f:
    for line in f:
        line = line.strip()

        if not line:
            continue

        if line.startswith('#'):
            continue

        line = line[:line.index('#')]
        line = line.replace(' ', '')

        before, after = line.split(';')
        chars_before.append(before)
        chars_after.append(after)

# Deduplicate 0028=>0029 0029=>0028 pairs.
new_chars_before = []
new_chars_after = []
for i in range(0, len(chars_before)):
    if not chars_before[i] in new_chars_after:
        new_chars_before.append(chars_before[i])
        new_chars_after.append(chars_after[i])

for i in range(0, len(new_chars_before)):
    print(f'(\'\\u{{{new_chars_before[i]}}}\', \'\\u{{{new_chars_after[i]}}}\'),')
