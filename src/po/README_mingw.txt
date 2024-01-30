TRANSLATING VIM MESSAGES

This file explains how to create and maintain po files using
gnu-gettext.win32, a MINGW32 Windows port of gettext by Franco Bez
<franco.bez@gmx.de>.  You can find it at:

	http://home.a-city.de/franco.bez/gettext/gettext_win32_en.html

First read the README.txt file for the general remarks


The file that does the work is Make_ming.mak in the po directory. It is an
adaptation of the Unix Makefile, but it does NOT test the presence of any po,
pot, or mo files, so use it at your own risk but with care: it could even kill
your canary. It has been tested by me several times (and with different
languages) with success.

The make utility must be run from the po directory.

First of all you must set the environment variable LANGUAGE to xx, where xx is
the name of your language. You can do it from the command line or adding a
line to your autoexec.bat file: set LANGUAGE=xx. You must also add your
language to the Make_all.mak file in the lines LANGUAGES, MOFILES, POFILES,
and CHECKFILES. If the encoding of the translation text differs from the
default UTF-8, add a corresponding entry in MOCONVERTED, specifying the
required encoding.

If you don't have a xx.po file, you must create it with the command:

	make -f Make_ming.mak first_time

This will produce a new brand xx.po file with all the messages in Vim ready
for translation. Then you must source the cleanup.vim script from inside Vim;
it will comment the untranslated messages (now, all). I recommend to use
syntax highlighting so you can identify the untranslated messages easily.
You also must remove the '..\' that prepends the name of the source files.
(I don't no why, but make is unable to change the directory from po to src and
back to po, so all the work must be done from the po dir, hence the '..\')

Then you must go step (2) below.

If you are updating a po file you must follow the next steps (they are nearly
the same as in the Unix case, only the commands change):

(1) Add new and changed messages from the Vim sources:

	make -f Make_ming.mak xx

    This will extract all the strings from Vim and merge them in with the
    existing translations.  Requires the GNU gettext utilities.  Also requires
    unpacking the extra archive.
    Your original xx.po file will be copied to xx.po.orig

    -- After you do this, you MUST do the next three steps! --

(2) Translate
    See the gettext documentation on how to do this.  You can also find
    examples in the other po files.
    Search the po file for items that require translation:
	/\#\~   and also the fuzzy translations, /\#, fuzzy
    Remove "#~" and "#, fuzzy" after adding the translation.

    There is one special message:
	msgid "Messages maintainer: The Vim Project"
    You should include your name and e-mail address instead, for example:
	msgstr "Berichten übersetzt bei: John Doe <john@doe.org>"

(3) Clean up
    This is very important to make sure the translation works on all systems.
    Comment-out all non-translated strings.  There are two types:
    - items marked with "#, fuzzy"
    - items with an empty msgstr
    You can do this with the cleanup.vim script:

	:source cleanup.vim

(4) Check:

	vim -S check.vim xx.po
	make -f Make_ming.mak xx.mo

    Look out for syntax errors and fix them.

(5) This is an extra step, ;-). If you want the vim.mo file installed in your
    system you must run:

	make -f Make_ming.mak install

    This will create the xx\LC_MESSAGES directory (if it does not exist) and
    will copy vim.po to it.
    You can also use the following command to install all languages:

	make -f Make_ming.mak install-all

(6) Another extra step ;-)). The command:

	make -f Make_ming.mak clean

    will delete the temp files created during the process.

Suggestions will be welcomed.

Eduardo F. Amatria <eferna1@platea.pntic.mec.es>

Happy Vimming with NLS!!

vim:tw=78:
