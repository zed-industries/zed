"""
Generate the BASE64_DECODE_TABLE constant. See its doc-comment.
"""

import string

# https://tools.ietf.org/html/rfc4648#section-4
alphabet = string.ascii_uppercase + string.ascii_lowercase + string.digits + "+/"
assert len(alphabet) == 64

reverse_table = [-1] * 256
for i, symbol in enumerate(alphabet):
    reverse_table[ord(symbol)] = i

print("[")
per_line = 16
for line in range(0, 256, per_line):
    print("   " + "".join(" %2s," % value for value in reverse_table[line:][:per_line]))
print("]")
