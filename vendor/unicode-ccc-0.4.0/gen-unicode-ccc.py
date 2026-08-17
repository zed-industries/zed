#!/usr/bin/env python3

import urllib.request
import os

URL = 'https://www.unicode.org/Public/16.0.0/ucd/extracted/DerivedCombiningClass.txt'
FILE_NAME = 'DerivedCombiningClass.txt'

if not os.path.exists(FILE_NAME):
    urllib.request.urlretrieve(URL, FILE_NAME)

classes = dict()
last_class = ''
ranges = []
with open(FILE_NAME) as f:
    for line in f:
        line = line.strip()

        if not line:
            continue

        if line.startswith('# Canonical_Combining_Class='):
            class_name = line[28:].replace('_', '')
            if last_class:
                classes[last_class] = ranges
                ranges = []

            last_class = class_name

        if line.startswith('#'):
            continue

        range, _ = line.split(';')
        range = range.strip()

        if '..' in range:
            start, end = range.split('..')
            ranges.append([start, end])
        else:
            ranges.append([range, None])
    classes[last_class] = ranges

for class_name, ranges in classes.items():
    if class_name == 'NotReordered':
        continue

    for range in ranges:
        if range[1]:
            print('0x{}..=0x{} => {},'.format(range[0], range[1], class_name))
        else:
            print('0x{} => {},'.format(range[0], class_name))
