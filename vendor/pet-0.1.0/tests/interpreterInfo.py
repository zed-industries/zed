# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

import json
import sys

obj = {
    "version_info": tuple(sys.version_info),
    "sys_prefix": sys.prefix,
    "sys_version": sys.version,
    "is64_bit": sys.maxsize > 2**32,
    "executable": sys.executable,
}

# Everything after this is the information we need
print("503bebe7-c838-4cea-a1bc-0f2963bcb657")
print(json.dumps(obj))
