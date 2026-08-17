#!/usr/bin/env python3

import os
import subprocess
import fnmatch
from pathlib import Path


ignore = [
    "comments_001.xml",
    "tree_001.xml",
]

input_dir = '../tests/files/'

files = os.listdir(input_dir)
files = fnmatch.filter(files, '*.xml')
files = sorted(files)

is_ok = True

for idx, xml_file_name in enumerate(files):
    xml_file_path = input_dir + xml_file_name
    res_file_path = input_dir + Path(input_dir + xml_file_name).stem + '.yaml'

    print('Test {} of {}: {}'.format(idx + 1, len(files), xml_file_name))

    if xml_file_name in ignore:
        continue

    with open(res_file_path, 'r') as f:
        expected_ast = f.read()

    try:
        lxml_ast = subprocess.check_output(['./lxml-ast.py', xml_file_path],
                                           stderr=subprocess.DEVNULL).decode('utf-8')
    except subprocess.CalledProcessError as e:
        if expected_ast.startswith('error:'):
            continue
        else:
            is_ok = False
            print('Failed')
            break

    if lxml_ast != expected_ast:
        is_ok = False
        print('Failed')
        break

if not is_ok:
    exit(1)
