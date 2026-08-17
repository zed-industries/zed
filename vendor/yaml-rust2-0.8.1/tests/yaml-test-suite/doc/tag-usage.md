Test Suite Tags
===============

The .tml files under the `test/` directory have a tags line that looks like
this:

```
  tags: block sequence mapping spec
```

The table below defines the tags that must be used.
This table is used by tools to validate the tags.

```
These tags can have one of the following for '*':

  ok          YAML is valid
  err         YAML is invalid
  want        YAML is invalid but should be valid
  dont        YAML is valid but shouldn't be

libyaml-*     libyaml differs from normal
1-1-*         YAML 1.1 differs from normal
1-2-*         YAML 1.2 differs from normal
1-3-*         YAML 1.3 differs from normal
2-0-*         YAML 2.0 differs from normal

alias         The test uses anchors *and* aliases
anchor        The test uses anchors (but *not* aliases)
binary        The test encodes binary data
comment       The test uses YAML comments
complex-key   The test includes a mapping key which is not a scalar, but a
              sequence or mapping
directive     The test has directives
double        The test involves double quoted scalars
duplicate-key The test includes a duplicate mapping key
edge          The test is an edge case
empty         The test has empty scalars
empty-key     The test includes an empty mapping key `: value`
error         The test is about YAML that has errors
explicit-key  The test uses `?` for an explicit key
flow          The test has flow style
folded        The test uses '>' folded scalars
footer        The test has '...' footer tokens
header        The test has '---' header tokens
indent        The test is concerned with indentation issues
literal       The test uses '|' literal scalars
local-tag     The test uses a local tag `!foo`
mapping       The test is concerned with mapping issues
missing       The test has explicit pair with key or value missing
scalar        The test is concerned with scalar issues
sequence      The test is concerned with sequence issues
simple        The test uses simple YAML
single        The test involves single quoted scalars
spec          The test is a YAML 1.2 Spec example
tag           The test has tags
unknown-tag   The test uses an unknown tag from the standard YAML schema
              `!!foo`
whitespace    The test is concerned with whitespace issues
```
