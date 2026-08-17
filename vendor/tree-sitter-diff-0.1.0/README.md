# `tree-sitter-diff`

[![CI][ci-badge]][ci-workflow]

_A [tree-sitter][tree-sitter] grammar for `diff`s_

Highlighting a `.diff` file:

<img src="assets/diff.png" width="500"/>

Injecting this grammar into [tree-sitter-git-commit][tree-sitter-git-commit]
in a verbose commit (`git commit --verbose`):

<img src="assets/helix-commit-with-diff.png" width="500"/>

[ci-badge]: https://github.com/the-mikedavis/tree-sitter-diff/actions/workflows/ci.yml/badge.svg
[ci-workflow]: https://github.com/the-mikedavis/tree-sitter-diff/actions/workflows/ci.yml
[tree-sitter]: https://tree-sitter.github.io/tree-sitter/
[tree-sitter-git-commit]: https://github.com/the-mikedavis/tree-sitter-git-commit
