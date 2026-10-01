"""Attribute xperf sampled-profile events to source lines via dbh.exe.

Usage: symbolize.py <xperf dump.csv> <exe> [--lines N]

The dump comes from `xperf -i trace.etl -tle -tti -target machine -o dump.csv`.
llvm-symbolizer resolves each instruction pointer to its inline chain, so
functions are reported inclusively (a sample counts for every function in its
chain) and lines by the innermost frame.
"""
import collections, re, subprocess, sys

SYMBOLIZER = r'C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\Llvm\x64\bin\llvm-symbolizer.exe'
# Leaf helpers too small to be interesting as the innermost frame.
TRIVIAL = re.compile(r'::(fast_min|fast_max|fast_clamp|min_x|max_x|step|endpoints|right|bottom|top_left|bottom_right|quadrant|x|y|cell|column_of|row_of|fills|ordered_bits|new|point|get|get_mut|len|is_empty|push|iter|zip|enumerate|next|deref|index|index_mut|clone|from|into|unwrap_or|then|filter|min|max|abs|floor|is_finite)$')

dump, exe = sys.argv[1], sys.argv[2]
top = int(sys.argv[sys.argv.index('--lines') + 1]) if '--lines' in sys.argv else 40
exe_name = exe.replace('\\', '/').rsplit('/', 1)[-1]

base = end = None
samples = collections.Counter()
modules = collections.Counter()
with open(dump, encoding='utf-8', errors='replace') as f:
    for line in f:
        fields = [x.strip() for x in line.split(',')]
        if len(fields) < 4:
            continue
        kind = fields[0]
        if kind in ('I-Start', 'I-DCStart') and exe_name in line and base is None:
            base, end = int(fields[3], 16), int(fields[4], 16)
        elif kind == 'SampledProfile' and fields[2].startswith(exe_name):
            ip = int(fields[4], 16)
            module = fields[7].split('!')[0]
            modules[module] += 1
            if base is not None and base <= ip < end:
                samples[ip - base] += 1

total = sum(modules.values())
print(f'{total} samples in {exe_name}; image base {base:#x}')
print('\nby module:')
for module, count in modules.most_common(8):
    print(f'{100 * count / total:6.1f}%  {module}')

# Resolve unique RVAs to inline chains, innermost first.
rvas = list(samples)
out = subprocess.run(
    [SYMBOLIZER, f'--obj={exe}', '--relative-address', '--inlining', '--functions=linkage', '--demangle'],
    input=''.join(f'{rva:#x}\n' for rva in rvas), capture_output=True, text=True,
).stdout
blocks = out.strip('\n').split('\n\n')
assert len(blocks) == len(rvas), (len(blocks), len(rvas))

def clean(name):
    name = re.sub(r'impl\$\d+', 'impl', name)
    return name.removeprefix('gpui::path_winding::')

chains = {}
for rva, block in zip(rvas, blocks):
    lines = block.split('\n')
    chain = []
    for i in range(0, len(lines) - 1, 2):
        function = clean(lines[i])
        m = re.match(r'(.*):(\d+):\d+$', lines[i + 1].replace('\\', '/'))
        chain.append((function, m.group(1), int(m.group(2))) if m else (function, '?', 0))
    chains[rva] = chain

inclusive = collections.Counter()
innermost = collections.Counter()
by_line = collections.Counter()
for rva, count in samples.items():
    chain = chains[rva]
    for function in {f for f, _, _ in chain}:
        inclusive[function] += count
    leaf = next(((f, p, l) for f, p, l in chain if not TRIVIAL.search(f)), chain[0] if chain else ('?', '?', 0))
    innermost[leaf[0]] += count
    by_line[(leaf[0], leaf[1], leaf[2])] += count

# The innermost of these in a sample's inline chain names its phase. Entries
# ending in `::` match any method of that type; others match exactly.
PHASES = [
    ('insertion_sort', 'sort: per-cell insertion sort'),
    ('sort_listings', 'sort: prefix sum + scatter'),
    ('list_curves', 'list: spans + x_range'),
    ('Measure::', 'walk: measure cell'),
    ('Walk::close', 'walk: close / copy tile'),
    ('Walk::push', 'walk: merge along row'),
    ('Walk::', 'walk: loop'),
    ('cell_size', 'cell size'),
    ('bin', 'bin: bounds / setup'),
]

def phase(chain, module):
    functions = [f for f, _, _ in chain]
    if any('path_binning_measure' in f for f in functions):
        return 'harness'
    if module != exe_name or any(f.startswith('alloc::raw_vec') or 'heap' in f.lower() for f in functions):
        return 'heap / system'
    for function in functions:
        for pattern, name in PHASES:
            if function.startswith(pattern) if pattern.endswith('::') else function == pattern:
                return name
    return 'other'

phases = collections.Counter()
module_of = {}
with open(dump, encoding='utf-8', errors='replace') as f:
    for line in f:
        fields = [x.strip() for x in line.split(',')]
        if len(fields) > 7 and fields[0] == 'SampledProfile' and fields[2].startswith(exe_name):
            ip = int(fields[4], 16)
            module = fields[7].split('!')[0]
            rva = ip - base if base is not None and base <= ip < end else None
            phases[phase(chains.get(rva, []), module)] += 1
print('\nby phase:')
for name, count in phases.most_common():
    print(f'{100 * count / total:6.1f}%  {name}')

print('\ninclusive, by function (a sample counts for each function in its inline chain):')
for function, count in inclusive.most_common(28):
    print(f'{100 * count / total:6.1f}%  {function}')

# Functions that are not inlined (sorts, memcpy) have no caller in an inline
# chain, so the outermost frame says which real function holds the sample.
containing = collections.Counter()
for rva, count in samples.items():
    chain = chains[rva]
    containing[chain[-1][0] if chain else '?'] += count
print('\nby containing (outermost) function:')
for function, count in containing.most_common(14):
    print(f'{100 * count / total:6.1f}%  {function}')

print('\nself, by innermost non-trivial function:')
for function, count in innermost.most_common(20):
    print(f'{100 * count / total:6.1f}%  {function}')

sources = {}
def source_line(path, line):
    if path not in sources:
        try:
            with open(path, encoding='utf-8', errors='replace') as f:
                sources[path] = f.read().split('\n')
        except OSError:
            sources[path] = []
    text = sources[path]
    return text[line - 1].strip() if 0 < line <= len(text) else ''

print(f'\ntop {top} innermost lines:')
for (function, path, line), count in by_line.most_common(top):
    short = path.rsplit('/', 1)[-1]
    print(f'{100 * count / total:6.1f}%  {short}:{line:<5} {function:<28} {source_line(path, line)[:80]}')
