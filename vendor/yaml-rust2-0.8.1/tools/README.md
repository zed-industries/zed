# `yaml-rust2` tools
This directory contains tools that are used to develop the crate.
Due to dependency management, only some of them are available as binaries from the `yaml-rust2` crate.

| Tool | Invocation |
|------|------------|
| `bench_compare` | `cargo bench_compare` |
| `dump_events` | `cargo run --bin dump_events -- [...]` |
| `gen_large_yaml` | `cargo gen_large_yaml` |
| `run_bench` | `cargo run --bin run_bench -- [...]` |
| `time_parse` | `cargo run --bin time_parse -- [...]` |

## `bench_compare`
See the [dedicated README file](./bench_compare/README.md).

## `dump_events`
This is a debugging helper for the parser. It outputs events emitted by the parser for a given file. This can be paired with the `YAMLRUST2_DEBUG` environment variable to have an in-depth overview of which steps the scanner and the parser are taking.

### Example
Consider the following `input.yaml` YAML file:
```yaml
- foo: bar
- baz:
  c: [3, 4, 5]
```

Running `cargo run --bin dump_events -- input.yaml` outputs:
```
      ↳ StreamStart
      ↳ DocumentStart
      ↳ SequenceStart(0, None)
      ↳ MappingStart(0, None)
      ↳ Scalar("foo", Plain, 0, None)
      ↳ Scalar("bar", Plain, 0, None)
      ↳ MappingEnd
      ↳ MappingStart(0, None)
      ↳ Scalar("baz", Plain, 0, None)
      ↳ Scalar("~", Plain, 0, None)
      ↳ Scalar("c", Plain, 0, None)
      ↳ SequenceStart(0, None)
      ↳ Scalar("3", Plain, 0, None)
      ↳ Scalar("4", Plain, 0, None)
      ↳ Scalar("5", Plain, 0, None)
      ↳ SequenceEnd
      ↳ MappingEnd
      ↳ SequenceEnd
      ↳ DocumentEnd
      ↳ StreamEnd
```

Running `YAMLRUST2_DEBUG=1 cargo run --bin dump_events -- input.yaml` outputs much more details:
<details>
<summary> Full output </summary>

```
Parser state: StreamStart
    ↳ StreamStart(Utf8) Marker { index: 0, line: 1, col: 0 }
      ↳ StreamStart

Parser state: ImplicitDocumentStart
  → fetch_next_token after whitespace Marker { index: 0, line: 1, col: 0 } '-'
    ↳ BlockSequenceStart Marker { index: 0, line: 1, col: 0 }
      ↳ DocumentStart

Parser state: BlockNode
      ↳ SequenceStart(0, None)

Parser state: BlockSequenceFirstEntry
    ↳ BlockEntry Marker { index: 2, line: 1, col: 2 }
  → fetch_next_token after whitespace Marker { index: 2, line: 1, col: 2 } 'f'
  → fetch_next_token after whitespace Marker { index: 5, line: 1, col: 5 } ':'
    ↳ BlockMappingStart Marker { index: 5, line: 1, col: 5 }
      ↳ MappingStart(0, None)

Parser state: BlockMappingFirstKey
    ↳ Key Marker { index: 2, line: 1, col: 2 }
    ↳ Scalar(Plain, "foo") Marker { index: 2, line: 1, col: 2 }
      ↳ Scalar("foo", Plain, 0, None)

Parser state: BlockMappingValue
    ↳ Value Marker { index: 5, line: 1, col: 5 }
  → fetch_next_token after whitespace Marker { index: 7, line: 1, col: 7 } 'b'
    ↳ Scalar(Plain, "bar") Marker { index: 7, line: 1, col: 7 }
      ↳ Scalar("bar", Plain, 0, None)

Parser state: BlockMappingKey
  → fetch_next_token after whitespace Marker { index: 11, line: 2, col: 0 } '-'
    ↳ BlockEnd Marker { index: 11, line: 2, col: 0 }
      ↳ MappingEnd

Parser state: BlockSequenceEntry
    ↳ BlockEntry Marker { index: 13, line: 2, col: 2 }
  → fetch_next_token after whitespace Marker { index: 13, line: 2, col: 2 } 'b'
  → fetch_next_token after whitespace Marker { index: 16, line: 2, col: 5 } ':'
    ↳ BlockMappingStart Marker { index: 16, line: 2, col: 5 }
      ↳ MappingStart(0, None)

Parser state: BlockMappingFirstKey
    ↳ Key Marker { index: 13, line: 2, col: 2 }
    ↳ Scalar(Plain, "baz") Marker { index: 13, line: 2, col: 2 }
      ↳ Scalar("baz", Plain, 0, None)

Parser state: BlockMappingValue
    ↳ Value Marker { index: 16, line: 2, col: 5 }
  → fetch_next_token after whitespace Marker { index: 20, line: 3, col: 2 } 'c'
  → fetch_next_token after whitespace Marker { index: 21, line: 3, col: 3 } ':'
    ↳ Key Marker { index: 20, line: 3, col: 2 }
      ↳ Scalar("~", Plain, 0, None)

Parser state: BlockMappingKey
    ↳ Scalar(Plain, "c") Marker { index: 20, line: 3, col: 2 }
      ↳ Scalar("c", Plain, 0, None)

Parser state: BlockMappingValue
    ↳ Value Marker { index: 21, line: 3, col: 3 }
  → fetch_next_token after whitespace Marker { index: 23, line: 3, col: 5 } '['
    ↳ FlowSequenceStart Marker { index: 23, line: 3, col: 5 }
      ↳ SequenceStart(0, None)

Parser state: FlowSequenceFirstEntry
  → fetch_next_token after whitespace Marker { index: 24, line: 3, col: 6 } '3'
  → fetch_next_token after whitespace Marker { index: 25, line: 3, col: 7 } ','
    ↳ Scalar(Plain, "3") Marker { index: 24, line: 3, col: 6 }
      ↳ Scalar("3", Plain, 0, None)

Parser state: FlowSequenceEntry
    ↳ FlowEntry Marker { index: 25, line: 3, col: 7 }
  → fetch_next_token after whitespace Marker { index: 27, line: 3, col: 9 } '4'
  → fetch_next_token after whitespace Marker { index: 28, line: 3, col: 10 } ','
    ↳ Scalar(Plain, "4") Marker { index: 27, line: 3, col: 9 }
      ↳ Scalar("4", Plain, 0, None)

Parser state: FlowSequenceEntry
    ↳ FlowEntry Marker { index: 28, line: 3, col: 10 }
  → fetch_next_token after whitespace Marker { index: 30, line: 3, col: 12 } '5'
  → fetch_next_token after whitespace Marker { index: 31, line: 3, col: 13 } ']'
    ↳ Scalar(Plain, "5") Marker { index: 30, line: 3, col: 12 }
      ↳ Scalar("5", Plain, 0, None)

Parser state: FlowSequenceEntry
    ↳ FlowSequenceEnd Marker { index: 31, line: 3, col: 13 }
      ↳ SequenceEnd

Parser state: BlockMappingKey
  → fetch_next_token after whitespace Marker { index: 33, line: 4, col: 0 } '\0'
    ↳ BlockEnd Marker { index: 33, line: 4, col: 0 }
      ↳ MappingEnd

Parser state: BlockSequenceEntry
    ↳ BlockEnd Marker { index: 33, line: 4, col: 0 }
      ↳ SequenceEnd

Parser state: DocumentEnd
    ↳ StreamEnd Marker { index: 33, line: 4, col: 0 }
      ↳ DocumentEnd

Parser state: DocumentStart
      ↳ StreamEnd
```

