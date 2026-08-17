YAML Test Suite
===============

Comprehensive Test Suite for YAML

## Overview

This repository contains data for testing the correctness of YAML processors.

The types of data include:

* Metadata about the test
  * Name (short phrase)
  * Tags
  * Description
* Input YAML
* Canonical output YAML
* Matching JSON
* Token stream notation
* Event stream notation
* Error data
* etc

To get a quick overview of the tests you can have a look at the [YAML Test
Matrix](http://matrix.yaml.info/), made from
<https://github.com/perlpunk/yaml-test-matrix>.

You can also view the latest test results from 15 different parsers in
[this Google sheet](https://tinyurl.com/2p97ah8a).

## Usage

The tests are available in 2 forms.
Files in the `src` directory encode all the data for YAML using YAML.
The data from these tests is also available in a form where each test
has its own directory.

For that, use the latest data release under
[https://github.com/yaml/yaml-test-suite/releases](
https://github.com/yaml/yaml-test-suite/releases):

    git clone https://github.com/yaml/yaml-test-suite -b data-YYYY-MM-DD

There are tests which have multiple similar subtests. Those subtests are
in their own numeric directories under the parent id, e.g.:

    VJP3/
    VJP3/00
    VJP3/00/===
    VJP3/00/error
    VJP3/00/in.yaml
    VJP3/00/test.event
    VJP3/01
    ...


The releases are made from the `data` branch, which is made from the data in
the YAML in the `main` branch.
You shouldn't use the data branch directly as the branch contains unreleased
commits which might be wrong, and it is squashed and force pushed from time to
time.

### Special Characters

The YAML files use a number of non-ascii unicode characters to indicate the
presence of certain characters that would be otherwise hard to read.

* `␣` is used for trailing space characters
* Hard tabs are reresented by one of:  (expanding to 4 spaces)
  * `———»`
  * `——»`
  * `—»`
  * `»`
* `↵` us used to show trailing newline characters
* `∎` is used at the end when there is no final newline character
* `←` indicates a carriage return character
* `⇔` indicates a byte order mark (BOM) character

Also these are used in test event output:

* `<SPC>` for a space character
* `<TAB>` for a tab character

## The `data` branch files

The YAML test files in the `src/` dir are turned into data files in the `data`
branch.
The `make data-update` command generates the `data` branch files under the
`./data/` directory.
For instance, a file `src/AB3D.yaml` will generate a `data/AB3D/` directory.

A YAML test file can have 1 or more tests.
Originally each file had one test, and all the data files were under
`data/AB3D/`.
If a YAML test file has more than one test, subdirectories are created:
`data/AB3D/00/`, `data/AB3D/01/`, `data/AB3D/02/`, etc.

The test files are:

* `===` -- The name/label of the test
* `in.yaml` -- The YAML input to be parsed or loaded
* `test.event` -- The event DSL produced by the parser test program
* `in.json` -- The JSON value that shoiuld load the same as `in.yaml`
* `out.yaml` -- The most normal output a dumper would produce
* `error` -- This file indicates the YAML should fail to parse
* `emit.yaml` -- Output an emitter would produce

## Makefile Targets

The Makefile has a number of targets for automating the process of adding new
tests and also preprocessing them into the `data` branch.

* `make data`

  Create a `data` worktree subdirectory with all the tests as data files.

* `make data-update`

  Update the `data` branch directory with the latest info in the `src`
  directory.

* `make export`

  Creates an `export.tsv` file with all the data from the `src` test files.
  This tsv data can be copied into a google spreadsheet.
  The [YAML parser playground](https://play.yaml.io/main/parser) has a button
  to copy a test to the same tsv form.

* `make import`

  Make a directory called `new` from a file named `import.tsv`.
  The `import.tsv` file should have data copied from a google spreadsheet.

* `make add-new`

  Copy the new tests under `new/` into `src/` to make a PR for new tests.

* `make testml`

  Generate `.tml` files under a `testml/` directory for all the suite tests.

* `make clean`

  Remove generated files and directories.

## Libaries using this test suite

* C
  * [libyaml](https://github.com/yaml/libyaml)
  * [libfyaml](https://github.com/pantoniou/libfyaml)
* C#
  * [YamlDotNet](https://github.com/aaubry/YamlDotNet)
* D
  * [dyaml](https://github.com/dlang-community/D-YAML)
* Delphi
  * [Neslib.Yaml](https://github.com/neslib/Neslib.Yaml)
* Haskell
  * [HsYAML](https://github.com/haskell-hvr/HsYAML)
* Java
  * [SnakeYAML Engine](https://bitbucket.org/asomov/snakeyaml-engine)
* Javascript
  * [yaml](https://github.com/eemeli/yaml)
* Nim
  * [NimYAML](https://github.com/flyx/NimYAML)
* Perl 5
  * [YAML::PP](https://github.com/perlpunk/YAML-PP-p5)

If your library is using the test suite, drop us a line and we can add it here.
It would also be nice if you could add a link back to this test suite.
