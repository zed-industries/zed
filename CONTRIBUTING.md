# Contributing to Vim

Patches are welcome in whatever form.
Discussions about patches happen on the [vim-dev][0] mailing list.
If you create a pull request on GitHub it will be
forwarded to the vim-dev mailing list.  You can also send your patch there
directly (but please note, the initial posting is subject to moderation).
In that case an attachment with a unified diff format is preferred.
Information about the mailing list can be found [on the Vim website][0]

A pull request has the advantage that it will trigger the Continuous
Integration tests, you will be warned of problems (you can ignore the coverage
warning, it's noisy).

Please consider adding a test.  All new functionality should be tested and bug
fixes should be tested for regressions: the test should fail before the fix and
pass after the fix.  Look through recent patches for examples and find help
with ":help testing".  The tests are located under "src/testdir".

Contributions will be distributed with Vim under the Vim license.  Providing a
change to be included implies that you agree with this and your contribution
does not cause us trouble with trademarks or patents.  There is no CLA to sign.

# Reporting issues

We use GitHub issues, but that is not a requirement.  Writing to the Vim
mailing list is also fine.

Please use the GitHub issues only for actual issues. If you are not 100% sure
that your problem is a Vim issue, please first discuss this on the Vim user
mailing list.  Try reproducing the problem without any of your plugins or settings:

    vim --clean

If you report an issue, please describe exactly how to reproduce it.
For example, don't say "insert some text" but say what you did exactly:
`ahere is some text<Esc>`.
Ideally, the steps you list can be used to write a test to verify the problem
is fixed.

Feel free to report even the smallest problem, also typos in the documentation.

You can find known issues in the todo file: `:help todo`.
Or open [the todo file][todo list] on GitHub to see the latest version.

# Syntax, indent and other runtime files

The latest version of these files can be obtained from the repository.
They are usually not updated with numbered patches.

If you find a problem with one of these files or have a suggestion for
improvement, please first try to contact the maintainer directly.
Look in the header of the file for the name and email address.

The maintainer will take care of issues and send updates to the Vim project for
distribution with Vim.

If the maintainer does not respond, contact the [vim-dev][0] mailing list.

# Translations

Translating messages and runtime files is very much appreciated!  These things
can be translated:
*   Messages in Vim, see [src/po/README.txt][1]
    Also used for the desktop icons.
*   Menus, see [runtime/lang/README.txt][2]
*   Vim tutor, see [runtime/tutor/README.txt][3]
*   Manual pages, see [runtime/doc/\*.1][4] for examples
*   Installer, see [nsis/lang/\*.nsi][5] for examples

The help files can be translated and made available separately.
See https://www.vim.org/translations.php for examples.

# How do I contribute to the project?

Please have a look at the following [discussion][6], which should give you some
ideas. Please also check the [develop.txt][7] helpfile for the recommended
style. Often it's also beneficial to check the surrounding code for the style
being used.

# I have a question

If you have some question on the style guide, please contact the [vim-dev][0]
mailing list. For other questions please use the [Vi Stack Exchange][8] website, the
[vim-use][9] mailing list or make use of the [discussion][10] feature here at github.

[todo list]: https://github.com/vim/vim/blob/master/runtime/doc/todo.txt
[0]: http://www.vim.org/maillist.php#vim-dev
[1]: https://github.com/vim/vim/blob/master/src/po/README.txt
[2]: https://github.com/vim/vim/blob/master/runtime/lang/README.txt
[3]: https://github.com/vim/vim/blob/master/runtime/tutor/README.txt
[4]: https://github.com/vim/vim/blob/master/runtime/doc/vim.1
[5]: https://github.com/vim/vim/blob/master/nsis/lang/english.nsi
[6]: https://github.com/vim/vim/discussions/13087
[7]: https://github.com/vim/vim/blob/master/runtime/doc/develop.txt
[8]: https://vi.stackexchange.com
[9]: http://www.vim.org/maillist.php#vim-use
[10]: https://github.com/vim/vim/discussions
