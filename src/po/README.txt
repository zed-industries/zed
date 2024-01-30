TRANSLATING VIM MESSAGES

In this directory you will find xx.po files, where "xx" is a language code.
Each file contains the translation of English Vim messages for one language.
The files are in "po" format, used by the gettext package.  Please refer to
the gettext documentation for more information.

The GNU gettext library, starting with version 0.10.37, supports converting
messages from one encoding to another.  This requires that it was compiled
with HAVE_ICONV.  The result is that the messages may be in any encoding
supported by iconv and will be automatically converted to the currently used
encoding.

The GNU gettext library, starting with version 0.10.36, uses a new format for
some encodings.  This follows the C99 standard for strings.  It means that
when a multi-byte character includes the 0x5c byte, this is not recognized as
a backslash.  Since this format is incompatible with Solaris, Vim uses the old
format.  This is done by setting the OLD_PO_FILE_OUTPUT and OLD_PO_FILE_INPUT
environment variables.  When you use the Makefile in this directory that will
be done for you.  This does NOT work with gettext 0.10.36.  Don't use it, get
0.10.37.

Have a look at these helper scripts, they may be useful for you:
https://github.com/adaext/vim-menutrans-helper


ON MS-WINDOWS

The distributed files are generated on Unix, but this should also be possible
on MS-Windows.  Download the gettext packages, for example from:

	http://sourceforge.net/projects/gettext
or
	https://mlocati.github.io/articles/gettext-iconv-windows.html

You might have to do the commands manually.  Example:

   cd c:\vim\vim91
   mkdir runtime\lang\ja\LC_MESSAGES
   msgfmt -o runtime\lang\ja\LC_MESSAGES\vim.mo src\po\ja.po


WHEN THERE IS A MISTAKE

If you find there is a mistake in one of the translations, please report this
to the maintainer of the translation.  His/her e-mail address is in the
comments at the start of the file.  You can also see this with the ":messages"
command in Vim when the translation is being used.


CREATING A NEW PO FILE

We will use "xx.po" as an example here, replace "xx" with the name of your
language.

- Edit Make_all.mak to add xx to LANGUAGES and xx.mo to MOFILES, xx.po to
  POFILES and xx.ck to CHECKFILES.
- If the encoding of the translation text differs from the default UTF-8, add a
  corresponding entry in MOCONVERTED, specifying the required encoding.
- If you haven't done so already, run ./configure in the top vim directory
  (i.e. go up two directories) and then come back here afterwards.
- Execute these commands:
  % make vim.pot
  % msginit -l xx
  % rm vim.pot
  The first command will generate a vim.pot file which is used by msginit to
  generate a correct xx.po file.  After that vim.pot is not needed.
- The remaining work is like updating, see the next section.


UPDATING A PO FILE

If you are the maintainer of a .po file, this is how you update the file.  We
will use "xx.po" as an example here, replace "xx" with the name of your
language.

(1) Add new and changed messages from the Vim sources:

	make xx

    This will extract all the strings from Vim and merge them in with the
    existing translations.  Requires the GNU gettext utilities.
    Your original xx.po file will be copied to xx.po.orig

    -- After you do this, you MUST do the next three steps! --

(2) Translate
    See the gettext documentation on how to do this.  You can also find
    examples in the other po files.  You can use "gF" on the file name to see
    the context of the message.
    Search the po file for items that require translation:

	/fuzzy\|^msgstr ""\(\n"\)\@!

    Remove the "#, fuzzy" line after adding the translation.

    There is one special message:
	msgid "Messages maintainer: The Vim Project"
    You should include your name and E-mail address instead, for example:
	msgstr "Berichten übersetzt bei: John Doe <john@doe.org>"

(3) Remove unused messages (optional)
    Remove messages that have been marked as obsolete.
    Such messages start with "#~".

    The cleanup script will also do that (see next step).

(4) Clean up
    This is very important to make sure the translation works on all systems.
    Comment-out all non-translated strings.  There are two types:
    - items marked with "#, fuzzy"
    - items with an empty msgstr
    You can do this with the cleanup.vim script:

	:source cleanup.vim

    Background: on Solaris an empty msgstr results in an empty message; GNU
    gettext ignores empty strings and items marked with "#, fuzzy".

    This also removes the line numbers from the file, so that patches are not
    messed up by changes in line numbers and show the actual changes in the
    text.

(5) Check:

    While editing the .po file:
        :source check.vim

    From the command line:
	vim -S check.vim xx.po
	make xx.mo

    Look out for syntax errors and fix them.

(6) Local tryout:
    Vim normally picks up the .mo files from:
	    $VIMRUNTIME/lang/{lang}/LC_MESSAGES/vim.mo
    To try out the messages with Vim use:
	    make tryoutinstall
    And run Vim with $VIMRUNTIME set to ../runtime


USING GETTEXT WITHOUT ICONV

When using gettext which doesn't support iconv, the encoding of the .mo file
must match your active encoding.  For that you must convert and change
encoding of *.po file in advance of generating the *.mo file.  For example, to
convert ja.po to EUC-JP (supposed as your system encoding):

(1) Convert the file encoding:

	mv ja.po ja.po.orig
	iconv -f UTF-8 -t EUC-JP ja.po.orig > ja.po

(2) Rewrite charset declaration in the file:

    Open ja.po find this line:
	"Content-Type: text/plain; charset=UTF-8\n"
    You should change "charset" like this:
	"Content-Type: text/plain; charset=EUC-JP\n"

There are examples in the Makefile for the conversions already supported.
