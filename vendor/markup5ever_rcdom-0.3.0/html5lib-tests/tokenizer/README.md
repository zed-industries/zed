Tokenizer tests
===============

The test format is [JSON](http://www.json.org/). This has the advantage
that the syntax allows backward-compatible extensions to the tests and
the disadvantage that it is relatively verbose.

Basic Structure
---------------

    {"tests": [
        {"description": "Test description",
        "input": "input_string",
        "output": [expected_output_tokens],
        "initialStates": [initial_states],
        "lastStartTag": last_start_tag,
        "errors": [parse_errors]
        }
    ]}

Multiple tests per file are allowed simply by adding more objects to the
"tests" list.

Each parse error is an object that contains error `code` and one-based
error location indices: `line` and `col`.

`description`, `input` and `output` are always present. The other values
are optional.

### Test set-up

`test.input` is a string containing the characters to pass to the
tokenizer. Specifically, it represents the characters of the **input
stream**, and so implementations are expected to perform the processing
described in the spec's **Preprocessing the input stream** section
before feeding the result to the tokenizer.

If `test.doubleEscaped` is present and `true`, then `test.input` is not
quite as described above. Instead, it must first be subjected to another
round of unescaping (i.e., in addition to any unescaping involved in the
JSON import), and the result of *that* represents the characters of the
input stream. Currently, the only unescaping required by this option is
to convert each sequence of the form \\uHHHH (where H is a hex digit)
into the corresponding Unicode code point. (Note that this option also
affects the interpretation of `test.output`.)

`test.initialStates` is a list of strings, each being the name of a
tokenizer state which can be one of the following:

-   `Data state`
-   `PLAINTEXT state`
-   `RCDATA state`
-   `RAWTEXT state`
-   `Script data state`
-   `CDATA section state`

 The test should be run once for each string, using it
to set the tokenizer's initial state for that run. If
`test.initialStates` is omitted, it defaults to `["Data state"]`.

`test.lastStartTag` is a lowercase string that should be used as "the
tag name of the last start tag to have been emitted from this
tokenizer", referenced in the spec's definition of **appropriate end tag
token**. If it is omitted, it is treated as if "no start tag has been
emitted from this tokenizer".

### Test results

`test.output` is a list of tokens, ordered with the first produced by
the tokenizer the first (leftmost) in the list. The list must mach the
**complete** list of tokens that the tokenizer should produce. Valid
tokens are:

    ["DOCTYPE", name, public_id, system_id, correctness]
    ["StartTag", name, {attributes}*, true*]
    ["StartTag", name, {attributes}]
    ["EndTag", name]
    ["Comment", data]
    ["Character", data]

`public_id` and `system_id` are either strings or `null`. `correctness`
is either `true` or `false`; `true` corresponds to the force-quirks flag
being false, and vice-versa.

When the self-closing flag is set, the `StartTag` array has `true` as
its fourth entry. When the flag is not set, the array has only three
entries for backwards compatibility.

All adjacent character tokens are coalesced into a single
`["Character", data]` token.

If `test.doubleEscaped` is present and `true`, then every string within
`test.output` must be further unescaped (as described above) before
comparing with the tokenizer's output.

xmlViolation tests
------------------

`tokenizer/xmlViolation.test` differs from the above in a couple of
ways:

-   The name of the single member of the top-level JSON object is
    "xmlViolationTests" instead of "tests".
-   Each test's expected output assumes that implementation is applying
    the tweaks given in the spec's "Coercing an HTML DOM into an
    infoset" section.