</details>

While this cannot be shown in Markdown, the output is colored so that it is a bit easier to read.

## `gen_large_yaml`
It is hard to find large (100+MiB) real-world YAML files that could be used to benchmark a parser. This utility generates multiple large files that are meant to stress the parser with different layouts of YAML files. The resulting files do not look like anything that would be encountered in production, but can serve as a base to test several features of a YAML parser.

The generated files are the following:

  - `big.yaml`: A large array of records with few fields. One of the fields is a description, a large text block scalar spanning multiple lines. Most of the scanning happens in block scalars.
  - `nested.yaml`: Very short key-value pairs that nest deeply.
  - `small_objects.yaml`: A large array of 2 key-value mappings.
  - `strings_array.yaml`: A large array of lipsum one-liners (~150-175 characters in length).

All generated files are meant to be between 200 and 250 MiB in size.

This tool depends on external dependencies that are not part of `yaml-rust2`'s dependencies or `dev-dependencies` and as such can't be called through `cargo run` directly. A dedicated `cargo gen_large_yaml` alias can be used to generate the benchmark files.

## `run_bench`
This is a benchmarking helper that runs the parser on the given file a given number of times and is able to extract simple metrics out of the results. The `--output-yaml` flag can be specified to make the output a YAML file that can be fed into other tools.

This binary is made to be used by `bench_compare`.

Synopsis: `run_bench input.yaml <iterations> [--output-yaml]`

### Examples
```sh
$> cargo run --release --bin run_bench -- bench_yaml/big.yaml 10
Average: 1.631936191s
Min: 1.629654651s
Max: 1.633045284s
95%: 1.633045284s

$> cargo run --release --bin run_bench -- bench_yaml/big.yaml 10 --output-yaml
parser: yaml-rust2
input: bench_yaml/big.yaml
average: 1649847674
min: 1648277149
max: 1651936305
percentile95: 1651936305
iterations: 10
times:
  - 1650216129
  - 1649349978
  - 1649507018
  - 1648277149
  - 1649036548
  - 1650323982
  - 1650917692
  - 1648702081
  - 1650209860
  - 1651936305
```

## `time_parse`
This is a benchmarking helper that times how long it takes for the parser to emit all events. It calls the parser on the given input file, receives parsing events and then immediately discards them. It is advised to run this tool with `--release`.

### Examples
Loading a small file could output the following:
```sh
$> cargo run --release --bin time_parse -- input.yaml
Loaded 0MiB in 14.189µs
```

While loading a larger file could output the following:
```sh
$> cargo run --release --bin time_parse -- bench_yaml/big.yaml
Loaded 220MiB in 1.612677853s
```
