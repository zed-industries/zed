en_US
en_US Hunspell Dictionary
Version 2015.08.24
Mon Aug 24 16:39:36 2015 -0400 [79c892e]
http://wordlist.sourceforge.net

README file for English Hunspell dictionaries derived from SCOWL.

These dictionaries are created using the speller/make-hunspell-dict
script in SCOWL.

The following dictionaries are available:

  en_US (American)
  en_CA (Canadian)
  en_GB-ise (British with "ise" spelling)
  en_GB-ize (British with "ize" spelling)

  en_US-large
  en_CA-large
  en_GB-large (with both "ise" and "ize" spelling)

The normal (non-large) dictionaries correspond to SCOWL size 60 and,
to encourage consistent spelling, generally only include one spelling
variant for a word.  The large dictionaries correspond to SCOWL size
70 and may include multiple spelling for a word when both variants are
considered almost equal.  Also, the general quality of the larger
dictionaries may also be less as they are not as carefully checked for
errors as the normal dictionaries.

To get an idea of the difference in size, here are 25 random words
only found in the large dictionary for American English:

  Bermejo Freyr's Guenevere Hatshepsut Nottinghamshire arrestment
  crassitudes crural dogwatches errorless fetial flaxseeds godroon
  incretion jalapeño's kelpie kishkes neuroglias pietisms pullulation
  stemwinder stenoses syce thalassic zees

The en_US and en_CA are the official dictionaries for Hunspell.  The
en_GB and large dictionaries are made available on an experimental
basis.  If you find them useful please send me a quick email at
kevina@gnu.org.

If none of these dictionaries suite you (for example, maybe you want
the larger dictionary but only use spelling of a word) additional
dictionaries can be generated at http://app.aspell.net/create or by
modifying speller/make-hunspell-dict in SCOWL.  Please do let me know
if you end up publishing a customized dictionary.

If a word is not found in the dictionary or a word is there you think
shouldn't be, you can lookup the word up at http://app.aspell.net/lookup
to help determine why that is.

General comments on these list can be sent directly to me at
kevina@gnu.org or to the wordlist-devel mailing lists
(https://lists.sourceforge.net/lists/listinfo/wordlist-devel).  If you
have specific issues with any of these dictionaries please file a bug
report at https://github.com/kevina/wordlist/issues.

IMPORTANT CHANGES FROM 2015.02.15:

The dictionaries are now in UTF-8 format instead of ISO-8859-1.  This
was required to handle smart quotes correctly.

ADDITIONAL NOTES:

The NOSUGGEST flag was added to certain taboo words.  While I made an
honest attempt to flag the strongest taboo words with the NOSUGGEST
flag, I MAKE NO GUARANTEE THAT I FLAGGED EVERY POSSIBLE TABOO WORD.
The list was originally derived from Németh László, however I removed
some words which, while being considered taboo by some dictionaries,
are not really considered swear words in today's society.

COPYRIGHT, SOURCES, and CREDITS:

The English dictionaries come directly from SCOWL 
and is thus under the same copyright of SCOWL.  The affix file is
a heavily modified version of the original english.aff file which was
released as part of Geoff Kuenning's Ispell and as such is covered by
his BSD license.  Part of SCOWL is also based on Ispell thus the
Ispell copyright is included with the SCOWL copyright.

The collective work is Copyright 2000-2015 by Kevin Atkinson as well
as any of the copyrights mentioned below:

  Copyright 2000-2015 by Kevin Atkinson

  Permission to use, copy, modify, distribute and sell these word
  lists, the associated scripts, the output created from the scripts,
  and its documentation for any purpose is hereby granted without fee,
  provided that the above copyright notice appears in all copies and
  that both that copyright notice and this permission notice appear in
  supporting documentation. Kevin Atkinson makes no representations
  about the suitability of this array for any purpose. It is provided
  "as is" without express or implied warranty.

Alan Beale <biljir@pobox.com> also deserves special credit as he has,
in addition to providing the 12Dicts package and being a major
contributor to the ENABLE word list, given me an incredible amount of
feedback and created a number of special lists (those found in the
Supplement) in order to help improve the overall quality of SCOWL.

The 10 level includes the 1000 most common English words (according to
the Moby (TM) Words II [MWords] package), a subset of the 1000 most
common words on the Internet (again, according to Moby Words II), and
frequently class 16 from Brian Kelk's "UK English Wordlist
with Frequency Classification".

The MWords package was explicitly placed in the public domain:

    The Moby lexicon project is complete and has
    been place into the public domain. Use, sell,
    rework, excerpt and use in any way on any platform.

    Placing this material on internal or public servers is
    also encouraged. The compiler is not aware of any
    export restrictions so freely distribute world-wide.

    You can verify the public domain status by contacting

    Grady Ward
    3449 Martha Ct.
    Arcata, CA  95521-4884

    grady@netcom.com
    grady@northcoast.com

The "UK English Wordlist With Frequency Classification" is also in the
Public Domain:

  Date: Sat, 08 Jul 2000 20:27:21 +0100
  From: Brian Kelk <Brian.Kelk@cl.cam.ac.uk>

  > I was wondering what the copyright status of your "UK English
  > Wordlist With Frequency Classification" word list as it seems to
  > be lacking any copyright notice.

  There were many many sources in total, but any text marked
  "copyright" was avoided. Locally-written documentation was one
  source. An earlier version of the list resided in a filespace called
  PUBLIC on the University mainframe, because it was considered public
  domain.

  Date: Tue, 11 Jul 2000 19:31:34 +0100

  > So are you saying your word list is also in the public domain?

  That is the intention.

The 20 level includes frequency classes 7-15 from Brian's word list.

The 35 level includes frequency classes 2-6 and words appearing in at
least 11 of 12 dictionaries as indicated in the 12Dicts package.  All
words from the 12Dicts package have had likely inflections added via
my inflection database.

The 12Dicts package and Supplement is in the Public Domain.

The WordNet database, which was used in the creation of the
Inflections database, is under the following copyright:

  This software and database is being provided to you, the LICENSEE,
  by Princeton University under the following license.  By obtaining,
  using and/or copying this software and database, you agree that you
  have read, understood, and will comply with these terms and
  conditions.:

  Permission to use, copy, modify and distribute this software and
  database and its documentation for any purpose and without fee or
  royalty is hereby granted, provided that you agree to comply with
  the following copyright notice and statements, including the
  disclaimer, and that the same appear on ALL copies of the software,
  database and documentation, including modifications that you make
  for internal use or for distribution.

  WordNet 1.6 Copyright 1997 by Princeton University.  All rights
  reserved.

  THIS SOFTWARE AND DATABASE IS PROVIDED "AS IS" AND PRINCETON
  UNIVERSITY MAKES NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR
  IMPLIED.  BY WAY OF EXAMPLE, BUT NOT LIMITATION, PRINCETON
  UNIVERSITY MAKES NO REPRESENTATIONS OR WARRANTIES OF MERCHANT-
  ABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF THE
  LICENSED SOFTWARE, DATABASE OR DOCUMENTATION WILL NOT INFRINGE ANY
  THIRD PARTY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.

  The name of Princeton University or Princeton may not be used in
  advertising or publicity pertaining to distribution of the software
  and/or database.  Title to copyright in this software, database and
  any associated documentation shall at all times remain with
  Princeton University and LICENSEE agrees to preserve same.

The 40 level includes words from Alan's 3esl list found in version 4.0
of his 12dicts package.  Like his other stuff the 3esl list is also in the
public domain.

The 50 level includes Brian's frequency class 1, words appearing
in at least 5 of 12 of the dictionaries as indicated in the 12Dicts
package, and uppercase words in at least 4 of the previous 12
dictionaries.  A decent number of proper names is also included: The
top 1000 male, female, and Last names from the 1990 Census report; a
list of names sent to me by Alan Beale; and a few names that I added
myself.  Finally a small list of abbreviations not commonly found in
other word lists is included.

The name files form the Census report is a government document which I
don't think can be copyrighted.

The file special-jargon.50 uses common.lst and word.lst from the
"Unofficial Jargon File Word Lists" which is derived from "The Jargon
File".  All of which is in the Public Domain.  This file also contain
a few extra UNIX terms which are found in the file "unix-terms" in the
special/ directory.

The 55 level includes words from Alan's 2of4brif list found in version
4.0 of his 12dicts package.  Like his other stuff the 2of4brif is also
in the public domain.

The 60 level includes all words appearing in at least 2 of the 12
dictionaries as indicated by the 12Dicts package.

The 70 level includes Brian's frequency class 0 and the 74,550 common
dictionary words from the MWords package.  The common dictionary words,
like those from the 12Dicts package, have had all likely inflections
added.  The 70 level also included the 5desk list from version 4.0 of
the 12Dics package which is in the public domain.

The 80 level includes the ENABLE word list, all the lists in the
ENABLE supplement package (except for ABLE), the "UK Advanced Cryptics
Dictionary" (UKACD), the list of signature words from the YAWL package,
and the 10,196 places list from the MWords package.

The ENABLE package, mainted by M\Cooper <thegrendel@theriver.com>,
is in the Public Domain:

  The ENABLE master word list, WORD.LST, is herewith formally released
  into the Public Domain. Anyone is free to use it or distribute it in
  any manner they see fit. No fee or registration is required for its
  use nor are "contributions" solicited (if you feel you absolutely
  must contribute something for your own peace of mind, the authors of
  the ENABLE list ask that you make a donation on their behalf to your
  favorite charity). This word list is our gift to the Scrabble
  community, as an alternate to "official" word lists. Game designers
  may feel free to incorporate the WORD.LST into their games. Please
  mention the source and credit us as originators of the list. Note
  that if you, as a game designer, use the WORD.LST in your product,
  you may still copyright and protect your product, but you may *not*
  legally copyright or in any way restrict redistribution of the
  WORD.LST portion of your product. This *may* under law restrict your
  rights to restrict your users' rights, but that is only fair.

UKACD, by J Ross Beresford <ross@bryson.demon.co.uk>, is under the
following copyright:

  Copyright (c) J Ross Beresford 1993-1999. All Rights Reserved.

  The following restriction is placed on the use of this publication:
  if The UK Advanced Cryptics Dictionary is used in a software package
  or redistributed in any form, the copyright notice must be
  prominently displayed and the text of this document must be included
  verbatim.

  There are no other restrictions: I would like to see the list
  distributed as widely as possible.

The 95 level includes the 354,984 single words, 256,772 compound
words, 4,946 female names and the 3,897 male names, and 21,986 names
from the MWords package, ABLE.LST from the ENABLE Supplement, and some
additional words found in my part-of-speech database that were not
found anywhere else.

Accent information was taken from UKACD.

My VARCON package was used to create the American, British, and
Canadian word list. 

Since the original word lists used in the VARCON package came
from the Ispell distribution they are under the Ispell copyright:

  Copyright 1993, Geoff Kuenning, Granada Hills, CA
  All rights reserved.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

  1. Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.
  2. Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.
  3. All modifications to the source code must be clearly marked as
     such.  Binary redistributions based on modified source code
     must be clearly marked as modified versions in the documentation
     and/or other materials provided with the distribution.
  (clause 4 removed with permission from Geoff Kuenning)
  5. The name of Geoff Kuenning may not be used to endorse or promote
     products derived from this software without specific prior
     written permission.

  THIS SOFTWARE IS PROVIDED BY GEOFF KUENNING AND CONTRIBUTORS ``AS
  IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
  LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
  FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL GEOFF
  KUENNING OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
  BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
  LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
  LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
  ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
  POSSIBILITY OF SUCH DAMAGE.

Build Date: Mon Aug 24 16:41:56 EDT 2015
Wordlist Command: mk-list --accents=strip en_US 60
===================================================
en_AU:
This dictionary was based on the en_GB Myspell dictionary 
which in turn was initially based on a subset of the 
original English wordlist created by Kevin Atkinson for 
Pspell and  Aspell and thus is covered by his original 
LGPL licence. 

The credit for this en_AU dictionary goes to:

Kelvin Eldridge (maintainer)
Jean Hollis Weber
David Wilson

- Words incorrect in Australian English removed
- a list from the previously removed words with corrected spelling was added
- a list of major rivers was added
- a list of place names was added
- a list of Australian mammals was added 
- a list of Aboriginal/Koori words commonly used was added

A total of 119,267 words are now recognized 
by the dictionary.

Of course, special thanks go to the editors of the 
en_GB dictionary (David Bartlett, Brian Kelk and 
Andrew Brown) which provided the starting point
for this dictionary.

The affix file is currently a duplicate of the en_AU.aff
created completely from scratch by David Bartlett and 
Andrew Brown, based on the published 
rules for MySpell and is also provided under the LGPL.

If you find omissions or bugs or have new words to 
add to the dictionary, please contact the en_AU 
maintainer at:

 "Kelvin" <audictionary@onlineconnections.com.au>



===================================================
en_CA:
en_CA Hunspell Dictionary
Version 2015.08.24
Mon Aug 24 16:39:36 2015 -0400 [79c892e]
http://wordlist.sourceforge.net

README file for English Hunspell dictionaries derived from SCOWL.

These dictionaries are created using the speller/make-hunspell-dict
script in SCOWL.

The following dictionaries are available:

  en_US (American)
  en_CA (Canadian)
  en_GB-ise (British with "ise" spelling)
  en_GB-ize (British with "ize" spelling)

  en_US-large
  en_CA-large
  en_GB-large (with both "ise" and "ize" spelling)

The normal (non-large) dictionaries correspond to SCOWL size 60 and,
to encourage consistent spelling, generally only include one spelling
variant for a word.  The large dictionaries correspond to SCOWL size
70 and may include multiple spelling for a word when both variants are
considered almost equal.  Also, the general quality of the larger
dictionaries may also be less as they are not as carefully checked for
errors as the normal dictionaries.

To get an idea of the difference in size, here are 25 random words
only found in the large dictionary for American English:

  Bermejo Freyr's Guenevere Hatshepsut Nottinghamshire arrestment
  crassitudes crural dogwatches errorless fetial flaxseeds godroon
  incretion jalapeño's kelpie kishkes neuroglias pietisms pullulation
  stemwinder stenoses syce thalassic zees

The en_US and en_CA are the official dictionaries for Hunspell.  The
en_GB and large dictionaries are made available on an experimental
basis.  If you find them useful please send me a quick email at
kevina@gnu.org.

If none of these dictionaries suite you (for example, maybe you want
the larger dictionary but only use spelling of a word) additional
dictionaries can be generated at http://app.aspell.net/create or by
modifying speller/make-hunspell-dict in SCOWL.  Please do let me know
if you end up publishing a customized dictionary.

If a word is not found in the dictionary or a word is there you think
shouldn't be, you can lookup the word up at http://app.aspell.net/lookup
to help determine why that is.

General comments on these list can be sent directly to me at
kevina@gnu.org or to the wordlist-devel mailing lists
(https://lists.sourceforge.net/lists/listinfo/wordlist-devel).  If you
have specific issues with any of these dictionaries please file a bug
report at https://github.com/kevina/wordlist/issues.

IMPORTANT CHANGES FROM 2015.02.15:

The dictionaries are now in UTF-8 format instead of ISO-8859-1.  This
was required to handle smart quotes correctly.

ADDITIONAL NOTES:

The NOSUGGEST flag was added to certain taboo words.  While I made an
honest attempt to flag the strongest taboo words with the NOSUGGEST
flag, I MAKE NO GUARANTEE THAT I FLAGGED EVERY POSSIBLE TABOO WORD.
The list was originally derived from Németh László, however I removed
some words which, while being considered taboo by some dictionaries,
are not really considered swear words in today's society.

COPYRIGHT, SOURCES, and CREDITS:

The English dictionaries come directly from SCOWL 
and is thus under the same copyright of SCOWL.  The affix file is
a heavily modified version of the original english.aff file which was
released as part of Geoff Kuenning's Ispell and as such is covered by
his BSD license.  Part of SCOWL is also based on Ispell thus the
Ispell copyright is included with the SCOWL copyright.

The collective work is Copyright 2000-2015 by Kevin Atkinson as well
as any of the copyrights mentioned below:

  Copyright 2000-2015 by Kevin Atkinson

  Permission to use, copy, modify, distribute and sell these word
  lists, the associated scripts, the output created from the scripts,
  and its documentation for any purpose is hereby granted without fee,
  provided that the above copyright notice appears in all copies and
  that both that copyright notice and this permission notice appear in
  supporting documentation. Kevin Atkinson makes no representations
  about the suitability of this array for any purpose. It is provided
  "as is" without express or implied warranty.

Alan Beale <biljir@pobox.com> also deserves special credit as he has,
in addition to providing the 12Dicts package and being a major
contributor to the ENABLE word list, given me an incredible amount of
feedback and created a number of special lists (those found in the
Supplement) in order to help improve the overall quality of SCOWL.

The 10 level includes the 1000 most common English words (according to
the Moby (TM) Words II [MWords] package), a subset of the 1000 most
common words on the Internet (again, according to Moby Words II), and
frequently class 16 from Brian Kelk's "UK English Wordlist
with Frequency Classification".

The MWords package was explicitly placed in the public domain:

    The Moby lexicon project is complete and has
    been place into the public domain. Use, sell,
    rework, excerpt and use in any way on any platform.

    Placing this material on internal or public servers is
    also encouraged. The compiler is not aware of any
    export restrictions so freely distribute world-wide.

    You can verify the public domain status by contacting

    Grady Ward
    3449 Martha Ct.
    Arcata, CA  95521-4884

    grady@netcom.com
    grady@northcoast.com

The "UK English Wordlist With Frequency Classification" is also in the
Public Domain:

  Date: Sat, 08 Jul 2000 20:27:21 +0100
  From: Brian Kelk <Brian.Kelk@cl.cam.ac.uk>

  > I was wondering what the copyright status of your "UK English
  > Wordlist With Frequency Classification" word list as it seems to
  > be lacking any copyright notice.

  There were many many sources in total, but any text marked
  "copyright" was avoided. Locally-written documentation was one
  source. An earlier version of the list resided in a filespace called
  PUBLIC on the University mainframe, because it was considered public
  domain.

  Date: Tue, 11 Jul 2000 19:31:34 +0100

  > So are you saying your word list is also in the public domain?

  That is the intention.

The 20 level includes frequency classes 7-15 from Brian's word list.

The 35 level includes frequency classes 2-6 and words appearing in at
least 11 of 12 dictionaries as indicated in the 12Dicts package.  All
words from the 12Dicts package have had likely inflections added via
my inflection database.

The 12Dicts package and Supplement is in the Public Domain.

The WordNet database, which was used in the creation of the
Inflections database, is under the following copyright:

  This software and database is being provided to you, the LICENSEE,
  by Princeton University under the following license.  By obtaining,
  using and/or copying this software and database, you agree that you
  have read, understood, and will comply with these terms and
  conditions.:

  Permission to use, copy, modify and distribute this software and
  database and its documentation for any purpose and without fee or
  royalty is hereby granted, provided that you agree to comply with
  the following copyright notice and statements, including the
  disclaimer, and that the same appear on ALL copies of the software,
  database and documentation, including modifications that you make
  for internal use or for distribution.

  WordNet 1.6 Copyright 1997 by Princeton University.  All rights
  reserved.

  THIS SOFTWARE AND DATABASE IS PROVIDED "AS IS" AND PRINCETON
  UNIVERSITY MAKES NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR
  IMPLIED.  BY WAY OF EXAMPLE, BUT NOT LIMITATION, PRINCETON
  UNIVERSITY MAKES NO REPRESENTATIONS OR WARRANTIES OF MERCHANT-
  ABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF THE
  LICENSED SOFTWARE, DATABASE OR DOCUMENTATION WILL NOT INFRINGE ANY
  THIRD PARTY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.

  The name of Princeton University or Princeton may not be used in
  advertising or publicity pertaining to distribution of the software
  and/or database.  Title to copyright in this software, database and
  any associated documentation shall at all times remain with
  Princeton University and LICENSEE agrees to preserve same.

The 40 level includes words from Alan's 3esl list found in version 4.0
of his 12dicts package.  Like his other stuff the 3esl list is also in the
public domain.

The 50 level includes Brian's frequency class 1, words appearing
in at least 5 of 12 of the dictionaries as indicated in the 12Dicts
package, and uppercase words in at least 4 of the previous 12
dictionaries.  A decent number of proper names is also included: The
top 1000 male, female, and Last names from the 1990 Census report; a
list of names sent to me by Alan Beale; and a few names that I added
myself.  Finally a small list of abbreviations not commonly found in
other word lists is included.

The name files form the Census report is a government document which I
don't think can be copyrighted.

The file special-jargon.50 uses common.lst and word.lst from the
"Unofficial Jargon File Word Lists" which is derived from "The Jargon
File".  All of which is in the Public Domain.  This file also contain
a few extra UNIX terms which are found in the file "unix-terms" in the
special/ directory.

The 55 level includes words from Alan's 2of4brif list found in version
4.0 of his 12dicts package.  Like his other stuff the 2of4brif is also
in the public domain.

The 60 level includes all words appearing in at least 2 of the 12
dictionaries as indicated by the 12Dicts package.

The 70 level includes Brian's frequency class 0 and the 74,550 common
dictionary words from the MWords package.  The common dictionary words,
like those from the 12Dicts package, have had all likely inflections
added.  The 70 level also included the 5desk list from version 4.0 of
the 12Dics package which is in the public domain.

The 80 level includes the ENABLE word list, all the lists in the
ENABLE supplement package (except for ABLE), the "UK Advanced Cryptics
Dictionary" (UKACD), the list of signature words from the YAWL package,
and the 10,196 places list from the MWords package.

The ENABLE package, mainted by M\Cooper <thegrendel@theriver.com>,
is in the Public Domain:

  The ENABLE master word list, WORD.LST, is herewith formally released
  into the Public Domain. Anyone is free to use it or distribute it in
  any manner they see fit. No fee or registration is required for its
  use nor are "contributions" solicited (if you feel you absolutely
  must contribute something for your own peace of mind, the authors of
  the ENABLE list ask that you make a donation on their behalf to your
  favorite charity). This word list is our gift to the Scrabble
  community, as an alternate to "official" word lists. Game designers
  may feel free to incorporate the WORD.LST into their games. Please
  mention the source and credit us as originators of the list. Note
  that if you, as a game designer, use the WORD.LST in your product,
  you may still copyright and protect your product, but you may *not*
  legally copyright or in any way restrict redistribution of the
  WORD.LST portion of your product. This *may* under law restrict your
  rights to restrict your users' rights, but that is only fair.

UKACD, by J Ross Beresford <ross@bryson.demon.co.uk>, is under the
following copyright:

  Copyright (c) J Ross Beresford 1993-1999. All Rights Reserved.

  The following restriction is placed on the use of this publication:
  if The UK Advanced Cryptics Dictionary is used in a software package
  or redistributed in any form, the copyright notice must be
  prominently displayed and the text of this document must be included
  verbatim.

  There are no other restrictions: I would like to see the list
  distributed as widely as possible.

The 95 level includes the 354,984 single words, 256,772 compound
words, 4,946 female names and the 3,897 male names, and 21,986 names
from the MWords package, ABLE.LST from the ENABLE Supplement, and some
additional words found in my part-of-speech database that were not
found anywhere else.

Accent information was taken from UKACD.

My VARCON package was used to create the American, British, and
Canadian word list. 

Since the original word lists used in the VARCON package came
from the Ispell distribution they are under the Ispell copyright:

  Copyright 1993, Geoff Kuenning, Granada Hills, CA
  All rights reserved.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

  1. Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.
  2. Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.
  3. All modifications to the source code must be clearly marked as
     such.  Binary redistributions based on modified source code
     must be clearly marked as modified versions in the documentation
     and/or other materials provided with the distribution.
  (clause 4 removed with permission from Geoff Kuenning)
  5. The name of Geoff Kuenning may not be used to endorse or promote
     products derived from this software without specific prior
     written permission.

  THIS SOFTWARE IS PROVIDED BY GEOFF KUENNING AND CONTRIBUTORS ``AS
  IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
  LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
  FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL GEOFF
  KUENNING OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
  BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
  LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
  LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
  ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
  POSSIBILITY OF SUCH DAMAGE.

Build Date: Mon Aug 24 16:41:58 EDT 2015
Wordlist Command: mk-list --accents=strip en_CA 60
===================================================
en_GB:
This dictionary was initially based on a subset of the
original English wordlist created by Kevin Atkinson for
Pspell and  Aspell and thus is covered by his original
LGPL licence. 

It has been extensively updated by David Bartlett, Brian Kelk
and Andrew Brown:
- Numerous Americanism have been removed;
- Numerous American spellings have been corrected;
- Missing words have been added;
- Many errors have been corrected;
- Compound hyphenated words have been added where appropriate.

Valuable inputs to this process were received from many other
people - far too numerous to name. Serious thanks to you all
for your greatly appreciated help.

This wordlist is intended to be a good representation of
current modern British English and thus it should be a good
basis for Commonwealth English in most countries of the world
outside North America.

The affix file has been created completely from scratch
by David Bartlett and Andrew Brown, based on the published
rules for MySpell and is also provided under the LGPL.

In creating the affix rules an attempt has been made to
reproduce the most general rules for English word
formation, rather than merely use it as a means to
compress the size of the dictionary. It is hoped that this
will facilitate future localisation to other variants of
English.

---

This is a locally hosted copy of the English dictionaries with fixed dash handling and new ligature and phonetic suggestion support extension:
http://extensions.openoffice.org/en/node/3785

Original version of the en_GB dictionary:
http://www.openoffice.org/issues/show_bug.cgi/id=72145

OpenOffice.org patch and morphological extension.

The morphological extension based on Wordlist POS and AGID data
created by Kevin Atkinson and released on http://wordlist.sourceforge.net.

Other fixes:

OOo Issue 48060 - add numbers with affixes by COMPOUNDRULE (1st, 111th, 1990s etc.)
OOo Issue 29112, 55498 - add NOSUGGEST flags to taboo words
New REP items (better suggestions for accented words and a few mistakes)
OOo Issue 63541 - remove *dessicated

2008-12-18 nemeth AT OOo

---

MARCO A.G.PINTO:
Since the dictionary hasn't been updated for many years,
I decided to fork it in order to add new words and fixes.

I even added words such as common names of software and hardware.

I grabbed Mozilla's version since it wasn't obfuscated. Alexandro Colorado and I
tried to unmunch the OpenOffice version but all we got was garbage.

The dictionary icon in the Extension Manager was designed by Pedro Marques.

The sources used to verify the spelling of the words I included in the dictionary:
1) Oxford Dictionaries;
2) Collins Dictionary;
3) Macmillan Dictionary;
4) Wiktionary (used with caution);
5) Wikipedia (used with caution);
6) Physical dictionaries

Main difficulties developing this dictionary:
1) Proper names;
2) Possessive forms;
3) Plurals.

Please let Marco A.G.Pinto know of any errors that you find:
E-mail: marcoagpinto@mail.telepac.pt
  Site: http://marcoagpinto.cidadevirtual.pt/proofingtoolgui.html


V2.00 - 25-AUG-2013
-------------------
Added the words:
1) Thunderbird
2) Firefox
3) LibreOffice
4) Ubuntu
5) Amiga (from Commodore)
6) mIRC
7) Facebook
8) ChanServ
9) NickServ
10) IRC
11) PhD (+plural)
12) Ph.D.
13) screenshot (+plural)
14) PowerPC
15) internship (+plural)
16) D-day
17) Wikipedia
18) committer (+plural)
19) anymore
20) GitHub
21) Blu-ray (+plural)
22) airplane (+plural)
23) unsubscribe


V2.01 - 31-AUG-2013
-------------------
Added the words:
24) cyberterrorist (+plural)
25) cyberterrorism
26) DVD-RW
27) DVD-RAM
28) DVD+RW
29) cyberattack (+plural)
30) encyclopedia (+plural)
31) spyware
32) adware
33) malware
34) Trojan  (First letter capitalized - Oxford Dictionary)
35) Freeware
36) rootkit (+plural)
37) keylogger (+plural)
38) keylogging
39) subprogram (+plural)
40) webcam (+plural)
41) antivirus
42) Coca-Cola
43) fuckable
44) fuckability
45) NSA
46) GnuPG
47) melancholiac
48) scammer (+plural)
49) XML
50) movie (+plural) (Oxford Dictionary says it is chiefly North American)
51) YouTube
52) cyberwar (+plural)


V2.02 - 4-SEP-2013
-------------------
Added the words:
53) aargh
54) ABC (+plural)
55) admin
56) ADSL
57) JPEG
58) megapixel (+plural)
59) PDF
60) HDD
61) FPU
62) boing
63) google
64) Google (capitalized-company)
65) IDE
66) fallback
67) re-establish
68) MUFON
69) transborder (+plural)
70) leveraging
71) geek (+plural)
72) bestseller (+plural)
73) playlist (+plural +ing)
74) PowerPoint
75) DALnet
76) meetup (Oxford Dictionary says it is chiefly North American)
77) traineeship (Fix - it was only inserted in plural)
78) phishing
79) phisher (+plural)
80) botnet  (+plural)
81) guestbook
82) widescreen (fix - it was inserted wide-screen)
83) shortcut (+plural)
84) retrocession
85) hardcover (Oxford Dictionary says it is chiefly North American)
86) Bryon (name of person)
87) Aleister (name of person)
88) teleport
89) teleportation
90) etheric
91) telepath
92) trustable
93) contactee (+plural)
94) chakra (+plural)
95) Adamski (name of person)
96) diapers (Added plural)
97) Pleiades (place in the universe)
98) PGP
99) lineman
100) linemen
101) videography
102) videographer
103) HDMI
104) mentoring
105) mentored (merged to mentor+ed)
106) anytime
107) skywatch (+er +ers +er's)
108) statewide
109) caseload
110) experiencer (+plural)
111) experienceable
112) blog (+plural)
113) blogging
114) blogged
115) blogger
116) crimeware
117) multisession
118) cyberstalking
119) cyberstalker (+plural)
120) PCMCIA
121) failover
122) placeholder (+plural)
123) showstopper (+plural)
124) showstopping
125) API
126) SeaMonkey
127) markup
128) watchdogging
129) watchdogged
130) cockapoo
131) Elohim (name)
132) undine (+plural)
133) Plutonian
134) technostructure
135) uncommercial
136) uncompetitively
137) uncompetitiveness
138) uncomplicatedly
139) uncomplicatedness
140) graphitic
141) blacklead
142) ufological
143) UEFA


V2.03 - 13-SEP-2013
-------------------
144) Merged uncompetitiveness and uncompetitively into uncompetitive.
145) e-book (+plural)
146) e-business
147) e-card (+plural)
148) e-crime (+plural)
149) e-criminal (plural)
150) infomediary
151) infomercial  (Oxford Dictionary says it is chiefly North American)
152) infonaut
153) SQL
154) cybercafe
155) cyberbullying
156) cyberbully
157) microcontroller
158) microcircuitry
159) PDFs (plural)
160) AGP
161) BIOS (computer)
162) bio (+plural) -> (Oxford Dictionary->informal->a biography)
163) biofuel
164) biodata
165) biodefence
166) biodiesel
167) bioconversion
168) biocomputer
169) biocomputing
170) bioclimatic
171) biochip
172) dpi
173) BitTorrent
174) spellcheck (+ er)
175) GPS
176) GIF
177) MPEG
178) NGO
179) NTSC
180) psychotomimetic
181) OLED
182) biocircuit
183) subwoofer
184) stereophonically
185) stereophony
186) subclinical
187) electro-oculography
188) electro-oculographic
189) electrochemistry
190) electrochemist
191) triplexed (added +ed to triplex)
192) sextodecimo (+plural)
193) thermography
194) thermographic
195) thermogram (+plural)
196) thermograph
197) infrared
198) fetus (+plural)
199) unhatched
200) unharmful (added +un to harmful)
201) open-source
202) gigabit (+plural)
203) gigaflop (+plural)
204) giganotosaurus
205) gigantism
206) polyploid (+plural)
207) polyploidy
208) intracellularly (added ly to intracellular)
209) daughterboard
210) daughtercard
211) transmissivity (+plural)
212) complementarities (plural)
213) RNA
214) polysaccharide (singular)
215) bioplastic
216) agonist
217) agonism
218) app (+plural)
219) Xanax
220) X-rated
221) subtotalling
222) subtotalled
223) derivational
224) Bergström (Swedish name)
225) 104+105 (placed in individual words due to a bug in the .AFF)


V2.04 - 20-SEP-2013
-------------------
226) spoofer (merged into spoof)
227) spoofery
228) spoofable
229) PayPal
230) distro (+plural)
231) geoinformation
232) geolocation
233) geolocate
234) favicon
235) gameplay
236) webmaster (+plural)
237) holler (+ ed)
238) chemtrail (+plural)
239) Günther (name)
240) twopenn'orth (John Wilcock from the Mozilla ML)
241) reinstallation
242) Alexandro (name)
243) URLs (plural)
244) discombobulate (+ed - Ian Neal from the Mozilla ML)
245) discombobulation (Ian Neal from the Mozilla ML)
246) Asturias
247) Asturian
248) LanguageTool (software)
249) UI (Oxford: User Interface)
250) pootle
251) pootling
252) counterclaim (+ed)
253) sublicense (+ed)
254) double-click (+ed +ing)
255) Pootle (capitalised, for the utility)
256) toolbar (+plural)
257) joypad
258) PNP
259) nanomaterial
260) nanoscale
261) virtualise (+s +d +ing)
262) virtualisation (+plural)
263) voicemail
264) steadier (added un)
265) steadiest (added un)
266) sci-fi (Oxford Dictionary: informal)
267) namespace
268) online (both on-line and online exist)
269) webmail
270) hardcover (removed, because in en_GB is hardback, thanks Ian Neal)
271) likelihoods (merged into likelihood with an s code)
272) postnuptial
273) Coloradan (relating Colorado: noun and adjective: Oxford)
274) LaVey (name)
275) Belial (name)
276) Rhadamanthus (name)
277) Sarpedon (name)
278) Pelops (name)
279) sociopath (+plural +ic +y)
280) schizo-affective
281) walk-through
282) unsexy (+ier + iest)
283) handheld
284) filename (+plural)
285) unhide (added un to hide)
286) overlayer (merged into overlay +plural)
287) overinsured
288) overinsurance
289) overkeen
290) overhit (+s +ing)
291) Chiu (name)
292) squawfish (+plural)
292) podcast (+able +er +ing)
293) autocorrect (+s +ed)
294) autocomplete (+s +ed)
295) waka
296) watercraft
297) spacecrafts (added plural - Oxford)
298) weatherboards (added plural - Oxford)
299) firewall (added +ed +ing)
300) firestorm (+plural)


V2.05 - 7-OCT-2013
------------------
301) GPU (+plural)
302) AMD (hardware company)
303) wireframe
304) leverages (with s)
305) tooltip (+plural)
306) hyperlink (+plural +ed)
307) wirelessly (merged into wireless)
308) ultraconservatism
309) ultramicroscope
310) ultramicroscopic
311) enigmatical
312) enharmonically (merged into enharmonic)
313) René (name)
314) Kant's
315) Kantian
316) Kantianism
317) Confucianist
318) Azrael (name)
319) exemplarily (merged into exemplary)
320) exemplarity
321) wholistic (+ally) (Oxford: variant spelling of holistic)
322) homeopathy (Oxford: also homoeopathy)
323) homeopathic (+ally)
324) homeopathist
325) Ashtar (name)
326) teleporting (added "ing" to "teleport")
327) Dreamtime
328) Patricia (name)
329) Patrícia (name with accent in "i")
330) videographics
331) megastar (informal)
332) songcraft
333) fado (+plural Oxford: Portuguese song)
334) divagate (+s +ed)
335) divagation
336) biopharmaceutics
337) WinZip (software)
338) WinRAR (software)
339) NTFS (file system for Windows)
340) UDF (file system)
341) subfolder (+plural)
342) Ubuntu's (added the ')
343) USB
344) smartphone (+plural)
345) rescript (added re into script)
346) OpenGL
347) GPL
348) LGPL
349) shutdown
350) uninstall (+s +ing +ed +er)
351) fundraise (+s +er +ing)
352) googled (merged into google)
353) pyjamaed (merged into pyjama)
354) hackerspace
355) miscreation (merged mis into creation)
356) quinary
357) senary
358) septenary
359) octonary
360) nonary
361) denary
362) duodenary
363) juvenilia
364) megastore
365) lightsabre (+Plural Oxford: from Star Wars)
366) ultracold
367) cerebrospinal
368) midbrain
369) hindbrain
370) brainstem
371) pluripotent
372) microcephaly (+lic)
373) microcephalous
374) hippocampus
375) hippocampi
376) clavicular
377) megastardom (informal)


V2.06 - 1-NOV-2013
------------------
378) ongoingness (merged into ongoing)
379) hacktivist (+plural)
380) hacktivism
381) cybercrime (+plural)
382) cybercriminal (+plural)
383) leakers (plural)
384) NSA's (added the 's)
385) infringers (plural)
386) username
387) Moodle
388) breathalyse
389) unappropriated
390) defalcator (+plural)
391) imponderably (merged into imponderable)
392) imponderability
393) forecasted (merged into forecast)
394) salvific (Oxford: adjective, Theology)
395) dualist
396) dualistic (+ally)
397) infographic
398) cybernaut
399) prepayable (merged into prepay)
400) prepayability (merged into prepay)
401) monoecious
402) monoecy
403) passerine (+plural)
404) Google's (added the 's)
405) subedit (+s +ed +ing)
406) subeditor
407) overhitting (merged into overhit)
408) countback
409) volleyer (merged into volley)
410) standalone (removed - According to Oxford is: stand-alone)
411) stroganoff
412) timeshare (+s +ed)
413) submodify (+er +ies +ing +tion)
414) timeline (+plural)
415) trainspotter (+plural)
416) trainspotting
417) burka
418) Brezhnev (Russian name)
419) Gorbachev (Russian name)
420) preset (+s)
421) Wicca
422) Wiccan
423) archangelic (merged into archangel)
424) angelology
425) typologist (merged into typology)
426) Wi-Fi
427) WC
428) wiki (yes, it exists in Oxford)
429) Wikipedian
430) LinkedIn (Business Social Network)
431) interlinkage
432) monotype
433) spiritlessly (merged into spiritless)
434) spiritlessness (merged into spiritless)
435) spiritism
436) spiritist
437) spiritistic
438) (merged spirit's into spirit)
439) bestselling
440) subterraneous
441) subterraneously (merged into subterraneous)
442) subterminal
443) viscousness (merged into viscous)
444) imperatrix
445) imperatrices
446) Metallica (+'s - the heavy metal band)
447) SMS (+plural +ing +ed)
448) MMS (+plural +ing +ed)
449) infowar
450) wipeable (merged into wipe)
451) multiplayer
452) (merged multitasking into multitask)
453) multitasked (merged into multitask)
454) multitasker (merged into multitask)
455) petabyte (+plural)
456) Bluetooth (+ed)
457) terabyte (+plural)
458) metadata
459) metafile
460) netiquette
461) DSL
462) listenership
463) videoconference (+ing)
464) narrowcast (+ed +ing +er)
465) cypherpunk (+plural)
466) MSN
467) viewport
468) Hezbollah
469) unibody (+plural)
470) escargot
471) yogurt (+plural)
472) knifepoint
473) guerrilla (fix: it had plural only and 's)
474) teacherly
475) varicella
476) herpesvirus
477) postnatally (merged into postnatal)
478) echoer (merged into echo)
479) echoey (merged into echo)
480) echoless (merged into echo)
481) (merged echo's into echo)
482) (merged echoic into echo)
483) Transifex
484) biogeochemical
485) biogeochemist
486) biogeochemistry
487) centremost
488) workpiece
489) WordPad
490) netbook (+plural)
491) Atlantean
492) Tir-na-nog
493) Hadean
494) Edenic (merged into Eden)
495) Midgard
496) Gehenna
497) Perseus
498) Andromeda
499) Pegasus
500) originalism
501) pastiched (merged into pastiche)
502) choirman
503) choirmen
504) conservatoire
505) hindwing
506) something's (Oxford - something's behalf - merged into something)
507) Croats (merged into Croat)
508) Bahamian
509) Qatari
510) Belorussia
511) Rwandese
512) Mongols (plural)
513) mongol (+plural - both in lower case - offensive - Down’s syndrome)
514) counterterrorism
515) counterterrorist
516) countermine (+ing)
517) pepperminty
518) duckwalk (+ed)
519) Fátima (name)
520) Fatima (name without accent)
521) fatidic
522) Nostradamus (name)
523) monocrat (+ic)
524) monocracy (+plural)
525) omnicompetent
526) omnicompetence
527) omnisexual
528) omnisexuality
529) ureteric
530) ureteral
531) vinculum
532) vincular
533) amygdala
534) amygdalae
535) BCG
536) hyperkeratosis
537) monolingually (merged into monolingual)
538) monoglot
539) polyglottal
540) polyglottic
541) polyglottism
542) mentoring + mentored (merged to mentor - The .AFF had no bug after all)
543) Afrotropical
544) unholier (merged into unholy)
545) unholiest (merged into unholy)
546) unholiness (merged into unholy)


V2.07 - 27-NOV-2013
-------------------
547) newsfeed
548) telerecording
549) televiewer
550) televiewing
551) radiogenic (+ally)
552) sRGB
553) RGB
554) tech (Oxford - informal)
555) infotech
556) bandolero (+plural)
557) honeytrap
558) gangboard
559) gangsta (Oxford - black slang)
560) bottlescrew
561) shrimper (+plural)
562) longline
563) prawner (merged into prawn)
564) wanna (Oxford - informal - want to; want a)
565) Wankie
566) wanky (Oxford - British vulgar slang)
567) kbyte (+plural)
568) maxwell (if in lower case it is a physics measure)
569) bushelful (+plural)
570) kilometric (merged into kilometre)
571) fibreless (merged into fibre)
572) midline
573) inbox
574) linkbait
575) pseudorandom
576) pseudorandomly (merged into pseudorandom)
577) pseudoscience
578) pseudoscientific
579) pseudoscientist
580) parapsychological
581) parapsychologically (merged into parapsychological)
582) parapsychic
583) psychometrist (merged into psychometry)
584) psychometrician
585) demonology (plural + demonological + demonologist)
586) vampirism
587) quickstepping (merged into quickstep)
588) quickstepped (merged into quickstep)
589) Solomonic (merged into Solomon)
590) millenary (+plural)
591) Floréal
592) Amazonia
593) timeout
594) undelete (+ed +ing)
595) screenable (merged into screen)
596) screenful (merged into screen)
597) preselection
598) preselective
599) techie (+plural - Oxford: informal)
600) techy (Oxford: informal)
601) chargrill (+ed)
602) technicals (added plural)
603) knockout
604) speedball
605) decametre (+ic)
606) decigram
607) decigramme
608) magnifico (Oxford: informal)
609) magnificoes (Oxford: informal)
610) postpositive (+vely)
611) intentionalism
612) grottoed (merged into grotto)
613) preadolescent
614) preambular
615) summariness (merged into summary)
616) summational
617) summative
618) Zeuxis (Oxford: Greek painter)
619) zettabyte
620) rollerball
621) autoroute
622) backstreets (plural)
623) downfold
624) masterdom
625) masterhood
626) masterless (merged into master)
627) zinced (merged into zinc)
628) cobaltic (merged into cobalt)
629) cobaltous
630) combust
631) combustor
632) multithreaded
633) multithreading
634) netizen (+plural - Oxford: informal)
635) Photoshop (Oxford: +s +ing +ed)
636) rangefinder
637) microcapsule
638) colonoscope
639) colonoscopy (+plural)
640) hypnotherapist (+plural)
641) abductee (+plural)
642) ET (+plural)
643) Harzan (name of person)
644) Denise (name of person)
645) celeb (+plural - Oxford: informal)
646) CEO (added +s and 's)
647) CNN (added 's)
648) scats (merged plural into scat)
649) synchronicity
650) Sananda (name of person)
651) micropyle
652) spermatozoid (+plural)


V2.08 - 1-JAN-2014
------------------
653) Ilachinski (name of person)
654) journaling (fix: it had two ll)
655) journaled (fix: it had two ll)
656) weathergirl
657) greyscale
658) rollback
659) pushback
660) patball
661) blowout
662) debut (+ed +ing)
663) debutante
664) proaction
665) proactively (merged into proactive)
666) proactivity
667) MUFON's (added 's)
668) intension (+al +ally)
669) conjecturable (merged into conjecture)
670) hypercorrect
671) hypercorrection
672) reduplicative
673) redwing
674) underwing (+plural)
675) nymphal (merged into nymph)
676) nymphean
677) Persephone (name)
678) myriapod
679) pictography
680) hieroglyphical
681) hieroglyphically (merged into hieroglyphical)
682) interlanguage
683) correlativity
684) predicability
685) sulphadiazine
686) biopharmaceutical (+plural)
687) dealmaker
688) dealmaking
689) subclause
690) dischargeable (merged into chargeable)
691) disciplic
692) discipular
693) tachistoscope
694) tachistoscopic (merged into tachistoscope)
695) tachistoscopically (merged into tachistoscope)
696) Dropbox
697) headhunt (+er +ing)
698) VCRs (plural)
699) videocassette
700) stuntwoman
701) stuntwomen
702) Steadicam
703) filmstrip
704) shirtsleeved (merged into shirtsleeve)
705) warez (Oxford: informal)
706) cybersex
707) cybershop (+s +ing +ed)
708) cybersecurity
709) cyberphobia
710) cyberphobe
711) cyberphobic
712) cyberneticist
713) multivalve
714) doomy
715) doomier (merged into doomy)
716) doomily (merged into doomy)
717) doominess (merged into doomy)
718) doomiest (merged into doomy)
719) satanically (merged into satanic)
720) monolatry
721) monolater
722) monolatrist
723) monolatrous
724) picnicky (merged into picnic)
725) fastball
726) curveball
727) fetal
728) EPROM
729) Perl
730) shovelware
731) prosthetically (merged into prosthetic)
732) microsurgeon
733) microsurgical
734) bistoury (+plural)
735) vaselined
736) OpenPGP
737) alienability
738) survivorship
739) refrigeratory
740) desiccative (merged into desiccate)
741) replaced spongelike with sponge-like (Fix - Oxford)
742) oystering (merged into oyster)
743) cephalopod
744) milreis (currency)
745) cruzado (currency) (+plural)
746) kwanza (currency) (+plural)
747) dobra (currency)
748) bitcoin (+plural)
749) spreadsheeting (merged into spreadsheet)
750) DDoS
751) bloggable (merged into blog)
752) hectarage
753) megalitre
754) Svedberg
755) petaflop
756) iPhone
757) iPad
758) criminalistics
759) triable
760) sequestrable (merged into sequestrate)
761) sequestrator
762) sociolegal
763) orthomolecular
764) cryopreserve
765) cryopreservation (merged into cryopreserve)
766) allogeneic
766) thrombose (fixed thromboses + added ed)
767) immunotherapy
768) shadowgraph
769) radiograph
770) microphysics
771) microphysical
772) ionizable (merged into ionize)
773) intramolecular
774) intramolecularly (merged into intramolecular)
775) superspace
776) metamaterial (+plural)
777) isoelectronic
778) seventhly
779) UVA
780) radioscopic (merged into radioscopy)
781) antigravity
782) spacewalk (+er)
783) Geminis (plural - merged into Gemini)
784) Geminian
785) xenology
786) xenologist
787) cockfight (+s +ing)
788) neuroticism
789) megalomanic
790) anaclitic
791) megalomaniacal
792) megaflop (+plural)
793) kilocalorie
794) multigym
795) powerlifting
796) powerlifter
797) kettlebell
798) translunar
799) checkbox (+plural)
800) duplexity
801) preagricultural
802) admins (plural)
803) housefather
804) housemother
805) mainstreamed (merged into mainstream)
806) merged mainstreamer into mainstream
807) remediate
809) remediation (merged into remediate)
810) biopiracy
811) sublittoral
812) biogeography
813) biogeographic (merged into biogeography)
814) biogeographer
815) biogeographically
816) biogeographical
817) excretive (merged into excrete)
818) hyperparasite
819) hyperparasitic (merged into hyperparasite)
820) hyperparasitism
821) acellular
822) charcuterie (+plural)
823) chiffonade
824) piri-piri
825) crispbread
826) fricasseed (merged into fricassee)
827) tsukemono (+plural)
828) feijoada
829) tourtière
830) macaronies (fixed - plural)
831) fourchette
832) quadratus
833) quadrati
834) submandibular
835) rectus
836) recti
837) umbilically (merged into umbilical)
838) preflighting (merged into preflight)
839) gyropilot
840) mainsheet
841) helmsmanship
842) unnavigability
843) coachroof
844) waypoint (+plural)
845) ASUS


V2.09 - 1-FEB-2014
-------------------
846) payday
847) MySpell
848) Alexandro's (added the 's)
849) neuropathy
850) neuropathic
851) stomatitis
852) malarious
853) remittent
854) haemodialysis
855) haemodialyses
856) inhalator
857) cataplexy
858) cataplectic
859) heartworm
860) encephalopathies (merged into encephalopathy)
861) bluetongue
862) endemism
863) exospheric (merged into exosphere)
864) polyphonist (merged into polyphony)
865) polyphonous
866) unsettlement (merged into unsettle)
867) spiderman (Oxford: informal)
868) spidermen (Oxford: informal)
869) airbrick
870) stringboard
871) icehouse
872) portcullised (merged into portcullis)
873) sexfoil
874) isometry
875) lunette
876) tympan
877) catchline
878) countermark
879) printability
880) sterilely (merged into sterile)
881) frogspawn
882) behaviouralism
883) behaviouralist
884) tartare
885) fishcake
886) pâtés (merged plural and removed the 's into the singular form)
887) dragonfish (+plural)
888) stonefish (+plural)
889) Quechua (+plural)
890) Quechuan
891) Bolognese
892) bolognese (lower case)
893) bolognaise
894) heteronym (+ic)
895) heteronymous
896) ungeared
897) ungenial (merged into genial)
898) ungenerousness
899) ungifted (merged into gifted)
900) unfussily (merged into unfussy)
901) Hunspell
902) ungainsayable
903) doubleton
904) causalgia
905) vesical
906) cryptogenic
907) ultrasonography
908) ultrasonographic (merged into ultrasonography)
909) tonometer
910) moonset
911) intercalary
912) Ramadan
913) overstimulation
914) psychomotor
915) psychosomatically (merged into psychosomatic)
916) hyperactively (merged into hyperactive)
917) hyperkinetic
918) hyperkinesis
919) captcha
920) APIs (plural)
921) iTunes
922) iPhones (plural)
923) iPads (plural)
924) iOS
925) hydroculture
926) perlite
927) hydrography
928) hydrographer
929) hydrographic
930) hydrographical
931) hydrographically
932) underframe
933) underminer (merged into undermine)
934) megacity (+plural)
935) mudbank
936) geospatial
937) postiche
938) sideburn (fixed: the speller only had plural)
939) upswept
940) reuptake (merged into uptake)
941) DEFCON (and defcon - Oxford: US armed forces)
942) shipbroker
943) furcula
944) furcular
945) furculae
946) springtail (+plural)
947) mealworm
948) viviparity
949) viviparously
950) plantable (merged into plant)
951) plantlet (+plural)
952) floriculture
953) floricultural (merged into floriculture)
954) floriculturist (merged into floriculture)
955) diaspore
956) diasporic (merged into diaspora)
957) tetanic (+ally)
958) Kalashnikov
959) alkane (+plural)
960) heterocyclic
961) benzenoid
962) semimetal
963) semimetallic (merged into semimetal)
964) betaine
965) bicyclic
966) astrochemistry
967) astrochemical
968) astrochemist
969) magnetopause
970) nanoscopic
971) nanostructure
972) nanotech
973) nanotube
974) nanorobot
975) EEPROM
976) chipmaker
977) bitstream
978) Bitstream (Oxford: trademark in the US)
979) superheterodyne
980) radiogoniometer
981) overscan
982) multitrack (+ed)
983) nanobot
984) bioelectronics
985) intercellular
986) heteromerous
987) neurosciences (plural)
988) biostatistical
989) biostatistician
990) pornification
991) pornify
992) laserdisc
993) genlock
994) S-VHS
995) chrominance
996) videocast
997) defrag (+s +ing +ed)
998) ultraportable
999) defragment (+ion +er)
1000) PDA
1001) webspace
1002) cybercrook (+plural)
1003) bootable
1004) preload (+ing)
1005) RTF
1005) GPRS
1006) SOSs (plural)
1007) chatline
1008) welcomely (merged into welcome)
1009) welcomeness (merged into welcome)
1010) welcomer (merged into welcome)
1011) welcomingly (merged into welcome)
1012) terroristic (merged into terrorist)
1013) terroristically
1014) instrumentalism
1015) superbug
1016) hantavirus
1017) herpesviruses (plural)
1018) herpetic
1019) leucoma
1020) headmasterly (merged into headmaster)
1021) schoolwork
1022) playschool
1023) schooldays
1024) encyclopedism
1025) encyclopedist
1026) encyclopedic
1027) exocentric
1028) endocentric
1029) psycholinguist (added singular)
1030) grimoire
1031) psychotronics
1032) Raelian (+plural)
1033) attunement
1034) attritional (merged into attrition)
1035) attornment
1036) subsoiling (merged into subsoil)
1037) fishmeal
1038) charro (+plural)
1039) horsebean
1040) pigman
1041) pigmen
1042) slaughterman
1043) agriproduct
1044) arboriculture
1045) arboricultural (merged into arboriculture)
1046) arboriculturist (merged into arboriculture)
1047) vermiculture
1048) nitrochalk
1049) unreactive (merged into reactive)
1050) bronzy 
1051) bronzer (merged into bronze)
1052) reliefs (plural)
1053) superalloy
1054) metallographic (merged into metallography)
1055) metallographically
1056) silversmithing (merged into silversmith)
1057) tinplate (+ed)
1058) EURIBOR
1059) Euromarket
1060) Eurocurrency (+plural)
1061) sexer (merged into sex)
1062) aristo (+plural - Oxford: informal term for aristocrat)
1063) toymakers (plural - merged into toymaker)
1064) toymaker's (merged into toymaker)
1065) rainswept
1066) acidophil (+plural +ic)
1067) basophil (+plural + merged ic into it)
1068) monocyte (+plural)
1069) glutamine
1070) chipset (+plural)
1071) HD
1072) ATA
1073) SATA
1074) AmigaOS
1075) Amigas (plural)
1076) archboard
1077) flatboat (+plural)
1078) keelboat (+plural)
1079) webinar (+plural)
1080) duodecimo (+plural)
1081) barmy (+ier +iest + ily +iness)
1082) barmbrack
1083) cushiony (merged into cushion)
1084) whoopers (plural)
1085) fireback (+plural)
1086) airgun (+plural)
1087) airheaded (Oxford: informal - merged into airhead)
1088) warplane (+plural)
1089) warpaint
1090) Cheyennes (plural)
1091) checklist (+plural)
1092) cafetière
1093) caffeinated
1094) cuppa (+plural - Oxford: informal)
1095) atriums (plural)
1096) basilican
1097) cartonnage
1098) cartophily
1099) cartophilist (+plural)
1100) Johan (name of person)
1101) virtuosic
1102) tessitura
1103) countertenor (+plural)
1104) antiretroviral (+plural)
1105) Prussia
1106) Prussian (+plural)
1107) countryfied (Oxford: also countrified)
1108) antiterror
1109) antiterrorist
1110) antitoxic
1111) micro-organism (Fix/deleted: it is microorganism)
1112) microorganism (+plural)
1113) micropayment (+plural)
1114) micronutrient (+plural)
1115) nanowire (+plural)
1116) nanobacterium
1117) nanobacteria
1118) nanobots (plural)
1119) nanoparticle (+plural)
1120) nanorobots (plural)
1121) nanotubes (plural)
1122) wordlessness (merged into wordless)
1123) wordlist
1124) contrafactive
1125) contrafactual
1126) contralateral
1127) contraposition
1128) multiplatform
1129) CSS
1130) yup
1131) yep
1132) XHTML
1133) SVGA
1134) namespaces (plural)
1135) sumptuary
1136) adminicle
1137) adminicular
1138) procurers (plural)
1139) paraplegic's (added the 's)
1140) victimhood
1141) camgirl
1142) unawakened (merged into awakened)
1143) gainly (Oxford: archaic)
1144) Ofcom (Oxford: abbreviation in the UK - Office of Communications)
1145) Facebook's (added the 's)
1146) giveaway (+plural)
1147) cheerleading
1148) telltale (+plural)
1149) zoospore (+plural)
1150) swarmer (+plural)
1151) hatchling (+plural)
1152) intercostal (+ly)
1153) radiolarian (+plural)
1154) ammonoid (+plural)
1155) allometry (+ic)
1156) dorsoventral (+ly)
1157) embryologic (merged into embryology)
1158) embryologically
1159) Pspell
1160) Aspell
1161) throwdown
1162) bodybuilder (+plural)
1163) bodybuilding
1164) bodyboard (+plural)
1165) bodyboarder (+plural)
1166) bodyboarding
1167) knowledgeability (merged into knowledge)
1168) knowledgable
1169) trichotomy (+plural)
1170) trichotomous
1171) tricker (merged into trick)
1172) trickish
1173) pixilated
1174) pixillated (other way of writing the above)
1175) glitchy (merged into glitch)
1176) cashback
1177) casuarina (+plural)
1178) MacOS
1179) overviewed (merged into overview)
1180) overviewing (merged into overview)
1181) overvaluation
1182) overwater (+ed +ing)
1183) overwear
1184) overwhelmingness
1185) overwind (+ing)
1186) overwound
1187) Bitrex (Oxford: trademark)
1188) bitonal
1189) bitonality
1190) hoo-ha (Oxford: British informal)
1191) synergist (+plural + ally)
1192) synfuel (+plural)
1193) aryl
1194) chromoly
1195) chromogenic
1196) haemophilic (+plural)
1197) antiscorbutic (+plural)
1198) antisense
1199) antirrhinum (+plural)


V2.10 - 1-MAR-2014
------------------
1200) POS
1201) Wikipedia's (added the ')
1202) PDAs (plural)
1203) FireWire
1204) mic (+plural - Oxford: short for microphone)
1205) deniability
1206) deniably (merged into deniable)
1207) remanent
1208) remanence
1209) coercivity
1210) CD-RW (+plural)
1211) onboard
1212) NAND
1213) modding+modded+modder (merged into mod)
1214) defragmenting (merged into defragment)
1215) DVD-ROM (+plural)
1216) CD-ROMs (plural)
1217) offline
1218) jeep (+plural)
1219) minidisc (+plural)
1220) predate (+ed)
1221) Boole (name + 's)
1222) phenotypic (merged into phenotype)
1223) phenotypical 
1224) phenotypically (merged into phenotypical)
1225) triploid (+plural)
1226) tetraploid (+plural)
1227) tetraploidy
1228) hexaploid
1229) hexaploidy
1230) sparrowhawk (+plural)
1231) whinchat (+plural)
1232) transgender (+ed +ing)
1233) homophobe (added singular, by fixing plural)
1234) intersexual
1235) intersexuality
1236) show's (added 's)
1237) battleaxes (plural)
1238) battlebus (+plural - Oxford: British informal)
1239) battlecruiser (+plural +'s)
1240) echinus
1241) echinoid (+plural)
1242) scriptoriums (plural of scriptorium)
1243) scriptoria (also plural of scriptorium)
1244) iMac
1245) cameraperson
1246) camerawork
1247) noir (+plural)
1248) noirish
1249) screwball (+plural +er)
1250) venesection
1251) gamelan
1252) gamepad (+plural)
1253) gamekeeping
1254) gamesman
1255) gamefowl
1256) catchlight (+plural)
1257) x86
1258) x64
1259) UTF-8
1260) PureBasic
1261) Algarve (famous city in Portugal)
1262) calculuses (added plural)
1263) Calcuttan
1264) caldarium
1265) caldaria (plural of caldarium)
1266) man-at-arms
1267) men-at-arms (plural)
1268) horsebox (+plural)
1269) horsebeans (plural)
1270) cornetto
1271) cornetti (plural of cornetto)
1272) cornett (+plural)
1273) cornetfish (+plural)
1274) cornerwise
1275) cornerman
1276) cornermen (plural of cornerman)
1277) Missourian
1278) eww (informal: exclamation)
1279) untypical (+ally)
1280) counter-jumper
1281) counter-offer (+plural)
1282) plumed (merged into plume)
1283) plume-like (fixed plumelike)
1284) plumbous
1285) blackleaded (merged into blacklead)
1286) duotone (+plural)
1287) duopolies (plural of duopoly)
1288) duology (+plural)
1289) underwork (+ed)
1290) alley-oop
1291) underpainting
1292) underperform (+s +ing)
1293) underperformance
1294) underplant (+s +ed)
1295) arbovirus (+plural)
1296) antibiosis
1297) bioenergy
1298) bioengineer (+s +ed)
1299) bioethical
1300) bioethicist (+plural)
1301) xenograft (+s)
1302) allograft (+s)
1303) homograft (+s)
1304) chemotaxis
1305) chemotactic
1306) secretor (+s)
1307) secretory
1308) meningioma (+plural)
1309) meningiomata (another plural of meningioma)
1310) meningitic
1311) flavivirus (+plural)
1312) seropositive
1313) seropositivity
1314) seronegative
1315) seronegativity
1316) seroprevalence
1317) SSD (+plural +'s)
1318) IOPS
1319) mSATA
1320) Z80
1321) tablier
1322) handcraft (+ed)
1323) candleholder (+plural)
1324) beadwork
1325) Polokwane (town in northern South Africa)
1326) Polonnaruwa (town in NE Sri Lanka)
1327) reptoid (+plural)
1328) poltroonery
1329) counteragent (+plural)
1330) counterblast
1331) counterbore (+plural)
1332) countercharge (+plural)
1333) countercheck (+plural)
1334) south-western
1335) wingbeat (+plural)
1336) sailplane (+plural)
1337) reimagine (+ed)
1338) groundbreaking
1339) groundbreaker (+plural)
1340) Shaolin
1341) teammate (+plural - fixed: team-mate)
1342) ninjutsu
1343) aikido
1344) McAfee
1345) cyber
1346) apparat (Oxford: chiefly historical)
1347) Mandela's (added the 's)
1348) Obama (name of person + 's)
1349) Snowden (name of person + 's)
1350) MasterCard
1351) Moodle's (added the 's)
1352) workflow (+plural)
1353) 720p
1354) 1080i
1355) 1080p
1356) artemisia (+plural)
1357) santolina (+plural)
1358) bloodroots (added plural)
1359) Emily (name + 's)
1360) H.264
1361) Corel
1362) brushstroke (+plural)
1363) 1024x768
1364) 1366x768
1365) 800x600
1366) 640x480
1367) 320x240
1368) 1280x720
1369) 1920x1080
1370) 1280x720p
1371) 1920x1080i
1372) 1920x1080p
1373) nasogastric
1374) nasopharynx
1375) nasopharyngeal
1376) mucosae (plural of mucosa)
1377) mucosal
1378) stroma
1379) stromata (plural of stroma)
1380) stromatoporoid (+plural)
1381) edaphosaurus (+plural)
1382) opacify (+s +ed +ing +er)
1383) oesophaguses (another plural of oesophagus)
1384) signifieds
1385) signifiers
1386) signifié (another term for signified)
1387) Harris (added 's)
1388) signifiant
1389) middleware
1390) choirgirl (+plural +'s)
1391) choisya (+plural)
1392) cowbane
1393) Nathel (name)
1394) eyehole (+plural)
1395) eyeline
1396) eyeliner (+plural)
1397) eyepatch
1398) fisheye
1399) fishbowl
1400) railroad (+plural +ing +ed)
1401) railman
1402) railmen (plural of railman)
1403) Horwich (name)
1404) railwayana
1405) raiments (plural)
1406) couchette (+plural)
1407) couchant
1408) addorsed
1409) coudé
1410) testee (+plural)
1411) fundus
1412) fundi (plural of fundus)
1413) funebrial
1414) micropipette (+plural)
1415) micropig (+plural)
1416) microphotograph (+s +ic +y)
1417) microphagous
1418) microphagic
1419) phytochemical (+plural)
1420) phytochrome
1421) phytogenetic
1422) phytogeography
1423) phytolith (+plural)
1424) lumens (plural)
1425) biogenic
1426) biogenesis
1427) biogenetic
1428) biogas
1429) high-pass
1430) sthenic
1431) veratrum (+plural)
1432) veratrine
1433) chimichanga (+plural)
1434) chimichurri
1435) burrito (+plural)
1436) drop-down
1437) pull-in
1438) cafe (+plural)
1439) brassard (+plural)
1440) brass-necked
1441) glissandos (plural, there is also glissandi)
1442) glissé (+plural)
1443) glitterati (Oxford: informal)
1444) Bollywood
1445) bollworm (+plural)
1446) sprayings (plural)
1447) horntail (+plural)
1448) Hornung (name of person)
1449) cracksman
1450) cracksmen
1451) cracknel (+plural)
1452) bucatini
1453) clementine (+plural - both lowercase - the fruit)
1454) cheesemonger (+plural)
1455) cheesewood
1456) cheffing + cheffed (merged into chef)


V2.11 - 1-APR-2014
------------------
1457) curriculum vitae
1458) curricula vitae (plural of curriculum vitae)
1459) multiparty
1460) polyphase
1461) polyphasic
1462) multiplatinum
1463) multiplay
1464) fabulate (+ion)
1465) fabulator
1466) fabulist (+plural +')
1467) autochange (+er)
1468) airbag (+plural)
1469) airband
1470) airbed
1471) airboat
1472) airbricks (added plural)
1473) stairlift (+plural)
1474) stairhead
1475) rundown
1476) staithe (+plural)
1477) double-ender
1478) notes inégales
1479) touchscreen (+plural)
1480) touchpoint (+plural)
1481) touchpad
1482) touchless
1483) toucher (+plural)
1484) faceplates (added plural)
1485) facetiae
1486) epigraphic (merged into epigraphy)
1487) epigraphically (merged into epigraphy)
1488) epigraphist (merged into epigraphy)
1489) epigrammatically
1490) trachoma
1491) trachomatous
1492) acetogenic
1493) biofilm (+plural)
1494) bioenergetics
1495) bioelectric
1496) bioelectrical
1497) supramolecular
1498) phospholipid (+plural)
1499) proteinase (+plural)
1500) proteoglycan (+plural)
1501) sulfate (+plural - another form of sulphate)
1502) alkene (+plural)
1503) alicyclic (+plural)
1504) cyclohexane
1505) involutes (+plural)
1506) parapodium
1507) parapodia (plural of parapodium) (+parapodial)
1508) polychaete (+plural)
1509) bryozoan (+plural)
1510) pustulate (+ing +ion)
1511) pussyfooter (+plural)
1512) pussycat (+plural)
1513) pussy-whip (+ed - Oxford: vulgar slang)
1514) multifold
1515) New Ager (+plural)
1516) New Ageism
1517) New Age
1518) groundbait (+ing)
1519) ground-to-air
1520) crosstrees
1521) flexitarian (+plural)
1522) nightlife
1523) ragweed
1524) bottlebrush (+plural)
1525) multi-agency
1526) multiaxial
1527) biohazard (+plural)
1528) bioindicator (+plural)
1529) seabird (+plural)
1530) murrelet (+plural)
1531) steelhead
1532) eutectoid
1533) user-friendly (added +ier +est +ness)
1534) user-hostile
1535) darknet
1536) backlight (+ed +ing)
1537) backlift
1538) backline
1539) backlink (+plural)
1540) cellmate
1541) Whitemoor (name)
1542) Sundberg (name)
1543) multiculturalist
1544) multiculturally
1545) multidirectional
1546) multifaith
1547) multifactorial
1548) electrocoagulation
1549) electrocautery
1550) aeromedical
1551) aeromagnetic
1552) aerolite (+plural)
1553) aerogramme
1554) aerogel (+plural)
1555) aeroelasticity
1556) aeroelastic
1557) under-read
1558) speedo (+plural - Oxford: short informal for speedometer)
1559) Speedos (Oxford: trademark Men’s brief, tight swimming trunks)
1560) speedballs (added plural)
1561) punchball
1562) Saughton (name)
1563) methamphetamine
1564) methanal
1565) flocculant (+plural)
1566) whistle-blowing
1567) telescreen
1568) Morrisons (name)
1569) long-stay
1570) multicore
1571) binational
1572) binate
1573) cross-compiler
1574) autoresponder
1575) autorotate
1576) autorotation (+plural)
1577) autosave (+ing +ed)
1578) autoroutes (added plural)
1579) autoshaping
1580) Comanches (added plural)
1581) Lakota (+plural)
1582) anti-ageing
1583) anti-American
1584) anti-British
1585) anti-capitalist
1586) anti-capitalism
1587) anti-Christian
1588) anti-constitutional
1589) anti-corruption
1590) anti-democratic
1591) anti-emetic
1592) anti-feminist
1593) anti-feminism
1594) anti-fraud
1595) anti-gay
1596) right-click
1597) cybernauts (added plural)
1598) cyberculture
1599) cyberespionage
1600) backchannel (+plural)
1601) backcast (+plural)
1602) left-click
1603) Hilberg (name +')
1604) unfeasible (+bly)
1605) unfeasibility
1606) unfavourite (merged into favourite)
1607) misconfigure (+ed)
1608) bitmapping + bitmapped  (merged into bitmap)
1609) CCD
1610) Williard (name)
1611) neonatologist
1612) neonatology
1613) neonicotinoid (+plural)
1614) pyrethroid (+plural)
1615) neopaganism
1616) neopagan (+plural)
1617) neophobia
1618) neophobic
1619) neontology
1620) neontological
1621) catabolism
1622) catabolic
1622) catabolize (+ing)
1623) biosynthesis
1624) biosynthetic
1625) metaphysician (+plural)
1626) metaplasia
1627) metaplastic
1628) by-blow
1629) Mendelssohn (added 's)
1630) goop
1630) goopy (+ness +er +est)
1631) goosander (+plural)
1632) waterbird (+plural)
1633) waterbed (+plural)
1634) footboard (+plural)
1635) alright
1636) footbrake (+plural)
1637) rpm
1638) footgear
1639) footle
1640) facade (also façade)
1641) footmark (added singular)
1642) tobyman + tobymen
1643) Witherington (name)
1644) Higton (name)
1645) ottocento
1646) mulattos (plural)
1647) vice anglais
1648) magnesian
1649) ultrabasic
1650) volcanicity
1651) geopark
1652) geomorphologist (merged into geomorphology)
1653) catastrophism
1654) catastrophist
1655) uniformitarianism
1656) gralloch
1657) superhelix
1658) superhelices
1659) superhelical
1660) superheavyweight
1661) superheavy
1662) supergroup
1663) supergravity
1664) superstring (+plural)
1665) supersymmetry (+ic)
1666) supertax
1667) supertwist
1668) LCD's (added the 's)
1669) multibillion
1670) multibuy (+plural)
1671) telepathist (merged into telepathy)
1672) telepathise (merged into telepathy)
1673) empath
1674) clairaudience
1675) clairaudient
1676) apozem (+plural - Oxford: archaic)
1677) electuary (+plural - Oxford: archaic)
1678) qubit (+plural)
1679) autodialler + autodials + autodialling + autodialled (merged into autodial)
1680) autocue (fix: It only accepted uppercase - in Oxford it is in lowercase)
1681) filmgoer (+plural)
1682) somatically
1683) microinject (+ed +ing)
1684) microinjection
1685) postdoctoral
1686) mutagenesis
1687) mutagenicity
1688) blastoderm
1689) syncytium
1690) syncytia
1691) syncytial
1692) plasmodium
1693) plasmodia
1694) plasmodial
1695) trophoblast (+plural +ic)
1696) trisomy (+plural)
1697) monosomy (+ic)
1698) Wacker (name)
1699) dystopias (plural)
1700) dystopian (+plural)
1701) dystopic
1702) Vorticist (+plural)
1703) Vorticism
1704) Bloomsbury
1705) Dadaism
1706) Dadaist (+ic)
1707) vortical
1708) vortically
1709) vorticose
1710) vorticular
1711) vorticella (+plural)
1712) araneid (+plural)
1713) wickiup
1714) wicketkeeper (+plural)
1715) wicketkeeping
1716) supersize (+s +ing +ed)
1717) supersensitive
1718) unaspirated
1719) forma (+plural)
1720) formae (another plural of forma)
1721) overleap (+s +ed)
1722) overleapt
1723) overlarge
1724) beget (+s +ing +er +est)
1725) foredune (+plural)
1726) embayment (+plural)
1727) longshore
1728) longship (+plural)
1729) longlist (+plural)
1730) shortlist (+s +ed)
1731) shortlistee (+plural)
1732) shorthold
1733) thug-like
1734) shorthair (+plural)
1735) short-tempered


V2.12 - 1-MAY-2014
------------------
1736) hurrahed + hurrahing (merged into hurrah)
1737) extrados
1738) intrados
1739) voussoir (+plural)
1740) Lillie (name)
1741) Udmurt
1742) Votyak (former term for the language Udmurt)
1743) Casio (+'s - company)
1744) W
1745) Windows XP
1746) status quo
1747) deadlight (+plural)
1748) deadeye (+plural)
1749) greenie (+plural)
1750) whinge (+s +ing +ed)
1751) whingeing (another form of the previous one)
1752) stonechat (+plural)
1753) wheatear (+plural)
1754) figbird (+plural)
1755) forktail (+plural)
1756) sunbird (+plural)
1757) downcurved
1758) leafbird (+plural)
1759) bulbul (+plural)
1760) drongo (+plural)
1761) drongoes (another plural of the previous one)
1762) bee-eater (+plural)
1763) bee-stung
1764) sunblind (+plural)
1765) sunbittern (+plural)
1766) antbird (+plural)
1767) hoatzin (+plural)
1768) manakin (+plural)
1769) cotinga (+plural)
1770) Neotropical
1771) neotropics
1772) fernbird (+plural)
1773) fernbrake
1774) ferox
1775) butterfish (+plural)
1776) butterflying + butterflied (merged into butterfly)
1777) superfamily (+plural)
1778) superfecundation
1779) superfetation
1780) ecdysis
1781) ecdysial
1782) clavate
1783) thalli
1784) thallus
1785) thallophytes (plural)
1786) cyanophyte (+plural)
1787) dysenteric (merged into dysentery)
1788) dysfunctionality
1789) dysgraphia
1790) dysgraphic
1791) transracial
1792) wildwood
1793) redshank (+plural)
1794) oystercatcher (+plural)
1795) turnstones (plural)
1796) transsexuality
1797) transputers (plural)
1798) chipmakers (plural)
1799) photonics
1800) photonegative
1801) photomultipliers (plural)
1802) photonic
1803) echograph
1804) echogram (+plural)
1805) echocardiograph (+y +ic)
1806) auscultatory
1807) cervicitis
1808) vaginosis
1809) preterm
1810) backmarker (+plural)
1811) backlot (+plural)
1812) backload (+plural)
1813) dropside (+plural)
1814) dropseed
1815) buttonbush (+plural)
1816) Lesley (name)
1817) hatband (+plural)
1818) hatbox (+plural)
1819) weighbridges (plural)
1820) backstroker (+plural)
1821) downhillers (plural)
1822) downhole
1823) doodah (Oxford: informal)
1824) doodly-squat (another term for diddly-squat)
1825) plenish (+ed)
1826) pleiotropy
1827) euroland
1828) eurozone
1829) Eurotrash
1830) Eurosceptic
1831) Eurosceptical
1832) Europoort
1833) Europhobe
1834) Europhile
1835) Europop
1836) Eurovision
1837) Athenaeum
1838) inunction
1839) linoleate
1840) oleate
1841) OLEDs (plural)
1842) peaberry (+plural)
1843) bibelot (+plural)
1844) grotesques (plural)
1845) elementals (plural)
1846) elementalism
1847) uncombined
1848) thaumaturge (+ic +ist +y)
1849) thaumaturgical
1850) mage (+plural)
1851) divinatory
1852) demonolatry
1853) candomblé
1854) Umbanda
1855) macumba
1856) maculate (+ed +ion)
1857) gouache (+plural)
1857) aquarelle (+plural)
1858) pegmatite (+plural)
1859) spessartine
1860) pyrope
1861) grossular
1862) almandine
1863) carnotite
1864) vanadate
1865) autunite
1866) uraninite
1867) vanadinite
1868) columbium
1869) Alentejo (name of region in Portugal)
1870) Galicia (name of region in Spain)
1871) Galician
1872) Callisto (name)
1873) callop (+plural)
1874) mulloway
1875) fingerlings (plural)
1876) parr
1877) smolt (+plural)
1878) amphipod (+plural)
1879) filmset (+ing +er)
1880) filmography (+plural)
1881) filmstrips (plural)
1882) filopodium
1883) filopodia (plural of filopodium)
1884) filopodial (merged into filopodia)
1885) pseudopodium
1886) pseudonymity
1887) pseudonymously
1888) pseudopods (plural)
1889) spermatidal
1890) biorhythmic (merged into biorhythm)
1891) biosafety
1892) biosecurity
1893) bioscientist (merged into bioscience)
1894) axoneme (+plural +axonemal)
1895) microtubule (+plural)
1896) microvascular
1897) cytoskeleton
1898) cytoskeletal
1899) abiotic
1900) abiogenesis
1901) abiogenic
1902) sinters (plural +ed +ing)
1903) flocculent
1904) flocculence
1905) haploidy (merged into haploid)
1906) autosome (+plural)
1907) autosomal
1908) Josh (name)
1909) cryonic (+s +ally)
1910)  cryopreserved (merged into  cryopreserve)
1911) autotransplant (+s +ion +ed)
1912) splenic
1913) bacilliform
1914) trinomial (+plural)
1915) superspecies
1916) downtrend
1917) downturned (merged into downturn)
1918) dot-com (+plural +er)
1919) dot-org
1920) dot-bomb (+plural)
1921) bloggers (plural)
1922) cyberspying
1923) cybersphere
1924) cyberslacker (+plural)
1925) cyberslacking
1926) bloatware
1927) gigapixel (+plural)
1928) bookmarklet
1929) clickjacking
1930) clickstream (+plural)
1931) client-server
1932) clientele
1933) upscale (+s +ed +ing)
1934) downrate (+ed)
1935) downmarket
1936) downlight (+s +er +ing)
1937) ludic
1938) schoolteaching
1939) Madison (added the 's)
1940) schoolmasterly (merged into schoolmaster)
1941) schoolmastering (merged into schoolmaster)
1942) bookselling
1943) bookplate (+plural)
1944) floorman
1945) floormen
1946) floorpan
1947) underbody (+plural)
1948) underboss (Oxford: informal)
1949) underbridge (+plural)
1950) overbridge (+plural)
1951) overbreed (+plural)
1952) overbred (past and past participle of the previous word)
1953) overboot (+plural)
1954) overboldly (merged into overbold)
1955) overboldness (merged into overbold)
1956) oversail (+plural +ing)
1957) euro (+plural)
1958) freewheel (+ed +ing)
1959) entresol
1960) pointe
1961) entropically
1962) entrustment (merged into entrust)
1963) entropion
1964) trichiasis
1965) luxate (+ed +ing +ion)
1966) dysplasia
1967) dysplastic
1968) adenoma (+plural)
1969) adenomata (another plural form of the previous word)
1970) herpetofauna (+al)
1971) bifid
1972) ampulla
1973) ampullae (plural of the previous word)
1974) autograft (+plural)
1975) conjunctival (merged into conjunctiva)
1976) autographic (merged into autograph)
1977) autography (merged into autograph)
1978) autoharp
1979) autohypnosis
1980) autointoxication
1981) flameout (+plural)
1982) flameproof
1983) flamethrower
1984) investigable
1985) investable (merged into invest)
1986) investible (merged into invest)
1987) canonicals (added plural)
1988) replaced salt-water with saltwater (fix)
1989) saltwort
1990) comfrey (+plural)
1991) canoness (+plural)
1992) archiepiscopacy (+plural)
1993) archiepiscopate
1994) Eurocommunism
1995) Eurocommunist
1996) Gramsci (+'s - name)
1997) eurocreep (Oxford: informal)
1998) Eurodollar (+plural)
1999) Eurocrat (+plural)
2000) Eurobond (+plural)
2001) Euro-MP (+plural)
2002) coalhouse
2003) coalfish (+plural)
2004) arenosol (+plural)
2005) arenavirus (+plural)
2006) adenovirus (+plural)
2007) adenoviral
2008) corynebacterium
2009) corynebacteria (plural of previous word)
2010) coryphée (+plural)
2011) ballboy (+plural)
2012) dogshore (+plural)
2013) autofocusing (merged into autofocus)
2014) autofill
2015) autoecology
2016) rallycross
2017) milord (+'s - word of the day in Oxford Dictionary)
2018) milometer
2019) mileometer (another word for the previous word)
2020) close-ratio
2021) jink (+ed +s +ing)
2022) ju-jutsu (another way of writing it)
2023) kung fu
2024) unvalidated
2025) unvaccinated
2026) aeromodelling
2027) aeromodeller
2028) angiogenesis
2029) dermatome (+plural)
2030) tragus
2031) tragi (plural of previous word)
2032) friarbird (+plural)
2033) raptor (+plural)
2034) gravlax
2035) gravadlax (another way of writing the previous word)
2036) caviar (also caviare)
2037) cavewoman
2038) cavewomen
2039) unfriend (+ed +ing)
2040) unfollow (+ed +ing)
2041) underway (also "under way")
2042) homeowner (+plural)
2043) teenybopper (+plural)
2044) teenybop
2045) teenaged (merged into teenage)
2046) teensy
2047) teensier + teensiest (merged into teensy)
2048) overpack (+s +ed)
2049) overnighted + overnights + overnighter (merged into overnight)
2050) overmuch
2051) overmighty
2052) overman (+s +ed +ing)
2053) overlordship
2054) overlooker (+plural)
2055) overlong
2056) overlock (+ed)
2057) overlocker (+plural)
2058) mudstone
2059) afferents (added plural)
2060) cholinergic
2061) fairwater
2062) dockage
2063) fah (Oxford: also fa)
2064) soybean (+plural)
2065) satanize + satanise (both possible - +s +ed)
2066) shitted (merged into shit)
2067) shat (Oxford: past participle shitted or shit or shat)
2068) autocephalous
2069) coachwood (+plural)
2070) coachwhip
2071) bird-like
2072) bipedality
2073) EuroMillions
2074) biphasic
2075) blackbuck (+plural)
2076) brushtail
2077) brushback (+plural)
2078) brush-turkey (+plural)
2079) Seychellois (place)
2080) Byelorussia (place - also Belorussia)
2081) bellyboard (+s +er +ing)
2082) bellyflop (+s +ed +ing)
2083) bellyband
2084) breasthook (+plural)
2085) breastfeeds (plural)
2086) breastpin (+plural)
2087) breaststroker (+plural)
2088) breastsummer
2089) biotecture
2090) bioterrorism
2091) bioterrorist
2092) biotherapy (+plural)
2093) agriproducts (plural)
2094) agriscience (+ist)
2095) agritourism
2096) destock (+ing)
2097) telecom (+plural - both in lower case)
2098) telecomputer (+plural)
2099) telecomputing
2100) debacle (+plural)
2101) deattribute (+ed)
2102) deattribution
2103) conservatoires (added plural)
2104) deathtrap
2105) death-or-glory
2106) death-dealing
2107) death-obsessed
2108) cupholder
2109) box-shifter (+plural)
2110) boxboard
2111) anti-tank
2112) bumboat (+plural)
2113) bumfluff
2114) Antichrists (added plural)
2115) animatronic (+plural)
2116) anime (Japanese cartoons)
2117) manga (Japanese cartoons)
2118) manfulness
2119) quadragenarian
2120) quadplex
2121) Eid (Muslim festival)
2122) Azania (name of place)
2123) Azanian
2124) azeotrope (+s +ic)
2125) fusel oil (+plural)
2126) monocoque
2127) polyclonal
2128) analyte (+plural)
2129) circumcircle (+plural)
2130) incircle
2131) incipit (+plural)
2132) universalist (+plural)
2133) Quaker (+plural)
2134) Quakerish
2135) Quakerism


V2.13 - 1-JUN-2014
------------------
2136) iPod (+plural)
2137) hoody (+plural)
2138) hoodie (+plural) (another form of the previous word)
2139) operant (+plural)
2140) operantly (merged into operant)
2141) pneumonectomy (+plural)
2142) pneumonic
2143) bronchopneumonia
2144) pneumonoultramicroscopicsilicovolcanoconiosis
2145) pruritus + pruritic
2146) bulla
2147) bullae (plural of previous word)
2148) sacs (added plural)
2149) sac-like
2150) aneurysmal (merged into aneurysm)
2151) subarachnoid
2152) subantarctic
2153) subagency (+plural)
2154) subagent (+plural)
2155) subadult (+plural)
2156) never-changing
2157) never-dying
2158) never-failing
2159) candlenut (+plural)
2160) Portlaoise (name)
2161) Portlaoighise (another form for the previous word)
2162) galangal
2163) lemongrass
2164) lemonwood
2165) candleberry (+plural)
2166) subacid
2167) greenbottle (+plural - Oxford: A metallic green fly)
2168) canola
2169) Bayer (name of company)
2170) UFO's (added the 's)
2171) Schratt (name)
2172) cyberbullying (merged into cyberbully)
2173) cyberbullies (plural - merged into cyberbully)
2174) cybercast (+s +ing)
2175) cyberchondriac (+plural)
2176) cyberchondria
2177) cyberphobics (added plural)
2178) constitutionalism
2179) statism
2180) statist (+plural)
2181) cardiological
2182) cardiographer + cardiography (merged into cardiograph)
2183) bootloader
2184) bootlick (+ing)
2185) disenchantingly
2186) disempower (+s +ing +ed +ment)
2187) disembarrass (+ed +ing +ment)
2188) diseconomy (+plural)
2189) initialling (merged into initial)
2190) retrievability
2191) laddish (+ness)
2192) mis-sell + mis-selling + mis-sold
2193) misallocate (+s +ed +ing +tion)
2194) misandry
2195) misanthropical
2196) misapprehensive
2197) imperceptive
2198) imperator + imperatorial
2199) bioarchaeology
2200) bioarchaeological
2201) bioarchaeologist
2202) bioactive
2203) bioactivity
2204) monolayer (added singular)
2205) multilinguals (added plural)
2206) scaleless (merged into scale)
2207) endothermal
2208) endotoxin (+plural)
2209) exotoxin (+plural)
2210) tiki (+plural)
2211) removed "informations" since it is a singular word only
2212) blagueur (rare)
2213) vestiary (+plural)
2214) cruciate
2215) interphase
2216) metaphase
2217) radiobiology
2218) radiobiological + radiobiologically
2219) radiobiologist (+plural)
2220) repairability
2221) multimodal
2222) multimeter (added plural)
2223) cryobiology + cryobiologist
2224) cryobiological
2225) andropause + andropausal
2226) terraform (+ing)
2227) terraformer (+plural)
2228) airscrew (+plural)
2229) triplanes (added plural)
2230) airside
2231) buff-tip
2232) dialectology + dialectological + dialectologist
2233) diatheses (plural of diathesis)
2234) sterol (+plural)
2235) sphingolipid (+plural)
2236) condonation (merged into condone)
2237) neo-Nazi (+plural)
2238) neo-Nazism
2239) neo-Marxist (+plural)
2240) neo-Marxism
2241) neo-liberal (+plural)
2242) neo-liberalism
2243) communalism
2244) communalist (+plural +ic)
2245) neocolonial
2246) neocolonialist (+plural)
2247) neoconservative (+plural)
2248) neoconservatism
2249) neocortices (plural of neocortex)
2250) neocortical
2251) neurologic (merged into neurology)
2252) neurolinguistic (+plural)
2253) analphabetic
2254) aspheric (+ally)
2255) aspherical
2256) fluxional
2257) fluxion (+plural)
2258) fluxgate
2259) demagnetizer
2260) demagnetiser
2261) chaology (+ist)
2262) semi-classical
2263) semi-conservative
2264) reinstalment
2265) Gerrit (software)
2266) occultation (merged into occult)
2267) occultist (+plural)
2268) occultness (merged into occult)
2269) contactless
2270) contact-breaker
2271) gnomon (+plural)
2272) (merged gnomonic into gnomon)
2273) Arminian (+plural)
2274) Arminianism
2275) popery
2276) Popemobile
2277) jokester
2278) jokesmith (+plural)
2279) biggies (added plural)
2280) Bigfoot
2281) Bigfeet (plural of previous word)
2282) bigeneric
2283) falcate
2284) emerald-cut
2285) dustheap
2286) dustcoat (+plural)
2287) dustcarts (added plural)
2288) barney (+plural)
2289) bunfight (+plural)
2290) backstabbings (added plural)
2291) Milroy (name)
2292) bungarotoxin
2293) neurotoxin (+plural)
2294) neurotoxic
2295) neurotoxicity
2296) neurotoxicology
2297) neurotrophic
2298) neurotypical
2299) user-definable
2300) user-generated
2301) unlikeable (merged into likeable)
2302) unliberated
2303) unlet (merged into let)
2304) unlearnedly
2305) unlay (+ing)
2306) unlaid (past and past participle of previous word)
2307) sensorimotor
2308) sensorineural
2309) sensorily (merged into sensory)
2310) subclavian
2311) semi-retired
2312) semi-opera (+plural)
2313) semi-modal
2314) semi-liquid (+plural)
2315) semi-invalid
2316) semi-lethal
2317) semi-fluid
2318) semi-finalist (+plural)
2319) semi-double
2320) semi-documentary (+plural)
2321) clock-watch (+ing +er)
2322) clockmakers (added plural)
2323) clockmaking
2324) chronographic (merged into chronograph)
2325) chronobiology
2326) chronobiologist (+plural)
2327) viewports (added plural)
2328) viewscreen
2329) hi-fi (+plural - fix: made it lowercase like in Oxford)
2330) hi-res (Oxford: informal)
2331) high-res (Oxford: informal - another form of the previous word)
2332) biopharma (+plural)
2333) unmunch (+s +ed +ing)
2334) focaccia
2335) Mailänder (name)
2336) backwards-compatible
2337) backward-compatible
2338) bottom-dwelling
2339) bottom-dweller (+plural)


V2.14 - 1-JUL-2014
------------------
2340) hard-code (+ed)
2341) autocorrecting (merged into autocorrect)
2342) autarchy (+ic +plural +al)
2343) autarky (same as the previous word - +plural +ic)
2344) marcasite
2345) microcrystalline
2346) microcredit
2347) irredeemability
2348) irrecusable
2349) bullseyes (added plural)
2350) bullshot
2351) inter-agency
2352) inter-allied
2353) inter-class
2354) interactant
2355) interactionism
2356) interactionist
2357) interarticular
2358) interatomic
2359) interbed (+ed)
2360) mudstones (added plural)
2361) shakuhachi (+plural)
2362) flat-top (+plural)
2363) flatbed
2364) formational (merged into formation)
2365) endpaper (added singular)
2366) electrotyper (added singular)
2367) electrotherapeutic
2368) electrotherapeutical
2369) electrosurgery
2370) electrosurgical
2371) electrotechnology
2372) electrotechnic (+s)
2373) electrotechnical
2374) electrothermal
2375) toolkit
2376) drop-dead
2377) englacial
2378) enginery
2379) gearhead (+plural)
2380) notchy (+ier +est)
2381) telemark (+ing)
2382) triclinic
2383) tricot
2384) catteries (added plural)
2385) diphtherial (merged into diphtheria)
2386) diphtheritic
2387) porphyria
2388) neuropathies (added plural)
2389) neuropath (+plural) (+ merged ic into it)
2390) acetylcholine
2391) Alzheimer (+ 's)
2392) alyssum (+plural)
2393) Solaris (OS)
2394) Arial (font)
2395) Verisign (+'s - company name)
2396) DNS
2397) Symantec (+'s - company name)
2398) courseware
2399) coursebook (+plural)
2400) blipvert (+plural)
2401) offshores + offshored (merged into offshore)
2402) eBay (+er)
2403) shovelboard
2404) shovelhead (shark)
2405) bonnethead (shark)
2406) blackfish (+plural)
2407) blackfly (+plural)
2408) astroturfing
2409) astrophotography
2410) astrophotographer (+plural)
2411) astrophotographic
2412) breakbeat (+plural)
2413) breakdance (+ing +er +ed)
2414) PCI
2415) HDDs (added plural)
2416) headquarter (added singular + ed)
2417) OEMs (added plural)
2418) Gb
2419) NAS
2420) drive's (added the 's)
2421) MMX
2422) cacheing (it can also be written caching)
2423) cacheable (merged into cache)
2424) cacheless (merged into cache)
2425) uncached
2426) rebrand (+ing)
2427) webcast (+plural +ing +er)
2428) videoconferences (added plural)
2429) videogram (+plural)
2430) headnotes (added plural)
2431) accretive
2432) amassment (+plural)
2433) bodger (+plural - Oxford: British informal)
2434) 'gainst (Oxford: short for against)
2435) unsex (+ed +s)
2436) epicene
2437) stepdad (+plural - Oxford: informal)
2438) stepfamily (+plural)
2439) stepmum (+plural - Oxford: informal)
2440) Stepford (name)
2441) robotically
2442) robotisation
2443) sh*tless (merged into sh*t - Oxford: vulgar slang)
2444) sh*thouse (Oxford: vulgar slang)
2445) sh*thead (Oxford: vulgar slang)
2446) sh*tface (Oxford: vulgar slang)
2447) shi*hole (Oxford: vulgar slang)
2448) trafficless (merged into traffic)
2449) downslope (+plural)
2450) eyeballed + eyeballing (merged into eyeball)
2451) brachycephalic
2452) brachycephaly
2453) registrant (+plural)
2454) registrarship
2455) rheumatology (+ist)
2456) rheumatological
2457) francophones (added plural)
2458) simple-mindedness (merged into simple-minded)
2459) uncompounded
2460) uncompanionable
2461) uncommunicated
2462) undeciphered
2463) undecided (+s +ly - merged into a new word)
2464) shilly-shally (+s +ing +ed +ier)
2465) shilly-shallyer (previous word can also have this form)
2466) precoital (+ally)
2467) precolonial
2468) preconcert (+ed)
2469) preconsciousness (merged into preconscious)
2470) precycling
2471) showbizzy (merged into showbiz)
2472) showband (+plural)
2473) bandmate
2474) bandleader (+plural)
2475) banditti (another plural of bandit)
2476) arse (+ed - Oxford: vulgar slang)
2477) interceders (added plural)
2478) interdental
2479) intercut (+s +ing)
2480) intercultural
2481) interculturalism
2482) intercross (+plural +ing)
2483) intercostals (added plural)
2484) intercool (+ed +er)
2485) intercontinentally
2486) intercommunity
2487) intercommunion
2488) anthropomorphize
2489) anthropomorphise (another form of the previous word)
2490) antinomians (added plural)
2491) antinomianism
2492) catechetics
2493) catechesis
2494) catchily + catchiness (merged into catchy)
2495) Chiu's (name - added the 's)
2496) examinee (added singular)
2497) backflip (+plural)
2498) backflow
2499) ATX
2500) reaccept (+ed)
2501) reacceptance
2502) reabsorption
2503) re-sort (+ing)
2504) compostable (merged into compost)
2505) composter (merged into compost)
2506) budstick (+plural)
2507) budwood
2508) budworm (+plural)
2509) earworm (+plural)
2510) silviculture (+al +ist)
2511) agro-industry (+plural +al)
2512) foodgrain (+plural)
2513) agronomical
2514) agronomically
2515) agrobiology
2516) agrobiological
2517) agrobiologist
2518) agroecosystem (+plural)
2519) algicide (+plural)
2520) aeroponic (+s)
2521) aeroponically
2522) aerophone (+plural)
2523) gyroplane (+plural)
2524) fixed-wing
2525) crowdsource (+ed +ing)
2526) crowdfunding
2527) crowdfunded
2528) creatives (added plural)
2529) Johnson's (added the 's)
2530) astrobiology
2531) astrobiological
2532) astrobiologist (+plural)
2533) archipelagoes (Oxford: another way of archipelagos)
2534) circumnavigator
2535) subalpine
2536) treeline
2537) boreal + Boreal 
2538) geotechnic (+s)
2539) geotechnical
2540) geothermally (merged into geothermal)
2541) cataclysmically
2542) Phoenician (+plural)
2543) Phoenicia
2544) bc ("Before Christ" in dates, can also be written in lower case)
2545) cross-post (+ing)
2546) cross-point
2547) rebreathe (+ed)
2548) rebreather (+plural)
2549) rebuildable (merged into rebuild)
2550) rebuilder (+plural)
2551) pumpkinseed (+plural)
2552) amberjack
2553) boxfish (+plural)
2554) trunkfish (+plural)
2555) brickyards (added plural)
2556) bricolage (+plural)
2557) genomics (added plural, suggested by the IRC nick on Mozilla wsmwk)
2558) proteomic (+plural)
2559) biomarker (+plural)
2560) microarray (+plural)
2561) proteome (+plural)
2562) bioinformatic (+plural)
2563) biographee (+plural)
2564) Lepidoptera + lepidoptera
2565) lepidopteran (+plural)
2566) lepidopterous
2567) decor (+plural)
2568) cliche (+plural)
2569) texted +  texting + texter + textless (merged into text)
2570) merged self-consciousness into self-conscious
2571) check-ins (added plural)
2572) pissy (Oxford: vulgar slang)
2573) low-cut
2574) firefighter (+plural)
2575) firefight (+plural)
2576) firefighting (merged into firefight)
2577) firefish (+plural)
2578) fireguard
2579) firelit
2580) firelock (+plural)
2581) blowouts (added plural)
2582) blowlamp
2583) blown-up
2584) blowing-up
2585) blowhole (+plural)
2586) blowfishes (added plural - Oxford: it can also be blowfish)
2587) blowback
2588) karateka (+plural)
2589) judoka (+plural)
2590) sensei (karate teacher)
2591) break-fall
2592) break-off
2593) karate-chop
2594) off-message
2595) off-patent
2596) off-pitch
2597) off-plan
2598) 4x4 (+plural - vehicles)
2599) protestor (+plural - Oxford: another way for protester)
2600) Proteus (a God + satellite)
2601) penistone
2602) telepaths (added plural)
2603) teleost (+plural)
2604) paddlefish (+plural)
2605) garfish (+plural)
2606) bowfin (+plural)
2607) infraclass (+plural)
2608) ruche (+plural +ed +ing)
2609) dolman (+plural)
2610) batwing
2611) storylines (added plural)
2612) hairband (+plural)
2613) batwoman
2614) batwomen
2615) megalopolitan (+plural)
2616) megalosaurus (+plural)
2617) iguanodon (+plural)
2618) carnosaur (+plural)
2619) infraorder (+plural)
2620) decapod (+plural)
2621) allée (+plural)
2622) averment (+plural)
2623) gratifier (merged into gratify)
2624) assuagement (merged into assuage)
2625) sangfroid
2626) unflashy (merged into flashy)
2627) ugly-looking
2628) ill-smelling
2629) ill-shaped
2630) ill-prepared
2631) ill-proportioned
2632) masturbator
2633) mastoiditis
2634) otitis
2635) otoscope
2636) otoscopic
2637) otoscopically
2638) uncultivable
2639) uncured
2640) uncurtained
2641) uncustomary
2642) undead
2643) undebatable
2644) undecidable
2645) undecidability
2646) facades (added plural)
2647) wodge (+plural)
2648) gloop (+y)
2649) herby (+ier +iest)
2650) bakeware
2651) swigger (+plural)
2652) citrusy
2653) citrussy (another way of the previous word - merged into citrus)
2654) swifty (+plural)
2655) swiftie (another way of previous word)
2656) enchantedly
2657) enchainment (merged into enchain)


V2.15 - 1-AUG-2014
------------------
2658) Cordova (name)
2659) upsize (+ing)
2660) downshift (+s +ing)
2661) righto (Oxford: informal)
2662) righty-ho (Oxford: another way of the previous word)
2663) burbot (+plural)
2664) non-black
2665) non-biodegradable
2666) non-biological
2667) non-capital
2668) non-chemical
2669) non-Christian
2670) non-Catholic
2671) portrayers (added plural)
2672) filmdom (+'s)
2673) portrayable (merged into portray)
2674) autochrome
2675) over-simplistic
2676) over-sentimental
2677) nonsensicality
2678) Nonya
2679) noob (+plural Oxford: informal)
2680) cancelbot (+plural)
2681) backloaded (merged into backload)
2682) backlist (+plural)
2683) gilet (+plural)
2684) blaxploitation
2685) bardolatry
2686) bardolater (+plural)
2687) bardolator (+plural Oxford: another way of the previous word)
2688) florilegium (+plural)
2689) florilegia (another plural of the previous word)
2690) floriferous
2691) aide-memoire
2692) aides-memoire + aides-memoires (both plural of previous word)
2693) breveted + breveting (merged into brevet)
          (Oxford says: brevets, breveting or brevetting, breveted or brevetted)
2694) compendiously + compendiousness (merged into compendious)
2695) addictively
2696) audiobook (+plural)
2697) audiogram (+plural)
2698) LP (+plural)
2699) boomy + boomier + boomiest + boominess
2700) boomers (added plural)
2701) bicolour (+ed)
2702) bicultural
2703) biculturalism
2704) areolate
2705) papule (+plural)
2706) papulae (another plural form of previous word)
2707) papular
2708) papulose
2709) papulous
2710) cholerically
2711) counterproductive
2712) counterproliferation
2713) counterpunch (+s +ed +ing)
2714) counterpuncher (+plural)
2715) counterrotate (+ing +ion)
2716) countershading
2717) countershaded
2718) counterstatement (+plural)
2719) countersubject
2720) counterterrorists (added plural)
2721) runaround
2722) juvenility
2723) juvenescence
2724) juvenescent
2725) ovotestis
2726) ovotestes (plural of previous word)
2727) ovoviviparous
2728) ovoviviparity
2729) coroneted (fix: coronetted and merged into coronet)
2730) bagwash
2731) bagworm (+plural)
2732) porkling
2733) cowhand
2734) Mb
2735) SecurDisc
2736) securable + securement + secureness (merged into secure)
2737) unthreatening
2738) unthreatened
2739) unthinkingness (merged into unthinking)
2740) unthinkability
2741) unthought
2742) unthread (+ed +ing)
2743) unthrifty + unthriftiness + unthriftily
2744) unthrone (+s +ed)
2745) Melbourne's (added the 's)
2746) DPhil (another way for PhD)
2747) nobble (+ing +ed)
2748) nobbler (+plural)
2749) bribable (merged into bribe)
2750) sawlog (+plural)
2751) peelers (added plural)
2752) bloodwort
2753) bloodwood (+plural)
2754) coastland (+plural)
2755) coasteering
2756) tuberculoid
2757) Hansen's (added the 's)
2758) lepromatous
2759) leprosarium (+plural)
2760) chemo (Oxford: informal)
2761) chemoattractant (+plural)
2762) granuloma (+plural)
2763) granulomata (another plural of previous word)
2764) granulomatous
2765) lurgy (+plural)
2766) bizarrerie (+plural)
2767) protogynous
2768) protogyny
2769) prolificacy
2770) geitonogamy
2771) geitonogamous
2772) xenogamy
2773) xenogamous
2774) autogamy
2775) autogamous
2776) agamospermy
2777) agamospermous
2778) unfertilized (ize)
2779) unfertilised (ise)
2780) enucleate (+ed +ion)
2781) protoplast (+plural)
2782) protoplanet
2783) protostar (+plural)
2784) circumlunar
2785) gibbosity
2786) Rushdie (name)
2787) Levinson (name)
2788) Patton (name)
2789) Naber (name)
2790) Moore’s (added the 's)
2791) narrational
2792) sidecar (+plural)
2793) indiscrete
2794) tonner
2795) doddle
2796) incidently
2797) imagesetter (+plural)
2798) imager (+plural)
2799) image-maker (+plural)
2800) dealmakers (added plural)
2801) dealign (+ed +ing +ment)
2802) dealfish (+plural)
2803) deal-breaker
2804) cardholder (added singular)
2805) B-movie (+plural)
2806) B-film (+plural)
2807) b'day (Oxford: informal - another short word for birthday)
2808) visuomotor
2809) visuospatial
2810) deistical
2811) carbon-14
2812) carbo-load (+ing)
2813) spiderweb (+s +ed +ing)
2814) Kasparov (name)
2815) Abdullah (name)
2816) childmind
2817) childbed
2818) childminder (added singular + 's)
2819) childminding
2820) paraphrasable
2821) paraphrastic
2822) deverbal (+plural)
2823) primordium
2824) primordia (plural of previous word)
2825) autoradiograph (+s +ed +ic +y)
2826) acentric
2827) dicentric
2828) biotech (Oxford: short for biotechnology)
2829) morphologic + morphologist (merged into morphology)
2830) morphogenetic
2831) morphogenic
2832) morpholine
2833) basicity
2834) chromous
2835) chromospheric (merged into chromosphere)
2836) plage (+plural)
2837) plagiaristic
2838) morphometry (+ic)
2839) morphometrically
2840) morphometrics
2841) neuroimaging
2842) neurohormone (+plural)
2843) vasopressin
2844) procoagulant (+plural)
2845) glycoprotein (+plural)
2846) ecdysone
2847) synchronic
2848) synchronically
2849) Darwin's (added the 's)
2850) evolutional (+ly)
2851) evolutive
2852) complementarily (merged into complementary)
2853) semiosis
2854) personhood
2855) acrolect (+plural +al)
2856) syntagmatic (+s)
2857) syntagmatically
2858) syntagm (+plural)
2859) syntagmas (another plural of syntagm)
2860) syntagmata (another plural of syntagm)
2861) cognitivist (+plural)
2862) cognitivism
2863) functionalist (+plural)
2864) electrophysiology (+ist)
2865) electrophysiological (+ly)
2866) electroplax
2867) electroplaque (another way of the previous word)
2868) electropolish (+ed +ing)
2869) nitinol
2870) stent (+plural)
2871) stenotypes (added plural)
2872) stenotypist (+plural)
2873) stenothermal
2874) Princeton (name)
2875) steepled (merged into steeple)
2876) steeplechasers (added plural)
2877) bookman
2878) bookmen (plural of previous word)
2879) booklouse
2880) booklice (plural of previous word)
2881) bookland
2882) bumf
2883) bumph (another way of previous word)
2884) bumboy (Oxford: vulgar slang)
2885) bumbledom
2886) bumbag (+plural - Oxford: informal)
2887) bum-sucking (Oxford: vulgar slang)
2888) bum-sucker (Oxford: vulgar slang)
2889) bum-bailiff (+plural)
2890) crookery
2891) cronyism
2892) croneyism (another way of previous word)
2893) depositaries (added plural)
2894) redeposition
2895) upwelling
2896) multistage
2897) multitalented
2898) dequeueing (another way of dequeuing)
2899) defriend
2900) unsecure
2901) unsectarian
2902) unseaworthiness
2903) utopians (added plural)
2904) decontrols (added +s)
2905) decontaminant
2906) deconstructionism
2907) deconstructionist (+plural)
2908) postmodern (fix: removed post-modern)
2909) postmodify (+s +ing +ed +tion +er)
2910) postorbital (+plural)
2911) postmodernism (fix: removed post-modernism)
2912) postmodernist (fix: removed post-modernist)
2913) postmodernity
2914) postdoctoral (fix: removed post-doctoral)
2915) postcolonial (fix: removed post-colonial)
2916) fix: removed post-office as actually it is "post office"
2917) dreck (+y)
2918) dreckish
2919) savvy (+er +est +s +ing +ed)
2920) salaryman
2921) salarymen (plural of previous word)
2922) untether (+s +ed +ing)
2923) reframe (+s +ed)
2924) GMOs (added plural)
2925) sext (+ing +er)
2926) selfie (+plural)
2927) selfy (another way of the previous word)
2928) tagger (+plural)
2929) tagetes
2930) fulminant
2931) haemoptysis
2932) telepathize (IZE - +s +ed +ing)
2933) cryptocrystalline
2934) cryptocurrency (+plural)
2935) cryptogam (+s)
2936) cryptogamic
2937) orthoepy (+ic)
2938) orthoepist
2939) orthogenesis
2940) orthogenesist
2941) orthogenetic
2942) orthogenetically
2943) myosin
2944) actin
2945) antiparallel
2946) palaeopathology
2947) palaeopathological
2948) palaeopathologist (+plural)
2949) playsuit
2950) playscheme (+plural)
2951) after-school
2952) aftercare (fix: removed after-care)
2953) afterdamp
2954) afterworld
2955) aftersun
2956) bergamot
2957) inswing (+ing +er)
2958) insurrectionary
2959) underage (fix: removed under-age)
2960) underactive
2961) put-you-up
2962) cragsman
2963) cragsmen (plural of previous word)
2964) wisecracker (+plural)
2965) campfire (fix: removed camp-fire)
2966) campsite (fix: removed camp-site)
2967) Caesar's (added the 's)
2968) bedload
2969) wordlists (added plural)
2970) bedjacket
2971) bedight
2972) bedhead (+plural)
2973) lustrate (+ed +ion)
2974) lustral
2975) contemporarily (merged into contemporary)
2976) Moldovan (+plural)
2977) Andorran (+plural)
2978) NW (North-west)
2979) Azerbaijani (+plural)
2980) acetaldehyde
2981) bailor
2982) bailment
2983) recognisance (added singular)
2984) distraint
2985) disquisitional (merged into disquisition)
2986) disputatiousness (merged into disputatious)
2987) preadaptation (+plural)
2988) preadapt (+ed)
2989) theropod (+plural)
2990) cowbird (+plural)
2991) preachiness
2992) pre-vocational
2993) pre-wash
2994) pre-shrunk
2995) pre-shrink
2996) pre-sell
2997) pre-Roman
2998) pre-release
2999) pre-scientific
3000) pre-service
3001) pre-tension (+s)
3002) punctum
3003) puncta (plural of previous word)
3004) biomechanics
3005) biomechanical
3006) biomechanically
3007) biomechanist
3008) phagocytic (merged into phagocyte)
3009) degranulate (+ion)
3010) neutrophil (+plural)
3011) prostaglandin (+plural)
3012) reticuloendothelial
3013) mononuclear
3014) histologic (merged into histology)
3015) monomorphic
3016) monomorphism
3017) monomorphous
3018) lysozyme
3019) ferritin
3020) ferrimagnetic
3021) ferrimagnetism
3022) rhizosphere
3023) translocate (+s +ing +ed +ion)
3024) transmarine
3025) transman
3026) transmen (plural of previous word)
3027) transmembrane
3028) transmigrant (+plural)
3029) Pilates
3030) Pilate (name)
3031) pilastered (merged into pilaster)
3032) dentil (+plural)
3033) pyrolysed (merged into pyrolyse)
3034) spellchecks + spellchecked + spellchecking (merged into spellcheck)
3035) unpowered
3036) unpractical
3037) unpracticality
3038) unpossessed
3039) unpolitical
3040) unpolitic
3041) unpointed
3042) unpoetic
3043) unpoetical
3044) unpoetically
3045) Prometheus (name)
3046) unplumbable
3047) unacademic
3048) Unabomber (name)
3049) unabating
3050) unscalable
3051) unscaleable (another way of previous word)
3052) unsay (+s +ing)
3053) unsatisfyingly
3054) unsatisfactoriness
3055) unsanctified
3056) unsalaried
3057) biosignature
3058) downwash
3059) airstream (+plural)
3060) landside
3061) airmobile
3062) airmanship
3063) divulgation (+plural)
3064) divulgence
3065) Diwali
3066) Divali (another way of previous word)
3067) Lakshmi (name)
3068) Lakshadweep (name)
3069) long-termism
3070) long-windedly
3071) longboard
3072) longdog
3073) goldendoodle (+plural)
3074) labradoodle (+plural)
3075) cockapoos (added plural)
3076) colourpoint
3077) Himalayan
3078) Birman
3079) Clermont-Ferrand (name)
3080) Auvergne (name)
3081) derivate (+plural)
3082) butyric (acid)
3083) Grammy + Grammies + Grammys (annual awards)
3084) heartbreaker (+plural)
3085) hyperacidity
3086) pyrosis
3087) calcific (merged into calcify)
3088) myxoma (+plural)
3089) myxomata (another plural of previous word)
3090) myxomatous
3091) bartholinitis
3092) asthmatically
3093) autohypnotic
3094) colectomy (+plural)
3095) chlamydial (merged into chlamydia)
3096) bronchospasm
3097) balanitis
3098) erythema (+al)
3099) erythematous
3100) chancroid (+plural)
3102) trichomoniasis
3103) candidiasis
3104) neutropenia
3105) neutropenic
3106) thrombocytopenia
3107) anticonvulsant (+plural)
3108) diazepam
3109) temazepam (fix: removed tamazepam)
3110) nitrazepam
3111) nitpicky (merged into nitpick)
3112) nitid
3113) Niterói (Oxford:  industrial port on Brazil)
3114) Folkestone (fix: removed Folkstone)
3115) barbican (+plural)
3116) Ragusa (name)
3117) ragworm
3118) lugworm (+plural)
3119) gapers (added plural)
3120) geoduck (+plural)
3121) geoengineering
3122) smily (another way of the word smiley) (+er +est)
3123) smileys (added plural - notice also smilies)
3124) gallberry (+plural)
3125) inkberry (+plural)
3126) inkhorn
3127) narratological
3128) narratologist (merged into narratology)
3129) narrativity
3130) eucatastrophe
3131) euchologion
3132) euchology (another way of the previous word)
3133) euchromatin
3134) euchromatic
3135) telomere (+plural +ic)
3136) telomerase
3137) cross-curricular
3138) cross-cultural
3139) cross-court
3140) forehanded (merged info forehand)
3141) forehock
3142) tapas
3143) tundish
3144) alloxan
3145) fenugreek
3146) diphtheroid
3147) blastomycosis
3148) granulocyte (+plural)
3149) ectomorph (+plural)
3150) ectoparasite (+plural)
3151) carboxylase
3152) cat-sit (+er +ers +ing)
3153) cat-bear
3154) -> 12) Ph.D. (removed one entry since it was there twice. Thanks, Filiep)
3155) groupset (+plural)
3156) beaconfish (+plural)
3157) characin (+plural)
3158) killifish (+plural)
3159) toothcarp
3160) topminnow (+plural)
3161) topman
3162) topmen (plural of previous word)
3163) toplessness
3164) Templar (+plural)
3165) withering
3166) withers (merged into wither)
3167) witherite
3168) analytics (merged into analytic)
3169) braincase
3170) sagittal (+ly)
3171) condyle (+plural)
3172) tibias (another plural of tibia)
3173) tibial (merged into tibia)
3174) medullary (merged into medulla)
3175) osteotomy (+plural)
3176) osteospermum (+plural)
3177) osteosarcoma (+plural)
3178) osteosarcomata (another plural way of previous word)
3179) osteoblast (+plural +ic)
3180) osteoarthritic
3181) LTR
3182) uncheck (+s +ing +ed)
3183) unselect (+s +ing +ed)
3184) unsentimentally (merged into unsentimental)
3185) unsensational (+ally)
3186) unseparated
3187) unserious
3188) unserviceability
3189) unsewn
3190) Macbeth (name)
3191) Ethelred's (added the 's)
3192) Gmail + gmail
3193) SETI
3194) doughboy (+plural)
3195) croquembouche
3196) choux
3197) loudmouth (+s - Fixed: loud-mouth)
3198) loud-mouthed (Oxford, it has a "-")
3199) loudhailer (+plural)
3200) biochips (added plural)
3201) biocircuits (added plural)
3202) high-security
3203) infirmarer
3204) infixed + infixing (merged into infix)
3205) septics (added plural)
3206) septically
3207) septicity
3208) embolus
3209) emboli (plural of previous word)
3210) cavitary
3211) cavernulous
3212) vermiculate (+ed)
3213) vermicular
3214) vermicide (+plural)
3215) vermicomposting
3216) vermicomposter
3217) esparto (+plural)
3218) espial
3119) leftish
3120) malcontents (added plural)
3121) Malbec
3122) Labrusca
3123) labradorite
3124) plagioclase
3125) aluminosilicate (+plural)
3126) muscovite (added lower case too)
3127) euhedral
3128) biotite
3129) titanite
3130) monazite
3131) xenotime
3132) chlorite (+ic)
3133) slaty (merged into slate)
3134) epidote
3135) chalcopyrite
3136) tetrahedrite
3137) electrum
3138) pyrargyrite
3139) pyrrhotite
3140) goethite
3141) kaolinite
3142) wurtzite
3143) ferberite
3144) tungstate
3145) tunesmith (+plural Oxford: informal)
3146) tunelessness (merged into tuneless)
3147) tuneage (Oxford: informal)
3148) Zhangjiakou (name of city in China)
3149) Kevin's (added the 's)
3150) non-compete
3151) non-communist
3152) barracouta (+plural)
3153) clownfish (+plural)
3154) damselfish (+plural)
3155) ghee
3156) Ghaznavid (+plural)
3157) Edwy (+'s - name)
3158) Antiochus (name)
3159) antinovel (+plural)
3160) disrelish
3161) louche (+ness)
3162) motorsailer
3163) motormouth (+ed)
3164) incentivize (+ed)
3165) incentivise (+ed - ize/ise)
3166) immunosuppressive (+plural)
3167) IMO (Oxford: informal: In My Opinion)
3168) blogosphere
3169) bloggy (+ier +est)
3170) blogroll
3171) liposome (+plural)
3172) liposculpture
3173) lipoprotein (+plural)
3174) lipopolysaccharide (+plural)
3175) aldosterone
3176) folate
3177) Cenozoic
3178) Cainozoic (another way of previous word)
3179) terebratulid (+plural)
3180) pedicle (+plural)
3181) pedicel (+plural)
3182) pedicellate
3183) intervertebral
3184) congruencies (added plural to congruency)
3185) quadrics (added plural)
3186) bilocation
3187) OBE (+plural)
3188) psoriatic
3189) whoever's (added the 's)
3190) multicellularity
3191) organismal (merged into organism)
3192) ODF
3193) copyleft (+ed)
3194) copyhold
3195) copyholder (+plural)
3196) copy-edit (+s +ed +ing)
3197) preamplifier (+plural - Fix: removed pre-amplifier)
3198) preamplify (+s +ing +ed)
3199) EQ
3200) epyllion
3201) epyllia (plural of previous word)
3202) unexcitable
3203) unexcitability
3204) unexaggerated
3205) uneventfulness (merged into uneventful)
3206) unevangelical
3207) equanimous
3208) Redditch (name of city)
3209) brutalism
3210) brutalist
3211) DDR
3212) backsight (+plural)
3213) kickflip (+plural)
3214) heelflip (+plural)
3215) frontside
3216) bodysurf (+er +ers +ing)
3217) bodyworker (+plural - merged into bodywork)
3218) thicknesses (added plural)
3219) thickheaded (fix: removed thick-headed) (added +ness)
3220) curation (merged into curate)
3221) cinephile (+plural)
3222) cinephilia
3223) cinematograph (+plural)
3224) cinematheque (+plural)
3225) Rohmer (name)
3226) summarization (-ize)
3227) summarisation (-ise)
3228) CVs (added plural)
3229) catechetical
3230) Christianity's (added the 's)
3231) catechetically
3232) anticlericalism
3233) warfarin
3234) taipan (+plural)
3235) workwear
3236) Adolf (+'s)
3237) MPLA
3238) UNITA
3239) Angolan (+plural +'s)
3240) Peter's (added upper case as name +'s)
3241) peterman
3242) petermen (plural of previous word)
3243) petersham
3244) grosgrain
3245) fieldfare (+plural)
3246) redwings (added plural)
3247) Zhu (name)
3248) Yuanzhang (name)
3249) minesweeping
3250) mineshafts (added plural)
3251) mineworker (added singular)
3252) Hohenzollern (name)
3253) hogweed (+plural)
3254) hognut
3255) Eminem (+'s - name)
3256) Elvira's (added 's - name)
3257) Elsa (+'s - name)
3258) Elisa (+'s - name)
3259) Elanor (+'s - name)
3260) Edison's (added 's - name)
3261) Egyptian's (added 's - name)
3262) Edwin's (added 's - name)
3263) Edwina's (added 's - name)
3264) Eleanor's (added 's - name)
3265) Eloise's (added 's - name)
3266) Eloy (+'s - name)
3267) Elias's (added 's - name)
3268) Emilia (+'s - name)
3269) Ernesto (+'s - name)
3270) Esmeralda (+'s - name)
3271) Evangelina (+'s - name)
3272) FBI's (added 's)
3273) Fernandez (+'s - name)
3274) Fernando's (added 's - name)
3275) Fiona (+'s - name)
3276) Firefox's (added 's)
3277) Flintstones (name)
3278) Florence's (added 's - name)
3279) Fitzgerald's (added 's - name)
3280) Fisher (+'s - name)
3281) Fischer's (added 's - name)
3282) Francesca's (added 's - name)
3283) Francis's (added 's - name)
3284) Francisca (+'s - name)
3285) Franks (+'s - name)
3286) Putin (+'s - name)
3287) Python (+'s - name)
3288) QWERTY (added upper case and removed lower case)
3289) AZERTY
3290) Rachael (+'s - name)
3291) Ramirez (+'s - name)
3292) Rambo (+'s - name)
3293) Coke (+plural + 's - the drink)
3294) Catholic + Catholic's (already had plural)
3295) Latinos (added plural - Oxford: Chiefly North American)
3296) KO + KO's + KO'ing + KO'd
3297) US (+'s - Country: United States in upper case)
3298) eclair (+s +'s)
3299) RTL


V2.16 - 1-SEP-2014
------------------
3300) Pinto (+'s - my name)
3301) add-on's (added the 's)
3302) Sagan (+'s - name)
3303) Steckling (name)
3304) Adamski's (added the 's - name)
3305) Ruppelt (+'s - name)
3306) Keyhoe (name)
3307) exoplanet (+plural)
3308) extrapyramidal
3309) antipsychotic (+plural)
3310) neuroleptic (+plural)
3311) benzodiazepine (+plural)
3312) benzocaine
3313) colposcope (+y)
3314) cytologically
3315) interspecific (+ally)
3316) bronchogenic
3317) amuck (other way of amock)
3318) amoebiasis
3319) superconscious (+ness +ly)
3320) unitive
3321) weakliness
3322) softie (added singular)
3323) softphone
3324) softshell (+plural)
3325) circumvallation (+plural)
3326) castellation (+plural)
3327) stats (Oxford: short for statistics)
3328) statoscope
3329) altimetry (+ic)
3330) innovational (merged into innovation)
3331) neoteric (+plural)
3332) Catullus (name)
3333) Vincenzo (name)
3334) Bellinis (added plural - Oxford: name + cocktail)
3335) catchfly (+plural)
3336) earthfall (+plural)
3337) foreshock (+plural)
3338) microseism (+plural +ic)
3339) microsite (+plural)
3340) biocompatible
3341) biocompatibility
3342) biocomputers (added plural)
3343) scopolamine
3344) hyoscyamine
3345) sexualize (+ion +ed -IZE)
3346) sexualise (+ion +ed -ISE)
3347) pansexual (+plural)
3348) pansexuality
3349) panspermia
3350) Cretans (added plural - Kevin Atkinson wordlist 60)
3351) Byzantines (added plural)
3352) Micmac (+plural - Kevin Atkinson wordlist 60)
3353) aberrational (merged into aberration -  Kevin Atkinson wordlist 60)
3354) anywise (Kevin Atkinson wordlist 60)
3355) armloads (added plural - Kevin Atkinson wordlist 60)
3356) backchat (Oxford: informal - Kevin Atkinson wordlist 60)
3357) zappy (+ier +iest Oxford: informal - Kevin Atkinson wordlist 60)
3358) Zarathustra
3359) Zarathustrian (+plural)
3360) ZAPU
3361) zeitgeist (+y - added lower case)
3362) whited (Kevin Atkinson wordlist 60)
3363) whensoever (Kevin Atkinson wordlist 60)
3364) whencesoever
3365) utilizable + utilizer (merged into utilize - Kevin Atkinson wordlist 60)
3366) 3365 + derivates with -ISE
3367) untasted (Kevin Atkinson wordlist 60)
3368) unrated (Kevin Atkinson wordlist 60)
3369) thingy (+plural - Kevin Atkinson wordlist 60)
3370) thingummy (+plural - Oxford: informal)
3371) taildragger (+plural)
3372) tailwheel
3373) unstick (+s +ing)
3374) unstimulating
3375) superglue (+s +ing +ed - Kevin Atkinson wordlist 60)
3376) superglueing (another way of previous word - ing)
3377) Lucifer's (added the 's)
3378) Cain's (added the 's)
3379) supergroups (added plural)
3380) supercoil (+s +ed +ing)
3381) osmolarity
3382) supergene (+plural)
3383) supergalaxy (+plural)
3384) supercluster (+plural)
3385) supercollider (+plural)
3386) superconduct (+ive)
3387) antiferromagnetic
3388) antiglobalization (-IZE)
3389) antiglobalisation (-ISE)
3390) counterparty (+plural)
3391) counterplot (+s +ing +ed)
3392) swansongs (added plural - Kevin Atkinson wordlist 60)
3393) syndicalists (added plural - Kevin Atkinson wordlist 60) 
3394) Trotskyism
3395) Trotskyist (+plural)
3396) Trotskyite (+plural)
3397) Stalinism
3398) Stalinists (added plural)
3399) recomposition
3400) 8-bit
3401) 16-bit
3402) 24-bit
3403) 32-bit
3404) 64-bit
3405) recommitment (merged into recommit)
3406) hypermutable
3407) hypermutation
3408) microsatellite (+plural)
3409) microreader (+plural)
3410) microprint (+ing)
3411) microform (+plural)
3412) microfungus
3413) microfungi (plural of previous word)
3414) microfossil (added singular)
3415) microflora
3416) microfauna
3417) microevolution
3418) microevolutionary
3419) macroevolution
3420) macroevolutionary
3421) microenterprise
3422) microelectromechanical
3423) microcyte (+plural +ic)
3424) Arabic (removed duplicate - PTG V3 build 64+)
3425) Atlantic (removed duplicate - PTG V3 build 64+)
3426) Lucy (removed duplicate - PTG V3 build 64+)
3427) ElGamal (cryptography)
3428) Elgamal (name of who created 3427)
3429) cryptosystem (+'s +plural)
3430) DES (+'s - cryptography)
3431) DSA (cryptography)
3432) TLS (cryptography)
3433) Diffie-Hellman (cryptography)
3434) VPNs (added plural)
3435) cryptanalysts (added plural)
3436) cryptanalytical
3437) AES (cryptography)
3438) Lagrange's (added 's)
3439) cryotherapy
3440) brachytherapy
3441) prostatectomy (+plural)
3442) photodynamic
3443) adjuvant (+plural)
3444) anorectal
3445) windproof
3446) dermabrasion
3447) sclerotherapy
3448) dermatoglyphics
3449) internegative (+plural)
3450) documentarian (+plural)
3451) documentarist (+plural)
3452) documentalist (+plural)
3453) toxophilite (+plural - Oxford: word of the day)
3454) pandanus
3455) pandan (another way of previous word)
3456) photomontage (+plural)
3457) photomosaic
3458) gluons (added plural)
3459) pions (added plural)
3460) antiquark (+plural)
3461) kaon (+plural)
3462) Easter (removed duplicate - PTG V3 build 64+)
3463) yogic
3464) asana (+plural)
3465) pranayama
3466) ashtanga
3467) astanga (another way of previous word)
3468) e-cigarette (+plural)
3469) cig (+plural - Oxford: informal for cigar)
3470) burley
3471) chromodynamics
3472) QCD (Oxford: short for quantum chromodynamics)
3473) backwind (+plural)
3474) forepart (+plural)
3475) voluted (merged to volute)
3476) volupté
3477) cristobalite
3478) suaver + suavest (merged into suave)
3479) suaveness (merged into suave)
3480) Rimbaud (+'s - name - Kevin Atkinson wordlist 50)
3481) Ringo (+'s - name - Kevin Atkinson wordlist 50)
3482) bluethroat (+plural)
3483) oxpecker (+plural)
3484) dunnock (+plural)
3485) credentialism
3486) camelback
3487) purslane
3488) glutathione
3489) cysteine
3490) glutamic
3491) asparagine
3492) histidine
3493) proline
3494) proliferous
3495) prolly (Oxford: informal of "probably")
3496) pleonastic
3497) pleonastically
3498) antistrophe (+plural)
3499) stanzaed (merged into stanza)
3500) stanzaic
3501) tailpiece (+plural - Kevin Atkinson wordlist 60)
3502) thumbprints (added plural - Kevin Atkinson wordlist 60)
3503) underclasses (added plural - Kevin Atkinson wordlist 60)
3504) Riefenstahl (+'s - name - Kevin Atkinson wordlist 50)
3505) Ragnarök
3506) Prozac (+'s - Kevin Atkinson wordlist 50)
3507) Karloff (+'s - name - Kevin Atkinson wordlist 50)
3508) Kasparov's (added +'s - name - Kevin Atkinson wordlist 50)
3509) KKK (+'s - Ku Klux Klan - Kevin Atkinson wordlist 50)
3510) Islamism (+'s - Kevin Atkinson wordlist 50)
3511) Islamist (Kevin Atkinson wordlist 50)
3512) Invar (+'s - Kevin Atkinson wordlist 50)
3513) Ingrid (+'s - name - Kevin Atkinson wordlist 50)
3514) highlandman
3515) highlandmen (plural of previous word)
3516) Aymara (+plural)
3517) Ayrshires (added plural - animal breed)
3518) Aboriginals + aboriginals (added the 's)
3519) ethnomusicologist (+plural)
3520) ethnomusicologic
3521) ethnomusicological
3522) ethnomusicology
3523) intranet (fix: + added plural - changed to lower case)
3524) Parkinson (+'s - name)
3525) workaholism
3526) timepass
3527) watchmaking
3528) saddlers (added plural)
3529) saddlery (+plural)
3530) decompilations (added plural)
3531) cross-assembler (+plural)
3532) antitype (+plural)
3533) demerge (+ed +er)
3534) Demerol
3535) sloshy (+ier +iest)
3536) captivatingly
3537) captchas (added plural)
3538) abandonware
3539) freeware (fix: changed to lower case)
3540) entrepreneurially (merged into entrepreneurial)
3541) entrepreneurialism
3542) gunsight (+plural)
3543) gunslinger (+plural - Oxford: informal)
3544) gunslinging (Oxford: informal)
3545) gunstock (+plural)
3546) cannoneer (+plural)
3547) cannonry
3548) anti-piracy
3549) anti-racists (added plural)
3550) blockheaded (merged into blockhead)
3551) blockish
3552) supervillain (+plural)
3553) sysop (+plural - Oxford: informal - A system operator)
3554) sysadmin (+plural - Oxford: informal - A system administrator)
3555) CRC
3556) choccy (+plural - Oxford: informal)
3557) G7
3558) G8
3559) G2B
3560) drippy (removed duplicate - PTG V3 build 64+)
3561) OSes (added plural)
3562) sexennial
3563) sexcentenary (+plural)
3564) sexagesimal (+plural +ly)
3565) unpicks (merged into unpick - Kevin Atkinson wordlist 60)
3566) viewership (+'s - Kevin Atkinson wordlist 60)
3567) vitriolically (Kevin Atkinson wordlist 60)
3568) walkies (merged into walkie - Kevin Atkinson wordlist 60)
3569) wanker (+plural - Oxford: vulgar slang - Kevin Atkinson wordlist 60)
3570) weepily + weepiness (merged into weepy)
3571) weepie (Kevin Atkinson wordlist 60)
3572) whatshername + whatsername (Kevin Atkinson wordlist 60)
3573) whatshisname + whatsisname (Kevin Atkinson wordlist 60)
3574) wheatmeal (Kevin Atkinson wordlist 60)
3575) wheatish
3576) wheatgrass
3577) wheatgerm
3578) borage
3579) echinacea
3580) ginkgos (another plural form of ginkgo)
3581) cocobolo (+plural)
3582) hardwoods (added plural)
3583) tablemate
3584) penne
3585) tortelli
3586) gnocchi
3587) woodworms (added plural - Kevin Atkinson wordlist 60)
3588) OpenSSL (cryptography)
3589) https
3590) subdomain (+s +'s)
3591) ChatZilla (+'s)
3592) Bugzilla (+'s)
3593) XULRunner (+'s)
3594) Instantbird
3595) rheme (+plural)
3596) gonna (Oxford: informal)
3597) subcritical
3598) subcostal
3599) amygdaloid
3600) ventromedial (+ly)
3601) trigeminal
3602) endothelium
3603) undersurface
3604) mesoderm (+al)
3605) mesogastrium
3606) mesogastria (plural of previous word)
3607) epigastrium
3608) epigastria (plural of previous word)
3609) hypogastrium
3610) hypogastria (plural of previous word)
3611) epifluorescence
3612) epifauna (+al)
3613) botanicals (added plural)
3614) funguses (Oxford: another plural of fungus)
3615) pelecypod (+plural)
3616) lamellibranch
3617) lamellibranchs (plural of previous word)
3618) wholefood (+plural - Kevin Atkinson wordlist 60)
3619) weekenders (added plural - Kevin Atkinson wordlist 60)
3620) wog (+plural - Oxford: offensive - Kevin Atkinson wordlist 60)
3621) woggle (+plural)
3622) neckwear
3623) necrobiosis
3624) necrobiotic
3625) Gregorio (+'s - name - Kevin Atkinson wordlist 50)
3626) Guantánamo (+'s - added accent - name - Kevin Atkinson wordlist 50)
3627) Hammarskjöld (+'s - added accent - name - Kevin Atkinson wordlist 50)
3628) Hayworth (+'s - name - Kevin Atkinson wordlist 50)
3629) Margolin (+'s - name)
3630) l10n
3631) i18n
3632) glocalization (-ize)
3633) glocalisation (-ise)
3634) localizable (-ize)
3635) localisable (-ise)
3636) logogram (+plural)
3637) foreseeably (merged into foreseeable)
3638) destructors (added plural)
3639) prepend + prepends + prepending + prepended
3640) tuple (+plural)
3641) ISO's + ISOs (added +'s +s)
3642) metrology (+ist)
3643) metrological
3644) somatosensory
3645) paraphilia (+plural)
3646) paraphiliac (+plural)
3647) asexuals (added plural)
3648) subgenre (+plural)
3649) standout (+plural)
3650) psychosexual (+ally)
3651) psychophysiologist (+plural - merged into psychophysiology)
3652) neuropsychology (+ist +ists)
3653) neuropsychological
3654) electrodermal
3655) CIA's (added the 's)
3656) cryptomeria (+plural)
3657) cryptonym (+plural)
3658) cryptorchid (+plural)
3659) cryptorchidism
3660) spermatic
3661) suprarenal
3662) midlife (fixed mid-life)
3663) Harleys (motorcycle)
3664) Harley-Davidson (+'s)
3665) flathead (+plural)
3666) teardrop (+plural +'s - fixed: tear-drop)
3667) knucklehead (+s +ed - Oxford: informal)
3668) motorhome (+plural)
3669) pushrod (+plural)
3670) Cadillac's (added the 's)
3671) drivetrain (+plural)
3672) sportster (+plural)
3673) crankpin (+plural)
3674) Porsche (name)
3675) unsourced
3676) woody (removed duplicate - PTG V3 build 64+)
3677) wonderful (removed duplicate - PTG V3 build 64+)
3678) Branson (+'s - name)
3679) Bramah (+'s - name)
3680) Keble (+'s - name)
3681) scholasticism
3682) scholarliness
3683) Confucians (added plural)
3684) quietism
3685) shamanic
3686) shamanist (+plural)
3687) shamanize (+ed - ize)
3688) shamanise (+ed - ise)
3689) archpriest (+plural)
3690) archosaur (+plural)
3691) basilosaurus
3692) deinstall (+s +ing +ed +er +ion +ions)
3693) deinstal (+s - another way of previous word)
3694) hot-swap (+ed +ing +able)
3695) blokeish (+ness)
3696) cochoa (+plural)
3697) barbet (+plural)
3698) tinkerbird (+plural)
3699) greenbul (+plural)
3700) hornbill (+plural)
3701) hornbook (+plural)
3702) deliration
3703) Delian (+plural)
3704) Delos (island)
3705) paralogy
3706) paralogous
3707) paralogism (+plural)
3708) speciate (+ed +ing +ion)
3709) centromere (+plural)
3710) centromeric
3711) centuple (+ed +ing)
3712) centrum (+plural)
3713) centra (another plural way of previous word)
3714) discoverability
3715) Chevy's (added the 's)
3716) thermite
3717) cutlassfish (+plural)
3718) Porsche (added 's + plural)
3719) cetology (+ist +ists)
3720) orca (+plural)
3721) Payne (+'s - name)
3722) Patagonia (name of place)
3723) Patagonian (+plural)
3724) non-humans (added plural)
3725) axonal (merged into axon)
3726) capsid (+plural)
3727) dendritic (+ally)
3728) Palaeolithic (fix: now starts with upper case)
3729) mutualism (+plural)
3730) mutualist (+plural +ic +ically)
3731) perioperative
3732) bruxism
3733) assistive
3734) wikis (added plural)
3735) odt
3736) docx
3737) pdf (added also in lower case)
3738) OpenDocument
3739) mouseover (+plural)
3740) goaty (merged into goat)
3741) goatish
3742) ballotine (+plural)
3743) galantine (+plural)
3744) terrines (added plural)
3745) saltimbocca
3746) zipx
3747) brill
3748) gratin
3749) ovenproof
3750) monkfish (+plural)
3751) anglerfish (+plural)
3752) Bokmål (language)
3753) mash-up
3754) ID + ID's + ID'ing + ID'd (Oxford: don't confuse with "id" in lower case)
3755) downloader (+plural)
3756) downmost
3757) supercell (+plural)
3758) turbocharge + turbocharging + turbocharged (merged into one entry)
3759) turboboost
3760) crashworthy (+ness)
3761) afterdeck
3762) foresheet (+plural)
3763) Paki (+plural - Oxford: offensive, used in GB)
3764) Lillee (+'s - name)
3765) Bradman (+'s - name)
3766) Dryopithecus
3767) cryptobiotic
3768) cryptobiosis
3769) saccharide (+plural)
3770) defat (+s +ed +ing)
3771) defeasance (+plural)
3772) defeasible
3773) defeatedly (merged into defeated)
3774) defeminize (+ed +ing - IZE)
3775) defeminise (+ed +ing - ISE)
3776) Apelles (+'s - name)
3777) mesmerize (+s +r +ing +ed - IZE)
3778) mesmerically (merged into mesmeric)
3779) unpersuadable
3780) unperson (+plural)
3781) unperfumed
3782) unpersuaded
3783) unphysical
3784) unphilosophic (+ally)
3785) unphilosophical
3786) unphysiological
3787) unphysiologic (+ally)
3788) unpicturesque
3789) unpigmented
3790) exfoliate (+s +ed +ing)
3791) exfoliant (+plural)
3792) Ardennes (name of place)
3793) unamplified
3794) unamiable
3795) unamended
3796) zing + zed (removed duplicate - PTG V3 build 64+)
3797) z's + zs (Z - can be either in upper or lower case)
3798) unambivalent (+ly)
3799) ready (removed duplicate - PTG V3 build 64+)
3800) unambitiousness
3801) unambiguity
3802) weaponed (merged into weapon)
3803) weaponless (merged into weapon -  Kevin Atkinson wordlist 50)
3804) whitefish + whitefishes + whitefish's (Kevin Atkinson wordlist 50)
3805) wheresoever (Kevin Atkinson wordlist 50)
3806) weeper (+s +'s - Kevin Atkinson wordlist 50)
3807) weenier + weeniest (merged into weeny)
3808) unskilful (+ly +ness)
3809) unready
3810) unreadably
3811) unreachableness
3812) unreachably
3813) unratified
3814) preseason (+plural)
3815) preselector
3816) preselections (added plural)
3817) unquotable (merged into unquote)
3818) unquiet (+ly +ness)
3819) unsuspected (+ly - Kevin Atkinson wordlist 50)
3820) jackrabbit jackrabbit's jackrabbits (Kevin Atkinson wordlist 50)
3821) amazon (added also lower case because of some animal names)
3822) Amazonas (name of state in Brazil)
3823) Trekkie (+'s +plural - Oxford: informal - another way of Trekker)
3824) trelliswork (fixed: trellis-work)
3825) Tharp (+'s - name - Kevin Atkinson wordlist 50)
3826) Sumerians (added plural)
3827) sudoku
3828) sudoriferous
3829) ecoterrorism
3830) ecoterrorist (+plural)
3831) ecosphere
3832) bioregionalism
3833) bioregionalist (+plural)
3834) biodynamic (+plural)
3835) bioenergetic (added plural)
3836) bioethanol
3837) biotransformation
3838) biotype (+plural)
3839) biowarfare
3840) bioweapon (+plural)
3841) firepower (fixed: fire-power)
3842) fireship (+plural)
3843) Cmdre (Oxford: abbreviation of Commodore)
3844) Cmdr (Oxford: abbreviation of Commander)
3845) Bdr (Oxford: abbreviation of Bombardier)
3846) aircraftman
3847) aircraftmen (plural of previous word)
3848) aircraftwoman
3849) aircraftwomen (plural of previous word)
3850) airdrop (+s +ing +ed)
3851) combfish (+plural)
3852) nannygai (+plural)
3853) redfish (+plural)
3854) rockfish (+plural)
3855) scorpionfish (+plural)
3856) calvary (added lower case too - Oxford: when used as noun)
3857) parasail (+ing)
3858) ballgirl (+plural)
3859) backcourt (+plural)
3860) foredawn
3861) multilateralism
3862) multilateralist (added singular)
3863) unheroic (+ally)
3864) unheretical
3865) unheedingly
3866) unheedful
3867) unhedged
3868) unhearing
3869) unhealthiness
3870) untaken
3871) untalkative
3872) talkboard (+plural)
3873) talkback
3874) talkathon (+plural - Oxford: informal)
3875) over-talkative (+ness)
3876) overabundant (fix: over-abundant)
3877) overabundance (fix: over-abundance)
3878) overabundantly
3879) overachievement (merged into overachieve)
3880) DirectX
3881) XP (therefore removed "Windows XP")
3882) AVCHD
3883) BD-R (+plural)
3884) BD-RE (+plural)
3885) M-DISC (+'s)
3886) LightScribe
3887) CD-R (+plural)
3888) pre-plan (+ed)
3889) BD-ROM (+plural)
3890) BD-XL
3891) BDXL
3892) H.265
3893) DivX
3894) divx (also in lower case)


V2.17 - 1-OCT-2014
------------------
3895) toolbar's (added the 's)
3896) advisors (added plural)
3897) adviser (+plural +'s)
3898) kookily (merged into kooky)
3899) yawp (+ed +er +ers)
3900) yay (Oxford: informal)
3901) Darth (+'s - name from Star Wars)
3902) glam (+s +ing +ed)
3903) nothosaur (+plural)
3904) semiaquatic
3905) plesiosaur (+plural)
3906) ichthyosaur (+plural)
3907) ichthyosaurus (another way of previous word)
3908) ichthyophagous
3909) ichthyophagy
3910) ichthyornis
3911) nerve-racking
3912) nerve-wracking (another way of previous word)
3913) nervure (+plural)
3914) fangirl (+plural - Oxford: informal)
3915) fanboy (+plural - Oxford: informal)
3916) Bielefeld (name of city in Germany)
3917) georgette (added lower case)
3918) buckskinned (merged into buckskin)
3919) parfleche (+plural)
3920) musquash (another way for muskrat)
3921) byssus (+plural)
3922) byssi (another plural of previous word)
3923) byssinosis
3924) epidemiologic
3925) atopy (+ic)
3926) sarcoidosis
3927) colorectal
3928) absorbance (+plural)
3929) quinoa
3930) SIM (+plural)
3931) Nohl (+'s - name)
3932) sandboxing + sandboxed (merged into sandbox)
3933) Karsten (+'s - name)
3934) GSM + gsm
3935) telco (+plural)
3936) echocardiogram (+plural)
3937) angiogram (+plural)
3938) mesentery (+plural +ic)
3939) innominate
3940) exteriorize (+ed +ion - IZE)
3941) exteriorise (+ed +ion - ISE)
3942) Peigan (+plural)
3943) Piegan (+plural - another way of previous word)
3944) Blackfeet (plural of Blackfoot)
3945) whitehead (+plural)
3946) whitefront (+plural)
3947) waterfowling
3948) waterfowler (+plural)
3949) watergate (also lower case for gate)
3950) waterhen (+plural)
3951) tardigrade (+plural)
3952) onychophoran (+plural)
3953) Taiping (Kevin Atkinson wordlist 50)
3954) Swammerdam (+'s - name - Kevin Atkinson wordlist 50)
3955) sunbed (added singular)
3956) sunbelt (fix: changed to lower case)
3957) sunbow (+plural)
3958) vitalism
3959) casuistical (+ally)
3960) homebuyer (+plural)
3961) homebuilder (+plural)
3962) homebuilding
3963) homeobox
3964) homoeobox (another way of previous word)
3965) homeopath (+plural - another way of homoeopath)
3966) prescribers (added plural)
3967) prescribable
3968) integrationist (+plural)
3969) allopathy
3970) allopathic
3971) allopathist (+plural)
3972) reiki
3973) reimportation
3974) reimposition
3975) idol-worshipping
3976) idocrase
3977) Jiangxi (name of place)
3978) Kiangsi (another way of previous word)
3979) Qing
3980) Ch'ing (another way of previous word)
3981) mafic
3982) retiré (Ballet)
3983) retiree (+plural)
3984) retirer (+plural)
3985) overtype (+ing)
3986) overweeningly (merged into overweening)
3987) rufous
3988) bettong (+plural)
3989) potoroo (+plural)
3990) bandicoot (+plural)
3991) roadkill (+plural)
3992) bilby (+plural)
3993) numbat (+plural)
3994) woylie (+plural)
3995) aardwolf
3996) aardwolves (plural of previous word)
3997) Århus (name of place - another word for Aarhus)
3998) Barranquilla (name of place)
3999) barramundi (+plural)
4000) fireguards (added plural)
4001) firebreak (+plural)
4002) firebrick (+plural)
4003) firebrat (+plural)
4004) firebomb (+ derivates - fixed: fire-bomb)
4005) fireblight
4006) fireballer
4007) fireballing
4008) blockwork
4009) blockship (+plural)
4010) blockboard
4011) Shandong (another way of Shantung)
4012) shandies (plural of shandy)
4013) aguardiente
4014) agroterrorism
4015) agroterrorist
4016) agrostology
4017) Hitchcock's (name - added the 's)
4018) botanize (+ing - IZE)
4019) botanise (+ing - ISE)
4020) wireline
4021) wireframes (added plural)
4022) chatbot (+plural)
4023) immersive
4024) barcode (+s +ed)
4025) MP (+plural)
4026) mp (lower case)
4027) barchan (+plural)
4028) barcarolle (another way of barcarole)
4029) barbule (+plural)
4030) Sumer (name of place)
4031) Sumbawa (name of place)
4032) Sudra (+plural - Kevin Atkinson wordlist 50)
4033) sudorific (+plural)
4034) depuration
4035) depurate (+ed)
4036) depurative (+plural)
4037) depurator (+plural)
4038) depthless (+ly)
4039) stomachic (+plural)
4040) febrifuge
4041) ophthalmia
4042) pyrexia
4043) sigil (+plural)
4044) sightworthy
4045) Tangiers (another way of Tangier - Kevin Atkinson wordlist 50)
4046) Tarantino (+'s - name - Kevin Atkinson wordlist 50)
4047) Terry (+'s - name - Kevin Atkinson wordlist 50)
4048) paraesthesia (+plural)
4049) paraesthesiae (another plural of previous word)
4050) paradrop (+s +ing +ed)
4051) atomism
4052) atomist (+plural)
4053) moderns (added plural)
4054) moderatorship (+plural)
4055) RSS
4056) drawdown (+plural)
4057) drawcard (+plural)
4058) drawbar (+plural)
4059) drawcord (+plural)
4060) zipless (Oxford: informal)
4061) wearability
4062) pre-order (+plural)
4063) pre-prepare (+ed)
4064) neurodegenerative
4065) neurocomputer (+plural)
4066) neuroeconomics
4067) neurofibril (+plural)
4068) neurofibrillary
4069) supranuclear
4070) supraorbital
4071) foramen
4072) foramina (plural of previous word)
4073) argyrophilic
4074) suprasegmental (+plural)
4075) phrenic
4076) supremacism
4077) miniscule (another way of minuscule)
4078) neo-fascist (+plural)
4079) neo-Gothic
4080) neo-Latin
4081) neo-impressionism
4082) neo-impressionist (+plural)
4083) neo-Georgian
4084) neo-Darwinian
4085) neo-Darwinism
4086) neo-Darwinist
4087) neo-Confucianism
4088) neo-Confucian
4089) blissed (merged into bliss)
4090) gazania (+plural)
4091) bluegum
4092) stringybark
4093) stringpiece
4094) mallee (+plural)
4095) tad (Oxford: informal)
4096) tacticity
4097) configurational (merged into configuration)
4098) associationism
4099) associationist (+plural)
4100) epidemiologic (merged into epidemiology)
4101) colosseum (+plural - fixed: lower case - another word for coliseum)
4102) clearwing
4103) peacemonger (+ing +ed)
4104) peacekeeper (added singular)
4105) InterCity (trademark)
4106) intercolonial
4107) intercolumniation
4108) intercolumnar
4109) astragal (+plural)
4110) astragalus
4111) astragali (plural of previous word)
4112) antherozoid (+plural)
4113) gametophyte
4114) gametophytic (merged into gametophyte)
4115) microspore (+plural)
4116) megaspore (+plural)
4117) megastores (added plural)
4118) megastars (added plural)
4119) megastructure (+plural)
4120) Wright's (added 's)
4121) wretcheder (merged into wretched)
4122) wretchedest
4123) arteriovenous
4124) parenchyma (+al)
4125) parenchymatous
4126) sawfish (+plural)
4127) sawlike
4128) crowberry (+plural)
4129) cordgrass
4130) benzoin
4131) unorganized
4132) benzoic (acid)
4133) stereochemistry
4134) stereochemical (+ally)
4135) epoxide (+plural)
4136) catechol
4137) norepinephrine
4138) fluorochrome (+plural)
4139) fluoropolymer
4140) tetrafluoroethylene
4141) waitressing (merged into waitress)
4142) busbar (+plural)
4143) busby (+plural)
4144) semibreve (+plural)
4145) semibold
4146) semidemisemiquaver
4147) hemidemisemiquaver (+plural - another way of previous word)
4148) hemicylindrical
4149) hemicycle (+plural)
4150) azulejo (+plural)
4151) archivolt (+plural)
4152) entablement
4153) architectonically (merged into architectonic)
4154) annulet (+plural)
4155) policymaker (+plural - Fixed: policy-maker)
4156) policymaking (fixed: policy-making)
4157) policier
4158) Melville's (added the 's)
4159) counterscarp (+plural)
4160) countershaft (+plural)
4161) artiodactyl (+plural)
4162) bioluminescence
4163) bioluminescent
4164) biomagnetism
4165) biomaterial
4166) biomathematics
4167) biome (+plural)
4168) transgene (+plural)
4169) introgression
4170) backcross (+s +ed +ing)
4171) cultivar (+plural)
4172) cultigen (+plural)
4173) bifacial
4174) biface (+plural)
4175) uniface
4176) tricycling (merged into tricycle)
4177) tricyclist (+plural)
4178) petroglyphs (added plural)
4179) ghostwrite (+derivates - fixed: ghost-write)
4180) ghostwritten (fixed: ghost-written)
4181) speechifies + speechifying + speechified + speechifier (merged into speechify)
4182) speechification
4183) speedwriting
4184) speedwriter (+plural)
4185) Magdalenian
4186) Aurignacian
4187) defloration
4188) deflocculate (+ed)
4189) deflocculation
4190) deflesh (+ed)
4191) arsehole (+plural +ed - Oxford: vulgar slang)
4192) arsenates (added plural)
4193) selenates (added plural)
4194) selenide (+plural)
4195) machinability
4196) weldability (merged into weld)
4197) weldable (merged into weld)
4198) weldmesh
4199) corkwood (+plural)
4200) Graeco-Roman
4201) Greco-Roman (another way of previous word)
4202) paperbark (+plural)
4203) paperboy (fixed: paper-boy)
4204) papergirl (fixed: paper-girl)
4205) paperknife
4206) paperknives (plural of previous word)
4207) papermaking
4208) papermaker
4209) paperchase
4210) paperboard
4211) oldspeak
4212) Oldowan
4213) Epidaurus (ancient Greek city)
4214) epicycloidal (merged into epicycloid)
4215) epicycloids (added plural)
4216) hypocycloid (+al)
4217) commissure (+plural)
4218) legalist (+plural)
4219) legalistically (merged into legalistic)
4220) Carcassonne
4221) Carchemish (city)
4222) chiliastic
4223) chiliarch (+plural - chiliarchs)
4224) bowsers (added plural)
4225) bowshot
4226) bowstrung (past and past participle of bowstring)
4227) fletchers (added plural)
4228) pickerelweed
4229) pickelhaube
4230) orebody (+plural)
4231) noise-maker (removed - American word)
4232) noisette (+plural)
4233) noisomeness (merged into noisome)
4234) Nok
4235) Osbert (name - Osbert Guy Stanhope Crawford)
4236) Stanhope (name - Osbert Guy Stanhope Crawford)
4237) Belgic
4238) encash (+ed +able +ment)
4239) consortiums (another plural of consortium)
4240) conspecific (+plural)
4241) conspecificity
4242) spatterware
4243) spatterdash (+plural)
4244) allseed
4245) zugzwang
4246) myxoedema
4247) filariasis
4248) geotag (+s +ing +ed)
4249) geotaxis
4250) geotactic
4251) geotropism
4252) geotropic
4253) megapode (+plural)
4254) ensiform
4255) geosyncline (+plural)
4256) bioclast (+s +ic)
4257) biocide (+s  +idal)
4258) antifouling
4259) alprazolam
4260) depredator (+plural)
4261) depredatory
4262) lo-fi
4263) low-fi (another way of previous word)
4264) bokeh
4265) lo-res
4266) low-res (another way of previous word)
4267) dojo (+plural)
4268) ippon (+plural)
4269) bushido (now in lower case according to Oxford)
4270) bushmeat
4271) clifftop (+plural)
4272) geoid
4273) eukaryotic
4274) biosocial
4275) biometeorology
4276) autostereoscopic
4277) taskbar
4278) gamification
4279) gamify (+plural +ing +ed)
4280) gamgee
4281) gametogenesis
4282) gametogenic
4283) gametogeny
4284) systematist (+plural)
4285) autorotated (merged into autorotate)
4286) cyphel
4287) fire-brick (removed: it is firebrick)
4288) coleopteran (+plural)
4289) coleopterous
4290) dermestid
4291) dermatophytosis
4292) dermatophytoses (plural of previous word)
4293) dermatosis
4294) dermatoses (plural of previous word)
4295) hypermetropia
4296) hypermnesia
4297) hyperon (+plural)
4298) parasitoid (+plural)
4299) biopharming
4300) biophilia
4301) biopolymer (+plural)
4302) geeked + geeking (merged into geek)
4303) geekdom
4304) geekish
4305) geeky (+ier +iest +ness)
4306) geekery (Oxford: informal)
4307) geekspeak (Oxford: informal)
4308) chappie (+plural - Oxford: informal)
4309) eyeshadow (fix: eye-shadow)
4310) eyeshot
4311) eyestalk
4312) eyestripe (+plural)
4313) unpatterned
4314) unpatrolled
4315) unpatriotically
4316) unpatched
4317) unpassable
4318) unpardonableness
4319) unpacker (+plural)
4320) unornamented
4321) unoriginality
4322) unoriginally
4323) unordinary
4324) unoiled
4325) unoffending
4326) unobscured
4327) unobjective
4328) unoaked
4329) unnoticeably
4330) unnamable (another way for unnameable)
4331) unnail (+ed)
4332) unmusicality
4333) unmusicalness (merged into unmusical)
4334) unmutilated
4335) unmurmuring (+ly)
4336) unmuffle (+ed)
4337) unmothered
4338) unmotherly
4339) unmoral
4340) unmorality
4341) unpeople (+s +ing)
4342) unpeg (+s +ing +ed)
4343) unpatented
4344) untouchability
4345) untouristed
4346) untowardness (merged into untoward)
4347) untransferable
4348) untranslatability
4349) untraversable
4350) untrendy (Oxford: informal)
4351) untrusting
4352) untuck (+ed)
4353) unknowingness (merged into unknowing)
4354) sideward (added singular)
4355) sidewall
4356) sideswipe (fixed: side-swipe)
4357) sidestroke (fixed: side-stroke)
4358) starchitect
4359) columnated
4360) starchitecture
4361) decretal (+plural)
4362) Decretum
4363) decubitus
4364) codebreaking
4365) codebreakers (added plural)
4366) codependence
4367) beatbox (+plural +er +ing)
4368) bandoneon
4369) bandpasses (added plural)
4370) photodiode (+plural)
4371) photodegrade
4372) photodegradation
4373) photodetector (+plural)
4374) photodissociation
4375) photoelectron (added singular)
4376) photoemission
4377) photoemissive
4378) photoemitter
4379) photoessay (+plural)
4380) photofit (+plural)
4381) photogram (+plural)
4382) landmines (added plural)
4383) shoreside
4384) shoreweed
4385) shorelark (+plural)
4386) bellbird (+plural)
4387) honeyeater (+plural)
4388) honeyguide (+plural)
4389) honeypot (+plural)
4390) honeysucker (+plural)
4391) honeywort
4392) snowberry (+plural)
4393) snowblade (+s +ing +er)
4394) snowblink
4395) granodiorite
4396) plutonic + Plutonic
4397) biostratigraphy
4398) biosurgery
4399) biosolids
4400) biosimilar (+plural)
4401) biospheric (merged into biosphere)
4402) biofuels (added plural)
4403) bioflavonoid (+plural)
4404) biometrical
4405) biometrician (+plural)
4406) ectoparasitic (merged into ectoparasite)
4407) microhabitat (+plural)
4408) study's (added the 's)
4409) anthropogenically (merged into anthropogenic)
4410) timescale (fixed: time-scale)
4411) spermatid (+plural +al)
4412) spermatocyte (+plural)
4413) spermatogonium
4414) spermatogonia (plural of previous word)
4415) seminiferous
4416) semiochemical (+plural)
4417) semiological
4418) elenchus
4419) elenchi (plural of previous word)
4420) alienee (+plural)
4421) aliform
4422) encephalon
4423) encephalography
4424) encephalograph
4425) encephalogram (+plural)
4426) myoclonus
4427) myoclonic
4428) capitular
4429) glomerulus
4430) glomeruli (plural of previous word)
4431) glomerular
4432) nephron (+plural)
4433) nephrotoxic
4434) nephrotoxicity
4435) nephrotoxin (+plural)
4436) vancomycin
4437) virality
4438) somewhen (Oxford: informal)
4439) somite (+plural)
4440) dichromatism
4441) dichromatic
4442) cephalothorax
4443) holotype (+plural)
4444) syntype (+plural)
4445) fishcakes (added plural)
4446) amatriciana
4447) amaurosis
4448) amaurotic
4449) foo (foo fighter)
4450) geoscience (+plural)
4451) geoscientist (+plural)
4452) botty (+plural)
4453) Swabia (place in Germany)
4454) Swabian
4455) clamant
4456) dragonet (+plural)
4457) bigeye (fish)
4458) bluefin (fish)
4459) yellowfin (fish)
4460) catalufa (+plural)
4461) catalyser
4462) APU
4463) dumbshow (+plural)
4464) dumbphone (+plural)
4465) dumbo (+plural - Oxford: informal)
4466) vetchling (+plural)
4467) buckthorn
4468) buckyball (+plural)
4469) nanomaterials (added plural)
4470) nanogram
4471) ng (Oxford: abbreviation of previous word)
4472) oxidase (+plural)
4473) pharaoh (+plural - changed to lower case - upper case only if name)
4474) abubble
4475) idiotype (+plural)


V2.18 - 1-NOV-2014
------------------
4476) mothercraft
4477) mainboard (+plural)
4478) maincrop
4479) tricuspid
4480) unicuspid
4481) fivestones
4482) fivesome
4483) Flathead (+plural - upper case - Kevin Atkinson wordlist 60)
4484) bioturbation
4485) pilgriming + pilgrimed (merged into pilgrim)
4486) pilgrimize (-IZE)
4487) pilgrimise (-ISE)
4488) rubescent
4489) evacuant
4490) Salvadorean (+plural +'s - Kevin Atkinson wordlist 60)
4491) statant
4492) Zarathustra's (added +'s - Kevin Atkinson wordlist 60)
4493) adventurist (+plural - Kevin Atkinson wordlist 60)
4494) acquiree (+plural)
4495) acquirer (+plural - Kevin Atkinson wordlist 60)
4496) answerphone (+plural - Kevin Atkinson wordlist 60)
4497) bulleted (merged into bullet)
4498) bedchambers (added plural - Kevin Atkinson wordlist 60)
4499) biodegrade (+s +ed +ing +ion  - Kevin Atkinson wordlist 60)
4500) blobfish (+plural)
4501) binman (Oxford: informal - Kevin Atkinson wordlist 60)
4502) binmen (plural of previous word - Kevin Atkinson wordlist 60)
4503) blowlamps (added plural - Kevin Atkinson wordlist 60)
4504) breathalyses (added plural - Kevin Atkinson wordlist 60)
4505) cellmates (added plural - Kevin Atkinson wordlist 60)
4506) chatlines (added plural - Kevin Atkinson wordlist 60)
4507) chirpily + chirpiness (merged into chirpy - Kevin Atkinson wordlist 60)
4508) computerate (Oxford: informal - Kevin Atkinson wordlist 60) 
4509) comfortless (merged into comfort - Kevin Atkinson wordlist 60)
4510) conchie (+plural - Oxford: informal - Kevin Atkinson wordlist 60)
4511) cookhouse (+plural - Kevin Atkinson wordlist 60)
4512) crabwise (Kevin Atkinson wordlist 60)
4513) crepuscular (Kevin Atkinson wordlist 60)
4514) cripes (Oxford: informal - Kevin Atkinson wordlist 60)
4515) criollo (+plural)
4516) cuboids (added plural - Kevin Atkinson wordlist 60)
4517) cuboidal (merged into cuboid)
4518) demonically (Kevin Atkinson wordlist 60)
4519) deselection (+plural - Kevin Atkinson wordlist 60)
4520) immunoblotting
4521) electrophorese
4522) electrophoretic + electrophoretically
4523) electrometer (+ic)
4524) electrometry
4525) electromyogram (+plural)
4526) multipolar
4527) multipolarity
4528) multipole
4529) unipolarity
4530) unipotent
4531) unironed
4532) uniserial
4533) unisexual (+ly)
4534) unisexuality
4535) unironic (+ly)
4536) UNISON
4537) unissued
4538) deep-discount
4539) supercalifragilisticexpialidocious (Oxford: informal)
4540) supercalifragilistic (another way of previous word)
4541) superbugs (added plural)
4542) autorickshaw (+plural)
4543) e-billing
4544) e-cash
4545) e-edition
4546) e-dating
4547) e-fit
4548) overclock (+ed +ing +s)
4549) overclocker (+plural +'s)
4550) overcheck (+plural)
4551) overcautiously + overcautiousness (merged into overcautious)
4552) overcaution
4553) overclass
4554) overcompensatingly
4555) overcompensatory
4556) overcommits (added plural)
4557) descale (+s +ing +ed - Kevin Atkinson wordlist 60)
4558) descaler (+plural)
4559) euryapsid (+plural)
4560) darkie (+plural - Kevin Atkinson wordlist 60)
4561) decathlete (+plural - Kevin Atkinson wordlist 60)
4562) dickhead (+plural - Kevin Atkinson wordlist 60)
4563) doodahs (added plural - Kevin Atkinson wordlist 60)
4564) doolally (Oxford: informal - Kevin Atkinson wordlist 60)
4565) doomster (+plural - Kevin Atkinson wordlist 60)
4566) doomwatch (+er +ers)
4567) draughtboard (+plural - Kevin Atkinson wordlist 60)
4568) ecolodge (+plural)
4569) ecofreak (+plural)
4570) ecofeminism
4571) ecofeminist (+plural)
4572) educationist (+plural - Kevin Atkinson wordlist 60)
4573) endgames (added plural - Kevin Atkinson wordlist 60)
4574) consecrator
4575) consecratory
4576) emblematical
4577) emblematically (Kevin Atkinson wordlist 60)
4578) emblematist
4579) dozier+doziest+dozily+doziness (merged into dozy - Kevin Atkinson wordlist 60)
4580) ecu (+plural - Kevin Atkinson wordlist 60)
4581) diaconate
4582) premodify (+s +ing +ed +ion +er)
4583) premonish (+ing)
4584) Petersburg (name)
4585) entryphone (+plural - Kevin Atkinson wordlist 60)
4586) extrajudicial (Kevin Atkinson wordlist 60)
4587) extrajudicially (merged into extrajudicial)
4588) selfhood
4589) continuants (added plural)
4590) cacuminal
4591) coarticulation (+plural)
4592) silicify (+s +ing +ed +ion)
4593) extraordinaire (Kevin Atkinson wordlist 60)
4594) extranet (+plural)
4595) extramusical
4596) fab (+s +ing +ed)
4597) fatsos (added plural - Kevin Atkinson wordlist 60)
4598) fitment (added singular - Kevin Atkinson wordlist 60)
4599) fluvial (Kevin Atkinson wordlist 60)
4600) fuckhead (+plural - Kevin Atkinson wordlist 60)
4601) fuckwit (Oxford: vulgar slang)
4602) fucoxanthin
4603) carotenoid (+plural)
4604) lutein
4605) ZX (ZX Spectrum - computer from the 1980s)
4606) exabyte (+plural)
4607) EB (abbreviation of previous word)
4608) yottabyte (+plural)
4609) YB (abbreviation of previous word)
4610) zettabytes (added plural)
4611) ZB (abbreviation of previous word)
4612) Amiga's (added the 's)
4613) lower-level
4614) higher-level
4615) akasha (+ic)
4616) inbreathe (+ed +ing)
4617) fracking
4618) commenters (added plural)
4619) bynames (added plural)
4620) bye-byes (added plural)
4621) goodliness (merged into goodly)
4622) mailshots (added plural)
4623) backbend (+plural)
4624) jawless (merged into jaw)
4625) hagfish (+plural)
4626) beardfish (+plural)
4627) clade (+plural)
4628) taxon
4629) taxa (plural of previous word)
4630) avoision
4631) kyu (martial arts)
4632) dan (martial arts)
4633) budo (martial arts)
4634) bokken (martial arts)
4635) kyudo (martial arts)
4636) kendo (martial arts)
4637) lamers (added plural)
4635) 2D
4636) 3D
4637) galactically (merged into galactic)
4638) galactagogue (+plural)
4639) prolactin
4640) oxytocin
4641) oxytocic
4642) monoamine (+plural)
4643) monamine (+plural - another way of previous word)
4644) neuropeptide (+plural)
4645) neuropathological
4646) neuropathologist (+plural)
4647) neuroma (+plural)
4648) neuromata (another plural of previous word)
4649) geriatrician (+plural)
4650) defervescence
4651) anastomose (added singular +ing +ed)
4652) arthroplasty
4653) vegetal
4654) aepyornis
4655) calcination (merged into calcine)
4656) dimercaprol
4657) dimerous
4658) meerkat (+plural)
4659) tamarin (+plural)
4660) mara (+plural)
4661) suricate (+plural)
4662) Bannockburn (place)
4663) Mozillian (+'s +plural)
4664) annock
4665) bendy (+ier iest +ness)
4666) funfairs (added plural - Kevin Atkinson wordlist 60)
4667) goodie (another way of goody)
4668) ganja (Kevin Atkinson wordlist 60)
4669) charas
4670) spliff (+plural)
4671) afebrile
4672) Eddington (+'s - name)
4673) dyskinesia (+plural)
4674) dyspraxia
4675) dysregulation
4676) bariatrics
4677) bronchodilator (+plural)
4678) bronchodilation
4679) pharmacodynamic (+plural)
4680) zonation (+plural)
4681) nomenclator
4682) nomenclatural
4683) monotypic
4684) monovalent
4685) nonagon
4686) gasman (Kevin Atkinson wordlist 60)
4687) gasmen (plural of previous word - Kevin Atkinson wordlist 60)
4688) gaslighting (merged into gaslight)
4689) gaslit
4690) gasometers (added plural - Kevin Atkinson wordlist 60)
4691) bionomic (+plural)
4692) anticathode (+plural)
4693) anticholinergic (+plural)
4694) misdial (+s +ing +ed)
4695) misdescribe (+s +ed +ion)
4696) misdoubt (+s +ed)
4697) compand (+s +er +ing +ed)
4698) datacomms
4699) datacoms (another way of previous word)
4700) dataveillance
4701) case-sensitive
4702) backstamp (+plural)
4703) backstay (+plural)
4704) donkeyman
4705) donkeymen (plural of previous word)
4706) bathymeter (+plural +ic)
4707) bathymetry
4708) scaleable (another way of scalable)
4709) intercrop (+s +ing +ed)
4710) intercorrelation (+plural)
4711) intercorrelate
4712) baseload
4713) baselessly
4714) baseborn
4715) subcell (Collins)
4716) subcellular (Collins)
4717) supralunar (Collins)
4718) supratemporal (Collins)
4719) hepatocellular (Collins)
4720) gluteals (added plural)
4721) gastrocnemius
4722) gastrocnemii (plural of previous word)
4723) soleus
4724) appendicular
4725) scute (+plural)
4726) allosaurus (+plural)
4727) allopurinol
4728) colchicine
4729) disarticulate (+ion)
4730) flexicuffs (Macmillan)
4731) plasticuffs (Macmillan)
4732) PCIe
4733) RJ45
4734) blackboy (+plural)
4735) mistle (mistle thrush)
4736) broadbill (+plural)
4737) Arabism
4738) Arabist (+plural)
4739) arabinose
4740) galactose
4741) xylose
4742) mannose
4743) fibrinogen
4744) extravascular
4745) intravascular (+ly)
4746) extravasate (+ed)
4747) petechia
4748) petechiae (plural of previous word)
4749) petcock
4750) petasus
4751) undersheriff (fixed: under-sheriff)
4752) underset (+s +ing)
4753) unviable
4754) unviability
4755) unviolated
4756) Eliot's (added the 's)
4757) unvitiated
4758) unwaged
4759) unwaisted
4760) hashtag (+plural)
4761) cliched (merged into cliche)
4762) gotcha (Oxford: informal)
4763) gotcher (Oxford: informal - another way of previous word)
4764) gossypol
4765) cottonweed (+plural)
4766) oligotrophy (+ic)
4767) eutrophic
4768) lapis
4769) UTF-16
4770) UTF-32
4771) 1D
4772) unvented
4773) unvaryingness
4774) meningococcus
4775) meningococci (plural of previous word)
4776) meningococcal
4777) serogroup (Collins)
4778) anti-war (fixed antiwar)
4779) anti-Western
4780) anti-tetanus
4781) anorgasmia
4782) activewear
4783) Diana's (added the 's)
4784) misestimate (+ed)
4785) misestimation (+plural)
4786) QuickTime
4787) trackpad
4788) ringtone (+plural)
4789) Netflix (company)
4790) podcasts (added plural)
4791) waifish
4792) waiflike
4793) retroelement (+plural)
4794) bivalents (added plural)
4795) non-disjunction
4796) spermatogenesis
4797) spermatophore (+plural)
4798) oviposit (+s +ing +ed)
4799) oviparity
4800) asphyxial
4801) asphyxiant (+plural)
4802) diverticular
4803) diverticulum
4804) diverticula (plural of previous word)
4805) axilla
4806) axillae (plural of previous word)
4807) aviating + aviated (merged into aviate)
4808) prosoma
4809) tagma
4810) tagmata (plural of previous word)
4811) antivivisection
4812) antivivisectionism
4813) gangstas (added plural - Kevin Atkinson wordlist 60)
4814) counterchange (+s +ed +ing)
4815) counterclaims (added plural)
4816) counterculture (fixed: counter-culture)
4817) countercultural
4818) counterdemonstration (+plural)
4819) Blairism
4820) Blairite (+plural)
4821) church (added also lower case)
4822) churchy + churchier + churchiest
4823) chuppah + chuppa
4824) chuppot (plural of previous word)
4825) chupacabra (+plural +'s)
4826) Gawd + gawd (Oxford: informal - Kevin Atkinson wordlist 60)
4827) geddit (Oxford: informal - Kevin Atkinson wordlist 60)
4828) gasholder (+plural - Kevin Atkinson wordlist 60)
4829) sunseeker (+plural - Oxford: informal)
4830) Jain (+plural)
4831) jailbreaker (merged into jailbreak)
4832) detainers (added plural)
4833) gigawatts (added plural - Kevin Atkinson wordlist 60)
4834) GW (abbreviation of previous word)
4835) gigaton (+plural)
4836) gigatonne (+plural - another way of previous word)
4837) godawful (Kevin Atkinson wordlist 60)
4838) goddammit (Collins - informal)
4839) godlessly (merged into godless - Kevin Atkinson wordlist 60)
4840) giros (added plural - Kevin Atkinson wordlist 60)
4841) groupware (+'s - Kevin Atkinson wordlist 60)
4842) Debian
4843) versioned + versioning + versional (merged into version)
4844) diehard (fixed: die-hard)
4845) versicoloured
4846) verset (+plural)
4847) triolet (+plural)
4848) sestina (+plural)
4849) villanelle (+plural)
4850) rondel (+plural)
4851) pantoum (+plural)
4852) pantryman
4853) pantrymen (plural of previous word)
4854) groundburst (Collins)
4855) adaptationism
4856) adaptationist
4857) adaptably
4858) Emmy (+plural - the awards)
4859) greenflies (added plural - Kevin Atkinson wordlist 60)
4860) gymslip (+plural - Kevin Atkinson wordlist 60)
4861) hairsprays (added plural - Kevin Atkinson wordlist 60)
4862) colourist (+plural)
4863) colourway (+plural)
4864) colouristic (+ally)
4865) watercolourist (+plural - merged into watercolour)
4866) watercock (+plural)
4867) waterdog (+plural)
4868) cottonmouth (+plural)
4869) deergrass
4870) dotterel (+plural)
4871) dottily (merged into dotty)
4872) Crockford (+'s)
4873) Avesta
4874) Vedic
4875) Kubuntu (+'s)
4876) ESA (+'s)
4877) ESC
4878) escabeche
4879) escalade
4880) rockwork (Collins)
4881) rocketeer (+plural)
4882) aerobrake (+ing)
4883) aerobicist (+plural)
4884) aerobiology
4885) neurophysiological
4886) neurophysiologist (+plural)
4887) atherogenic
4888) atherogenesis
4889) pathophysiology
4890) pathophysiologic
4891) pathophysiological
4892) pathophysiologically
4893) pathophysiologist
4894) colonoscopes (added plural)
4895) autotransformers (added plural)
4896) autotoxin (+plural)
4897) autotoxic
4898) allogenic
4899) catastrophists (added plural)
4900) tailplanes (added plural)
4901) tailspinning (merged into tailspin)
4902) tailspun (past of tailspin)
4903) tailwater (+plural)
4904) fishable (merged into fish)
4905) hemimetabolous
4906) hemimetabolic
4907) DLL (+plural)
4908) aerophagy


V2.19 - 1-DEC-2014
------------------
4909) Zuckerberg (+'s - name)
4910) XL
4911) XXL
4912) Facebooking (+ed +er +ers - merged into Facebook)
4913) convocational
4914) convolutional (merged into convolution)
4915) denticle (+plural)
4916) reticulary (Collins - another way for reticulate)
4917) cephalopods (added plural)
4918) crinoid (+plural)
4919) brachiopod (+plural)
4920) conodont (+plural)
4921) intracompany (Collins)
4922) agglutinogen (+plural)
4923) downdrift
4924) downcut (+s +ing)
4925) upcurve (+ed - Collins + Oxford)
4926) upcycle (+ed)
4927) upcast
4928) faceworker
4929) homeworking (merged into homework)
4930) homewrecker (+plural)
4931) teleworker (+plural - Macmillan)
4932) jack-o'-lantern (+plural)
4933) jackanapes
4934) goosefoot (+plural)
4935) sumpweed (Collins)
4936) sandbagger (+plural)
4937) shopfront (+plural)
4938) shopaholic (+plural)
4939) shopfitter (+plural)
4940) shopfitting
4941) boutiquey (Oxford: informal)
4942) Baphomet (Collins)
4943) baphometic (Collins)
4944) Mendes (name)
4945) Judaeo-Christian (Collins)
4946) magus
4947) Szandor (name)
4948) cosmopolite (+plural)
4949) cosmopolitanize (+ed -IZE)
4050) cosmopolitanise (+ed -ISE)
4051) spacewalks (added plural)
4052) spaceplane (+plural)
4053) spacefaring
4054) Lilith (+'s - name)
4055) Samael (+'s - name)
4056) clubmate (+plural)
4057) curtilage (+plural)
4058) circumstantiality
4059) disburden
4060) disbud (+s +ing +ed)
4061) disbenefit (+plural)
4062) disbound
4063) basketmaker (+plural)
4064) basketmaking
4065) gadgeteer (+plural)
4066) gadgety
4067) cabbalistic
4068) cabbalist (+plural)
4069) cabbalism
4070) Kabbalah + Kabbala + Cabbala + Qabalah
4071) Kabbalism
4072) Kabbalist (+plural)
4073) Kabbalistic
4074) Hasidism + Chasidism + Chassidism + Hassidism
4075) copyread (+ing +er)
4076) hardbody (+plural +ed)
4077) Hardecanute (+'s - name)
4078) bentonite
4079) citrine
4080) ricking + ricked (merged into rick)
4081) cowfish (+plural)
4082) diffusivities (added plural)
4083) cal (Oxford: small calorie, abbreviation)
4084) Cal (Oxford: large calorie, abbreviation)
4085) multispectral
4086) BTC (Oxford: abbreviation of bitcoin)
4087) backbiting
4088) deracinate (+ed)
4089) train's (added the 's)
4090) decoct (+ed +ing)
4091) decocture
4092) dibasic
4093) monobasic
4094) diamine (+plural)
4095) triaxial
4096) triatomic
4097) triathlete (+plural)
4098) deadeners (added plural)
4099) copperas
4100) spadefoot (+plural)
4101) spadefish (+plural)
4102) spadille
4103) cribellum
4104) cribella (plural of previous word)
4105) cribellate
4106) chelicerate (+plural)
4107) lexicology
4108) lexicological + lexicologically
4109) pomodoro
4110) unobvious (Collins)
4111) recompress (+s +ed +ing +ion)
4112) redistributor (+plural)
4113) hyphenator (+plural)
4114) refactor (+s +ed +ing)
4115) misdistribution (Collins)
4116) maldistribution
4117) maldistributed
4118) distributary (+plural)
4119) dogwatch (+plural)
4120) dogstail
4121) dogskin
4122) dogsbodies + dogsbodying (merged into dogsbody)
4123) austenite (+ic)
4124) Ozzies (added plural)
4125) microstructure (+plural +al)
4126) cementite
4127) pearlite
4128) contactor (+plural)
4129) elastomers (added plural)
4130) elastomeric
4131) nitro (Oxford: Short for nitroglycerine)
4132) polyolefin (Collins)
4133) gapping
4134) hypotaxis
4135) hypotactic
4136) parataxis
4137) paratactic (+ally)
4138) phasor (+plural)
4139) phasic
4140) reactance
4141) acetanilide
4142) orcein
4143) cocreate (Collins)
4144) Cota-Robles (name)
4145) DSLR (+s +'s)
4146) HDSLR (+s +'s)
4147) SDK (Collins)
4148) FreeBSD
4149) AutoCAD (+'s)
4150) CAD (in upper case - Oxford: short for Computer-aided design)
4151) cadaverousness (merged into cadaverous)
4152) CADCAM
4153) androecium
4154) androecia (plural of previous word)
4155) androecial
4156) gynoecium
4157) gynoecia (plural of previous word) 
4158) gynocentric
4159) unallowable
4160) unallied
4161) unalive
4162) unalarmed
4163) unaired
4164) unaggressive
4165) unaesthetic
4166) unadvisedly
4167) unadjacent
4168) unaddressed
4169) IRCop (+plural)
4170) moldavite (Collins)
4171) Jaban (+'s - name)
4172) Joseph's (added the 's)
4173) starships (added plural)
4174) Anunnaki
4175) Lyra
4176) Sheran (+'s - name)
4177) relink (+ed +ing)
4178) reliquiae
4179) relist (+ed +ing)
4180) relleno (+plural)
4181) tortious (+ly)
4182) licensors (added plural)
4183) RTFM (Oxford: vulgar slang)
4184) Thunderbird's (added the 's)
4185) personas (added plural)
4186) Los (Los Angeles - city)
4187) neuroanatomical
4188) neuroanatomist (+plural)
4189) neurochemistry
4190) neurochemist
4191) neurochemical
4192) nephrology
4193) nephrological
4194) nephrologist (+plural)
4195) interventional (merged into intervention)
4196) subcortical
4197) putamen (+plural)
4198) putamina (another plural of previous word)
4199) nondevelopment (Collins)
4200) subdevelopment (Collins)
4201) antidevelopment (Collins)
4202) predevelopment (Collins)
4203) neurohormonal (Collins)
4204) military's (added the 's)
4205) gizmo (+s +'s)
4206) gismo (+s +'s - another way of previous word)
4207) antimilitarism (Collins)
4208) militaries (added plural)
4209) Clausewitz (+'s - name)
4210) antimilitarist (+plural - Collins)
4211) hairgrip (+plural - Kevin Atkinson wordlist 60)
4212) bikeable
4213) bikable (another way of previous word)
4214) BMX
4215) boneshaker (+plural - Oxford: informal)
4216) pushbike (+plural - Oxford: informal - Fixed: push-bike)
4217) pushful (+ly +ness)
4218) pushpit
4219) lectionary (+plural)
4220) commination (+plural)
4221) comminatory
4222) grotty (+ier +iest +ness - Kevin Atkinson wordlist 60)
4223) polenta
4224) carpaccio
4225) Carpaccio (name)
4226) footy (Oxford: informal - Kevin Atkinson wordlist 60)
4227) footie (another way of previous word - Kevin Atkinson wordlist 60)
4228) footslog (+s +ing +ed +er +ers)
4229) fleapit (+plural - Kevin Atkinson wordlist 60)
4230) glaive (+plural)
4231) whoremaster (+plural)
4232) gangmaster (+plural)
4233) spymaster (+plural)
4234) workmaster (Collins)
4235) weaponeer (+ing - Collins)
4236) ironmaster (+plural)
4237) goldfield (+plural - Kevin Atkinson wordlist 60)
4238) headcase (+plural - Oxford: informal - Kevin Atkinson wordlist 60)
4239) headcam
4240) headbutt (+ed +ing)
4241) headbanging
4242) headbanger (+plural)
4243) headage
4244) suckler (+plural)
4245) unweaned
4246) unweary
4247) unwedded (+ness - another way of unwed)
4248) unweighted
4249) unwept
4250) unwhipped
4251) unwinnable
4252) unwisdom
4253) unwired
4254) unwinking (+ly)
4255) helipad (+plural - Kevin Atkinson wordlist 60)
4256) holism (Kevin Atkinson wordlist 60)
4257) holist (+plural)
4258) serialism
4259) serialist (+plural)
4260) atonalism
4261) atonalist (+plural)
4262) jeeze + geez (another way of jeez - Oxford: informal)
4263) wearables (added plural)
4264) millennials (added plural)
4265) Jehovah's (added the 's)
4266) millenarianist (+plural)
4267) millenarians (added plural)
4268) apocalyptically
4269) eschatological
4270) eschatologist (+plural)
4271) unmediated
4272) unmelodic
4273) unmelted
4274) unmemorably (merged into unmemorable)
4275) unmercifulness (merged into unmerciful)
4276) unmetalled
4277) unmetered
4278) unmetrical
4279) unmethodically (merged into unmethodical)
4280) unmilitary
4281) unmindfully
4282) unmindfulness
4283) unmissed
4284) unmistaken
4285) unmitigatedly
4286) bombardon (+plural)
4287) saxhorn (+plural)
4288) sedum (+plural)
4289) bellflowers (added plural)
4290) untrusty (Collins)
4291) counterstyle (Collins)
4292) unthorough (Collins)
4293) uninstallation (+plural)
4294) hoick (+s +ing +ed - Kevin Atkinson wordlist 60)
4295) hoofers (added plural - Kevin Atkinson wordlist 60)
4296) hetero (+s +'s - Kevin Atkinson wordlist 60)
4297) hotpots (added plural - Kevin Atkinson wordlist 60)
4298) interwar (Kevin Atkinson wordlist 60)
4299) ironwood (added singular - Kevin Atkinson wordlist 60)
4300) jerrycan (+plural - Kevin Atkinson wordlist 60)
4301) jerrican (+plural - another way of previous word)
4302) leucotomy (+plural - Kevin Atkinson wordlist 60)
4303) lexis (changed to lower case - Oxford)
4304) lexigram (+plural)
4305) errorless (merged into error)
4306) catchweight
4307) listenable (Kevin Atkinson wordlist 60)
4308) listenability
4309) linkman + linkmen (Kevin Atkinson wordlist 60)
4310) lughole (+plural - Kevin Atkinson wordlist 60)
4311) lulus (added plural - Kevin Atkinson wordlist 60)
4312) lumpectomy (+plural - Kevin Atkinson wordlist 60)
4313) meteorologically
4314) unbearableness
4315) unbeautiful (+ly)
4316) unbefitting (+ly +ness)
4317) unbeholden
4318) irreligion
4319) irreligionist
4320) irreligiously
4321) irreligiousness
4322) irremissible
4323) irremovability
4324) irremovably
4325) irreplaceably
4326) irreproachability
4327) irreproducible
4328) bcc (Blind carbon copy - used in e-mails)
4329) b.c.c. (another way of previous word)
4330) aberdevine (Collins)
4331) siskin (Collins)
4332) uninstal (+s - Oxford: another way in British for uninstall)
4333) prolongedly
4334) prolusion
4335) phonecard (+plural)
4336) collet
4337) photocard (Collins)
4338) photocall (+plural)
4339) phonematic (Collins - another way of phonemic)
4340) battlespace (Collins)
4341) counterbored (merged into counterbore)
4342) bolthole (+plural)
4343) necrophile
4344) necrophilic
4345) necrophilism
4346) necrophilist
4347) necrophobia
4348) theurgy
4349) theurgic
4350) theurgical
4351) theurgist (+plural)
4352) Wiccans (added plural)
4353) rosacea
4354) asthenia
4355) asthenic
4356) cellulitis
4357) barotrauma
4358) necrose (+ed)
4359) necropsies (plural of necropsy)
4360) PayPal's (added the 's)
4361) majeure (force majeure)
4362) benchwork
4363) dogfood (+ing)
4364) outgun (+s +ing +ed)
4365) Dakota's (added the 's)
4366) Hutterite (+plural)
4367) cytophotometry (+ic)
4368) cytophotometer (+plural)
4369) macronuclear (Collins)
4370) conjugant (Collins)
4371) conjurator (Collins)
4372) conjugality
4373) unconjugal (Collins)
4374) sexercise (Collins)
4375) sexennium (+plural)
4376) sexennia (another plural of previous word)
4377) cinquefoil
4378) sexologic (Collins)
4379) sexological
4380) uncontainable
4381) unconstructive
4382) muscleman + musclemen (Kevin Atkinson wordlist 60)
4383) muscly (+ier +iest - Kevin Atkinson wordlist 60)
4384) weanlings (added plural)
4385) unwatchful
4386) unwatered
4387) myxomatosis (Kevin Atkinson wordlist 60)
4388) naturism (Kevin Atkinson wordlist 60)
4389) naturists (add plural - Kevin Atkinson wordlist 60)
4390) non-official
4391) nosebags (add plural - Kevin Atkinson wordlist 60)
4392) nosepiece
4393) noshery (+plural)
4394) nosocomial
4395) electroreception
4396) electroreceptor
4397) planktic
4398) planktonic
4399) foraminifer (+plural)
4400) foraminifera (another plural of previous word)
4401) foraminiferal
4402) foraminiferan (+plural)
4403) foraminiferous
4404) zooxanthella
4405) zooxanthellae (plural of previous word)
4406) zooxanthellate
4407) SCOWL (+'s)
4408) readme + README (Collins)
4409) rechipping
4410) officialese (Kevin Atkinson wordlist 60)
4411) oilcan (+plural - Kevin Atkinson wordlist 60)
4412) oilbird (+plural)
4413) goatsucker (+plural)
4414) nightjar (+plural)
4415) chirr (+ing +ed)
4416) churr (+ing +ed - another way of previous word)
4417) chiropteran (+plural)
4418) cardiothoracic
4419) cardiorespiratory
4420) sequela
4421) sequelae (plural of previous word)
4422) sepulture
4423) orangish
4424) orangewood (Collins)
4425) outperformance
4426) outworker (+plural - Kevin Atkinson wordlist 60)
4427) outworld (+er +ers)
4428) oxtails (added plural - Kevin Atkinson wordlist 60)
4429) paintball (+s +er +ers +ing)
4430) paparazzo (Kevin Atkinson wordlist 60)
4431) phat (Kevin Atkinson wordlist 60)
4432) Picasso (name)
4433) Picassoesque
4434) picaro (+plural)
4435) pinewood (+plural - Kevin Atkinson wordlist 60)
4436) pitheads (added plural - Kevin Atkinson wordlist 60)
4437) playschools (added plural - Kevin Atkinson wordlist 60)
4438) playmaker (+plural)
4439) playmaking
4440) racegoer (added singular)
4441) racecard (+plural)
4442) poppets (added plural - Kevin Atkinson wordlist 60)
4443) postmenopausal (Kevin Atkinson wordlist 60)
4444) precast (+s +ing)
4445) adenomatous
4446) psychoanalytically (Kevin Atkinson wordlist 60)
4447) bibliotherapy
4448) punchbag (+plural - Kevin Atkinson wordlist 60)
4449) headguard (+plural)
4450) quick-fire
4451) quietist (+plural +ic)
4452) ragga
4453) railcard (+plural - Kevin Atkinson wordlist 60)
4454) railbus (+plural)
4455) railcar (+plural)
4456) springless
4457) springlet (+plural)
4458) hornbeams (added plural)
4459) junglefowl
4460) backstory (+plural)
4461) bandfish (+plural)
4462) agrestic
4463) agrichemical (+plural - another way of agrochemical)
4464) apatosaurus
4465) brachiosaurus
4466) diplodocus
4467) daybook (+plural)
4468) dayboat
4469) daysailor
4470) bifold
4471) windslab (+plural)
4472) dynode (+plural)
4473) defervesce (+ed)
4474) defibrillation
4475) defibrillate (+ed +ion)
4476) discographer (+plural)
4477) discoid (+al)
4478) venter (+plural)
4479) ventail
4480) coeternal (+ally)
4481) fluoroscopy
4482) fluoroquinolone
4483) non-pregnant
4484) sulfacetamide
4485) microsurgeons (added plural)
4486) colossuses (another plural of colossus)
4487) maldevelopment
4488) cicatrize (+s +ed +ing +ion -IZE-)
4489) cicatrise (+s +ed +ing +ion -ISE-)
4490) cicatricial
4491) novelly (merged into novel)
4492) lesbigay (+plural - Oxford: informal)
4493) lesbo (+plural - Oxford: informal offensive)
4494) Lesbos (Island in Greece)
4495) Sprachgefühl
4496) eggcorn
4497) advocaat
4498) caipirinha (+plural)
4499) cachaca
4500) chincherinchee (+plural)
4501) chinaberry (+plural)
4502) cranesbill (+plural)
4503) abutilon
4504) clivia (+s +'s)
4505) beddable (Oxford: informal)
4506) fanciable (Oxford: informal)
4507) culverin (+plural)
4508) localism (added singular)
4509) localist (+plural)
4510) bacteraemia
4511) untagging
4512) freemium
4513) freeride (+ing)
4514) biathlete (+plural)
4515) fieldcraft
4516) fairlead (+plural)
4517) Bel (another word for Baal)
4518) Dione (name)
4519) Enceladus (name)
4520) mucocutaneous (Collins)
4521) bpm (abbreviation: beats per minute)
4522) BPhil
4523) caravanners (added plural)
4524) caravanette (+plural)
4525) coatigan
4526) all-knowing
4527) supranationality
4528) supramundane
4529) subnuclear
4530) subnormality
4531) suboptimally (merged into suboptimal)
4532) subordinary (+plural)
4533) subnetwork
4534) submunition (+plural)
4535) submultiple (+plural)
4536) submucosa
4537) submucosae (plural of previous word)
4538) gradable
4539) gradability
4540) cross-beam (+plural)
4541) cross-border
4542) cross-colour
4543) cross-contaminate (+ed +ing)
4544) cross-connection (+plural)
4545) cross-correlate (+ed)
4546) cross-correlation (+plural)
4547) i-mode
4548) I-beam (+plural)
4549) I-chun (variant of Yichun)
4550) Yichun (city in China)
4551) demos
4552) demoi (plural of previous word)
4553) demotivation
4554) goateed (merged into goatee)
4555) swallow-tailed
4556) developable
4557) devein (+ed)
4558) ethnologic (merged into ethnology)
4559) ethnologically
4560) folkloristic
4561) folklife
4562) theanthropic
4563) guncotton
4564) tagliatelle
4565) capellini
4566) fettucine + fettucini (there are three words for this, one was already in)
4567) fettuccine (fix: removed plural, since this is already a plural word)


V2.20 - 1-JAN-2015
------------------
4568) scramjet (+plural)
4569) neurofeedback (Collins)
4570) astrochemists (added plural)
4571) astrocompass
4572) m (abbreviation)
4573) ratbag (+plural - Kevin Atkinson wordlist 60)
4574) Rottweiler (+plural - but uppercase - Kevin Atkinson wordlist 60)
4575) rotters (added plural - Kevin Atkinson wordlist 60)
4576) roughcast (Kevin Atkinson wordlist 60)
4577) rowlock (+plural - Kevin Atkinson wordlist 60)
4578) schoolkid (+plural - Kevin Atkinson wordlist 60)
4579) exteroceptor (+plural)
4580) interoceptor (+plural)
4581) chemoceptors (+plural - Collins)
4582) proprioceptor (+plural)
4583) propshaft
4584) proprioceptively
4585) bayside
4586) earthnut
4587) pignut (+plural)
4588) androcracy (+plural)
4589) androcratic
4590) androcentric
4591) androcentrism
4592) partridgeberry (+plural)
4593) anabiosis
4594) anabiotic
4595) chlorosis
4596) chloroses (plural of previous word)
4597) chlorotic
4598) aleurone
4599) acropetal (+ally)
4600) basipetal (+ally)
4601) taproot (+plural)
4602) tapster (+plural)
4603) digestif (added singular)
4604) predigestion
4605) ptyalin
4606) Carnivora (plural noun)
4607) extendability
4608) extendable
4609) multiscreen (Collins)
4610) multirange (Collins)
4611) nonequilibrium (Collins)
4612) OODA (loop - military)
4613) differentia
4614) differentiae (plural of previous word)
4615) ecotourism (+'s)
4616) ecotour (+plural)
4617) ecotourist (+plural)
4618) ecotoxicology
4619) ecotoxicological
4620) ecotoxicologist
4621) nontarget (Collins)
4622) ecotown (+plural)
4623) arcology (+plural)
4624) bioremediation
4625) bioregion (+plural)
4626) bioregional
4627) cropland (+plural)
4628) autoxidation
4629) autoxidize (+s +ing - IZE)
4630) autoxidise (+s +ing - ISE)
4631) autotype (+s)
4632) autogenesis (Collins)
4633) autogeny (Collins: another way of previous word)
4634) autogenetic (Collins)
4635) electrogenesis (Collins)
4636) Valium
4637) Librium
4638) Orbison (+'s - name)
4639) intrathecal (+ally)
4640) intraspecific
4641) genet (+plural)
4642) antechapel
4643) bioplay
4644) bioplasm (+ic - Collins)
4645) emoticon (+plural)
4646) emotionalist (+plural)
4647) subnational (Collins)
4648) anti-government
4649) nongovernment (Collins)
4650) nongovernmental (Collins)
4651) supergovernment (Collins)
4652) governmentalism (Collins)
4653) incrementalism
4654) incrementalist (+plural)
4655) inadaptive (Collins)
4656) adaption (+plural)
4657) preadaptive (Collins)
4658) adaptogen (+plural)
4659) balneology
4660) balneological
4661) balneologist (+plural)
4662) balneotherapy
4663) caramelization (-IZE)
4664) caramelisation (-ISE)
4665) carambola (+plural)
4666) eggfruit (Collins)
4667) satsuma (+plural)
4668) Satsuma (name of province)
4669) blackberried (merged into blackberry)
4670) hackberry (+plural)
4671) hackathon (+plural - Oxford: informal)
4672) incurvate (+ed)
4673) incurvation (+plural)
4674) railbed (Collins)
4675) railage
4676) railless
4677) bogie (+plural)
4678) bogland
4679) Achill (island)
4680) deuterons (added plural)
4681) photosystem (+plural)
4682) phototherapy
4683) phototransistor
4684) phototropism
4685) phototropic
4686) phototaxis
4687) phototaxes (plural of previous word)
4688) phototactic
4689) heliotropism
4690) heliotropic
4691) shitwork (Oxford: vulgar slang)
4692) wetware
4693) disincorporate (+ing)
4694) disinfest (+ing +ion)
4695) disintermediate (+ed +ion)
4696) disinvent (+ed)
4697) disinvite (+ed +ing)
4698) disinvoltura
4699) disjoins + disjoined (merged into disjoin)
4700) ubuntu (+'s - also allow lower case)
4701) permalink (+plural)
4702) permaculture
4703) permadeath
4704) foldback (Collins)
4705) readvance (Collins)
4706) readapt (+s +ing +ed +ion)
4707) readerly
4708) undersense (Collins)
4709) letterset
4710) intersex (+plural)
4711) interspace (+s +ed)
4712) interspinal
4713) interspinous
4714) interstadial
4715) intersegmental (+ally)
4716) Peterson (+'s - name)
4717) Jespersen (+'s - name)
4718) petrogeny
4719) petrogenesis (another way of the previous word)
4720) countercurrent (+plural)
4721) counterfactual (+plural)
4722) firebug (fixed: fire-bug and added plural)
4723) firepit
4724) firedog (+plural)
4725) palazzo (+plural)
4726) palazzi (another plural way of the previous word)
4727) palea
4728) paleae (plural of previous word)
4729) extrorse
4730) introrse
4731) introspect
4732) anadromous
4733) catadromous
4734) intrigant
4735) colourable
4736) cryptococcosis
4737) cryptococcal
4738) coolth
4739) CoolMax (trademark)
4740) bombazine
4741) breathability
4742) Pennsylvanian (+plural)
4743) chetrum (+plural - currency)
4744) afghani (+plural - currency)
4745) avoparcin
4746) enterovirus (+plural)
4747) rhinovirus (+plural)
4748) picornavirus (+plural)
4749) coronavirus (+plural)
4750) VS (Veterinary Surgeon)
4751) vs (Versus)
4752) vroom (+ed)
4753) min
4754) secs (added plural)
4755) SECAM
4756) offshoring
4757) vegetatively
4758) vegetativeness
4759) sargassum
4760) clubroot
4761) nightclubber (+plural - merged into nightclub)
4762) mentorship (Collins)
4763) ballgown (+plural)
4764) videographers (added plural)
4765) post-tension
4766) extensometer (+plural)
4767) floodwater (+plural)
4768) floorcloth (+plural)
4769) posthospital (Collins)
4770) posttreatment (Collins)
4771) twerk (+s +ing)
4772) choreology
4773) choreologist
4774) onesie (+plural)
4775) descriptivist (+plural)
4776) phenomenalism
4777) phenomenalist
4778) phenomenalistic
4779) physicalism
4780) physicalist
4781) physicalistic
4782) mentalism
4783) mentalistic
4784) neonomian (+plural - Collins)
4785) neonomianism (Collins)
4786) techno
4787) soundscape (+plural)
4788) Thomas's (added the 's)
4789) semiquaver (added singular)
4790) semiquinone (+plural)
4791) phototoxicity (Collins)
4792) superoxide
4793) superorder
4794) supernaturalist (+plural)
4795) supernormality
4796) superordinates (added plural)
4797) superplastic
4798) superplasticity
4799) supersale (Collins)
4800) superreal (Collins)
4801) supermale (Collins)
4802) supersalt (Collins)
4803) superjet (Collins)
4804) supersell (+er - Collins)
4805) superfusion
4806) superfood (+plural)
4807) superfluidity
4808) superfluid (+plural)
4809) superfatted
4810) auriculate
4811) auriculotherapy
4812) allopath
4813) catmint (+'s)
4814) catlick (Oxford: informal)
4815) circumvallate (+ed +ing)
4816) play's (added the 's)
4817) venography
4818) venographic
4819) venographically
4820) extravasation (merged into extravasate)
4821) vasculature
4822) cross-promotion
4823) cross-promote (+ed)
4824) cross-question (+ing +ed)
4825) home-cooked
4826) seedeater (fixed: seed-eater)
4827) bowerbird (+plural)
4828) brassfounder (+plural - Collins)
4829) coppernob (+plural - Oxford: informal)
4830) flybridge
4831) skybridge (Collins)
4832) fly-by (+plural "fly-bys")
4833) arm-wrestling
4834) arm-wrestle
4835) audax (+plural)
4836) valorousness (merged into valorous)
4837) venturous (+ness)
4838) temerarious (+ness)
4839) mesothelioma (+plural)
4840) mesotherapy
4841) mesothorax
4842) mesothoracic
4843) metathorax
4844) metathoracic
4845) interneural (Collins)
4846) stereoregular (Collins)
4847) stereometry
4848) stereological (+ally - Collins)
4849) stereogram (+plural)
4850) stereocamera (+plural)
4851) stereognosis
4852) stereognostic
4853) thermic
4854) interneuron
4855) interneurone (another way of previous word)
4856) interneuronal
4857) internode (+plural)
4858) depaint (Collins)
4859) dependingly (Collins)
4860) depravingly (Collins)
4861) depravation (merged into deprave - Collins)
4862) depreciatory
4863) Németh (+'s - name)
4864) László (+'s - name)
4865) cryptonymous
4866) monorchid (+plural)
4867) monorchidism
4868) cathodoluminescence
4869) anisotropically
4870) undrained
4871) metagenesis
4872) metagenetic
4873) metageneses (plural of metagenesis - Collins)
4874) muniment (+plural)
4875) alienage
4876) spondylolysis (Collins)
4877) agammaglobulinaemia
4878) agammaglobulinemia (another way of previous word)
4879) antepartum
4880) intrapartum (Collins)
4881) intrasexual (Collins)
4882) bicephalous
4883) electronica
4884) craniofacial
4885) craniology
4886) craniological
4887) craniologist (+plural)
4888) overexcitable
4889) overexcitement
4890) Napster (+'s - software)
4891) scattergun
4892) scattershot
4893) scatty (+ier +iest +ly +ness - Oxford: informal)
4894) impactful
4895) frameable + frameless (merged into frame)
4896) Xeon
4897) Centrino
4898) vPro
4899) Itanium (+'s)
4900) microclimate (+s +ic)
4901) microclimatically
4902) micropower
4903) microprobe (+plural)
4904) macrocephalic
4905) macrocephalous (another way of previous word)
4906) macrocephaly
4907) cerebrovascular
4908) intracranial (+ly)
4909) intractableness
4910) intradermal (+ly)
4911) micropropagation
4912) micropterous
4913) microRNA
4914) microscooter (+plural)
4915) yo-yos (added plural)
4916) yo-yoing, yo-yoed (verb - merged into yo-yo)
4917) yo-yoes (verb)
4918) foldable (merged into fold)
4919) microampere (Collins)
4920) Thomist (+plural)
4921) Thomistic
4922) covenantal
4923) covenantor (+plural +'s)
4924) scorelines (added plural - Kevin Atkinson wordlist 60)
4925) scorekeeping
4926) scrutineer (added singular - Kevin Atkinson wordlist 60)
4927) shambolic (+ally - Kevin Atkinson wordlist 60)
4928) sharpish (Kevin Atkinson wordlist 60)
4929) shebeen (+plural - Kevin Atkinson wordlist 60)
4930) sheetlike (Collins - Kevin Atkinson wordlist 60)
4931) shitload (Oxford: vulgar slang - Kevin Atkinson wordlist 60)
4932) shit-faced (Oxford: vulgar slang - Kevin Atkinson wordlist 60)
4933) bristlebird (+plural)
4934) birdling (+plural)
4935) shh (Kevin Atkinson wordlist 60)
4936) sh (another way of previous word)
4937) acupoint (+plural)
4938) aromatherapeutic
4939) showerproof (+ed +ing)
4940) circlip (+plural)
4941) desuperheater (+plural)
4942) Interweb
4943) intervocalically
4944) obstruent (+plural)
4945) sonorant (+plural)
4946) atonic
4947) atony
4948) homorganic
4949) homophile (+plural)
4050) servery (+plural - Kevin Atkinson wordlist 60)
4051) smoocher (+plural - merged into smooch)
4052) smoochy (+ier +iest)
4053) weirdo (+plural +'s - Kevin Atkinson wordlist 40)
4054) Mbit (Collins)
4055) XMP
4056) SNR (signal-to-noise ratio)
4057) THz
4058) skinful (Kevin Atkinson wordlist 60)
4059) skinfold (+plural)
4060) subscapular (Collins)
4061) smokily (merged into smoky)
4062) smokey (another way of smoky - Kevin Atkinson wordlist 60)
4063) snog (+s +ing +ed +er +ers - Kevin Atkinson wordlist 60)
4064) softback (+plural -  Kevin Atkinson wordlist 60)
4065) unremitted (+ly - Collins)
4066) slutty
4067) sluttishness (merged into sluttish)
4068) softcover (Kevin Atkinson wordlist 60)
4069) songwriting (Kevin Atkinson wordlist 60)
4070) sprog (+s +ing +ed - Kevin Atkinson wordlist 60)
4071) spiv (+plural - Kevin Atkinson wordlist 60)
4072) spivvish
4073) spivvy (+ier +iest)
4074) splake
4075) splanchnopleure
4076) endoblast (+ic - Collins)
4077) splashback (+plural)
4078) splashboard (+plural)
4079) splatterpunk (+plural)
4080) spleenwort (+plural)
4081) polypody (+plural)
4082) bryophyte (+plural)
4083) trichome (+plural)
4084) trichology
4085) trichological
4086) trichologist (+plural)
4087) stallholder (added singular - Kevin Atkinson wordlist 60)
4088) stallage
4089) pontage (Collins)
4090) lastage (Collins)
4091) arborist
4092) eyebright
4093) arrowgrass
4094) barrenwort (+plural)
4095) bioadhesive (+plural)
4096) buccal
4097) gingiva
4098) gingivae (plural of previous word)
4099) radula
4100) radulae (plural of previous word)
4101) radular
4102) graminivorous
4103) grammalogue
4104) anteverted
4105) anteroposterior
4106) anterolateral
4107) anterograde
4108) Atkins's (added the 's)
4109) churchianity (Collins)
4110) hiya (Oxford: informal)
4111) HIV-negative
4112) HIV-positive
4113) acidimetry (+ic)
4114) acidimetrical
4115) acidimetrically
4116) conjunctional
4117) amphibologies (added plural)
4118) statementing + statemented (Kevin Atkinson wordlist 60)
4119) stationmasters (added plural - Kevin Atkinson wordlist 60)
4120) Lankan (+plural - Sri Lanka)
4121) sundress (+plural - Kevin Atkinson wordlist 60)
4122) sundrops
4123) suntrap (+plural - Kevin Atkinson wordlist 60)
4124) summat (Kevin Atkinson wordlist 60)
4125) suppertime (Collins - Kevin Atkinson wordlist 60)
4126) supremos (added plural troppy
4127) confederal
4128) collectorate
4129) centrism
4130) strikebound (Collins - Kevin Atkinson wordlist 60)
4131) sweatsuit (+plural - Kevin Atkinson wordlist 60)
4132) succubi (plural of succubus - Kevin Atkinson wordlist 60)
4133) stripey (another way of stripy - Kevin Atkinson wordlist 60)
4134) stripier
4135) stripiest
4136) strivers (added plural)
4137) lamish (Collins)
4138) spambot (+plural - Collins)
4139) spammie (Collins)
4140) pregnable
4141) preglacial
4142) pregnability (Collins)
4143) pregrowth (Collins)
4144) preguide (Collins)
4145) prehension
4146) prehistorian (+plural)
4147) prehuman (+plural)
4148) brainbox (+plural - Oxford: informal)
4149) troppy (+ier +iest +ly +ness - Kevin Atkinson wordlist 60)
4150) strophanthin
4151) analeptic (+plural)
4152) meperidine
4153) prolepsis
4154) prolepses (plural of previous word)
4155) proleptic
4156) propertyless (Collins)
4157) Avernus (lake in Italy)
4158) stunners (added plural - Kevin Atkinson wordlist 60)
4159) stunsail (+plural)
4160) tarty (+ier +iest +ly +ness)
4161) tarweed (+plural)
4162) forb (+plural)
4163) betatrons (added plural)
4164) thermopiles (added plural)
4165) chromakey
4166) tatami (+s +'s - Kevin Atkinson wordlist 60)
4167) clapperboard
4168) telesales (Kevin Atkinson wordlist 60)
4169) telerecordings (added plural)
4170) telework
4171) televisually (merged into televisual)
4172) keypal (+plural - Oxford: informal)
4173) penfriend (+plural)
4174) twinset (+plural - Kevin Atkinson wordlist 60)
4175) zookeeper (+plural +'s - Kevin Atkinson wordlist 60)
4176) zloties (added plural - Kevin Atkinson wordlist 60)
4177) zlotys (another plural of previous word)
4178) zine (+plural - Kevin Atkinson wordlist 60)
4179) ya (Kevin Atkinson wordlist 60)
4180) Y2K (year 2000)
4181) clickable
4182) clickbait
4183) wordsmith (+plural - Kevin Atkinson wordlist 60)
4184) vapourware
4185) overhype (+ed)
4186) bibliolater (+plural)
4187) bibliomancy
4188) Ching (I Ching)
4189) ching
4190) bibliomania
4191) bibliomaniac (+'s)
4192) bibliophilic
4193) bibliophily
4194) bibliopole (+plural)
4195) bibliometric (+plural)
4196) bibliomane (+plural)
4197) wing-like
4198) downstroke
4199) downswing (+plural)
4200) waybill (+s +'s - Kevin Atkinson wordlist 60)
4201) watchable (Kevin Atkinson wordlist 60)
4202) watchability
4203) weedkiller (+plural - Kevin Atkinson wordlist 60)
4204) weedicide (+plural)
4205) weedgrown
4206) weebill (+plural)
4207) apostlebird (+plural)
4208) currawong (+plural)
4209) weeder (+plural)
4210) weedless (merged into weed - Kevin Atkinson wordlist 60)
4211) warm-blooded (+ness)
4212) warmblood (+plural)
4213) vulturine
4214) vulturish
4215) vulturous (Kevin Atkinson wordlist 60)
4216) boobook (+plural)
4217) blackpoll (+plural)
4218) watchstrap (+plural - Collins - Kevin Atkinson wordlist 60)
4219) voicelessness (merged into voiceless - Kevin Atkinson wordlist 60)
4220) vigesimal (Kevin Atkinson wordlist 60)
4221) centesimal (+ly)
4222) vincible (Kevin Atkinson wordlist 60)
4223) vincibility
4224) urogenital (Kevin Atkinson wordlist 60)
4225) urography
4226) urogram (+plural)
4227) urodynamic (+plural)
4228) varix
4229) varices (plural of previous word)
4230) endocarditis
4231) endocarditic
4232) unsmart (Collins)


V2.21 - 1-FEB-2015
------------------
4233) Crowley's (added the 's)
4234) recursivity (+plural - Wiktionary)
4235) recursiveness (Wiktionary)
4236) she-male (+plural)
4237) pseudocode (+plural +ed - Wiktionary)
4238) pseudocopulation
4239) scumbag (+plural - Oxford: informal)
4240) heterosocial (Collins)
4241) heterosociality (Collins)
4242) homosocial (Collins)
4243) homosociality (Collins)
4244) homosexualist (+plural - Collins)
4245) homosex (Collins)
4246) heteropteran (+plural)
4247) heteropterous
4248) limewash (+plural +ing +ed)
4249) limescale
4250) BMWs (added plural)
4251) limepit
4252) Robben (name of island)
4253) Bornholm (name of island)
4254) born-free (+plural)
4255) Bornean
4256) quoll (+plural)
4257) monkery (Oxford:  derogatory)
4258) zealotry
4259) deseed (+ed)
4260) deseeder (+plural)
4261) terotechnology
4262) terotechnological
4263) terotechnologist (+plural)
4264) microtechnology
4265) microtechnological
4266) waveguide (+plural)
4267) Adidas (+'s - name)
4268) geotechnology (Collins)
4269) femtosecond (+plural)
4270) fs (abbreviation of previous word)
4271) femtocell
4272) unfought (Collins)
4273) unforked (Collins)
4274) unforgivingness (Collins)
4275) refigure (Collins - +plural +ing +ed - Wiktionary)
4276) refire (Collins - +plural +ing +ed - Wiktionary)
4277) dentary (+plural)
4278) readd (Collins - +plural +ing +ed - Wiktionary)
4279) rebottle (Collins - +plural +ing +ed +er +ers - Wiktionary)
4280) hypertrophic (merged into hypertrophy)
4281) Barbie (+plural +'s - name)
4282) guidable
4283) cutscene (+plural)
4284) whatsit (+plural - Kevin Atkinson wordlist 60)
4285) whee (Kevin Atkinson wordlist 60)
4286) horseshed
4287) Fenimore (+'s - name)
4288) coopery (+plural)
4289) peridot (+plural)
4290) peridium
4291) peridia (plural of previous word)
4292) galea
4293) galeae (plural of previous word)
4294) low-carb (Oxford: informal)
4295) low-class
4296) yonks (Kevin Atkinson wordlist 60)
4297) Zinfandel (+'s +plural)
4298) zonal (+ly)
4299) rack-and-pinion
4300) fishplate (+plural - fixed: fish-plate)
4301) bodyshell (+plural)
4302) switchable (merged into switch - Kevin Atkinson wordlist 40)
4303) postdoc (+plural - Kevin Atkinson wordlist 40)
4304) postcolonialism
4305) postcolonialist
4306) postcoded (merged into postcode)
4307) mailboat (+plural)
4308) Percy's (added the 's)
4309) postwoman
4310) postwomen (plural of previous word)
4311) Boulogne (+'s - name)
4312) synovial
4313) flexion
4314) flection (another way of previous word)
4315) dorsiflex (+plural +ing +ed +ion)
4316) electromyograph
4317) electromyography
4318) chemoprevention (Collins)
4319) Palomar (+'s - name)
4320) serine
4321) bugbane (+plural)
4322) squawroot (+plural)
4323) tidemark (+plural)
4324) tideline (+plural)
4325) saltings (added plural)
4326) geodata
4327) neoclassicist (+plural)
4328) neorealism
4329) neorealist (+plural)
4330) academicism
4331) academism (another way of previous word)
4332) academicals (added plural)
4333) Picasso's (added the 's)
4334) iconology
4335) iconological
4336) iconolatry
4337) deconstruct (+plural +ing +ed +ive)
4338) outsized (merged into outsize)
4339) should've
4340) corporatize (+plural +ed +ing +ion - IZE)
4341) corporatise (+plural +ed +ing +ion - ISE)
4342) hereditament (+plural)
4343) hereditarily + hereditariness (merged into hereditary)
4344) hereditarian (+plural)
4345) hereditarianism
4346) nanotechnological
4347) nanotechnologist (+plural)
4348) neurovascular (Collins)
4349) nanocomposite (+plural)
4350) undersize
4351) undershrub (+plural)
4352) kneeled (another past of kneel)
4353) kneehole
4354) knee-highs (added plural)
4355) cerebroside (+plural)
4356) workup (+plural)
4357) jackstay
4358) Tobruk (+'s - name of place)
4359) woosh (+plural +ing +ed - another way of whoosh)
4360) Jacinto (+'s - name)
4361) starboarding (merged into starboard)
4362) bioprospecting
4363) bioprospector (+plural)
4364) Kamchatka (+'s - name)
4365) flamethrowers (added plural)
4366) laserdiscs (added plural)
4367) Betacam (trademark)
4368) Betamax (trademark)
4369) PlayStation (+plural)
4370) breakfront
4371) Easton (+'s - name)
4372) geochronological
4373) geochronologist (+plural)
4374) declivitous
4375) bluntish (Collins)
4376) subchapter (+plural - Collins)
4377) cosupervisor (+plural - Wiktionary)
4378) supervoltage (+plural)
4379) cosupervision (Wiktionary)
4380) cosupervise (+plural +ing +ed - Wiktionary)
4381) resect (+ed +able)
4382) resectional
4383) resectionist
4384) microbiologically (Collins)
4385) microbiologic (Collins)
4386) Cmd. (abbreviation - UK - Command Paper - 4th series)
4387) Cmnd. (abbreviation - UK - Command Paper - 5th series) 
4388) handgrip (+plural)
4389) cyanoacrylate (+plural)
4390) photostimulation (+plural - Wiktionary)
4391) chromophore (+plural)
4392) chromophoric
4393) chromoplast (+plural)
4394) exoatmospheric
4395) exobiological
4396) exobiologist (+plural)
4397) ethnolinguistics
4398) ethnolinguist (+plural)
4399) ethnohistory
4400) ethnohistorian (+plural)
4401) ethnohistoric
4402) ethnohistorical
4403) ethnoscience
4404) ethogram (+plural)
4405) twinflower (+plural)
4406) townland (+plural)
4407) chronometry
4408) chronometrical
4409) chronometrically
4410) chronotherapy
4411) chronostratigraphy (+ic)
4412) substage
4413) subspecific
4414) subsonically
4415) deflagration
4416) ketones (added plural)
4417) ketonic
4418) ferrocene (+plural)
4419) ferroconcrete
4420) ferroelectrics (added plural)
4421) ferroelectricity
4422) ferromagnetism
4423) Zeeman (Zeeman effect)
4424) atomicities (added plural)
4425) multiagent (+plural - Wiktionary) 
4426) non-typical
4427) non-transparency
4428) non-U (Oxford: informal)
4429) non-urban
4430) non-use
4431) non-validity
4432) non-variable
4433) non-vintage
4434) non-word (added singular)
4435) irregardless (Oxford: informal)
4436) irrefutability
4437) refutability (Collins)
4438) regalism
4439) regalist (+plural)
4440) canonry (+plural)
4441) Bastogne (name of place)
4442) Junagarh (name of place)
4443) flaunching
4444) leadwork (+plural - Collins)
4445) flat-woven
4446) flat-weave
4447) dhurrie (+plural)
4448) durrie (+plural - another way of previous word)
4449) awardee (+plural)
4450) Waterville (name of place)
4451) award-winning
4452) traumatization (-IZE)
4453) traumatisation (-ISE)
4454) traumatism
4455) multicentre (Collins)
4456) multicentral (Collins)
4457) crossmatch (+plural +ing +ed)
4458) posttransfusion (Collins)
4459) eponymously (merged into eponymous)
4460) Lanchester (+'s - name)
4461) routinism
4462) routinist (+plural)
4463) routineness (merged into routine)
4464) streamlet (+plural)
4465) Zealander (added singular)
4466) psychosurgery
4467) psychosurgical
4468) radiochemist (+plural)
4469) bohrium
4470) permeance (+plural)
4471) electropositive
4472) electronegativity (+plural)
4473) electrophilic
4474) electrophile (+plural)
4475) nucleonic (+plural)
4476) nucleophilic
4477) nucleophile (+plural)
4478) nucleoplasm
4479) nucleoprotein (+plural)
4480) nucleosome (+plural)
4481) nucleosomal
4482) overprescribe (+s +ing +ed +ion)
4483) titanically
4484) Cicciolina (+'s - name)
4485) pornocracy (Collins)
4486) voguishly + voguishness (merged into voguish - Collins)
4487) prenominal (+ly - Collins)
4488) curricula vitae (removed since it was two words)
4489) curriculum vitae (removed since it was two words)
4490) curriculums (another plural way of curriculum)
4491) gradience
4492) ethnomethodological
4493) ethnomethodologist (+plural)
4494) avunculate
4495) queendom
4496) queenless
4497) queenlike
4498) queenship
4499) filamented (merged into filament)
4500) microelectrode (+plural - Collins)
4501) astrometry (+ic)
4502) Gaia (+'s - name)
4503) Gaian (+plural)
4504) ethicality
4505) primitivist (+plural)
4506) supervisee (+plural - Collins)
4507) re-enacts + re-enacted + re-enacting (merged into re-enact)
4508) re-enactment (+plural)
4509) re-enactor (+plural)
4510) handlist (+plural)
4511) Barack (name)
4512) McKinsey (+'s - name)
4513) NGOs (added plural)
4514) Croatians (added plural)
4515) Armenians (added plural)
4516) Assyrians (added plural)
4517) Encaenia
4518) succulently (merged into succulent)
4519) imbibition
4520) shadowland (+plural)
4521) shadowgraphs (added plural)
4522) radiographs (added plural)
4523) radiographed (added ed)
4524) radiographically
4525) radioimmunoassay
4526) radioisotopic
4527) andrology
4528) andrologist
4529) areology
4530) areological
4531) areologist (+plural)
4532) areole (+plural)
4533) glochid (+plural)
4534) discommodious
4535) discommodity
4536) sassy (+ier +iest +ly +ness - Oxford: informal)
4537) WWI (World War I)
4538) NYC (New York City)
4539) bushbuck (+plural)
4540) waterbuck (+plural)
4541) bonobo (+plural)
4542) capuchin (+plural)
4543) Capuchin (+plural - upper case - Franciscan order)
4544) Pio (+'s - name)
4545) stigmatist
4546) Francesco's (added the 's)
4547) sublanguage (+plural)
4548) sublateral
4549) subkingdom (+plural)
4550) Milton's (added the 's)
4551) subjunctively
4552) sublate (+plural +ed +ing +ion)
4553) egomaniacal
4554) egosurf (+plural +ed +ing - Oxford: informal)
4555) Baikal (+'s - name of lake)
4556) Baykal (+'s - another way of the previous word)
4557) Bresenham (+'s - name)
4558) milfoil (+plural)
4559) allamanda (+plural)
4560) plumeria (+plural)
4561) voiceful
4562) voice-activated
4563) glassmaking (fixed: glass-making)
4564) glassmaker (+plural)
4565) glasspaper (fixed: glass-paper)
4566) fiddleback
4567) saddleback (+plural +ed)
4568) birdlife (Collins)
4569) xix + XIX (Roman numeral - 19)
4570) Cyanobacteria (upper case)
4571) cyanobacteria (lower case)
4572) cyanobacterial
4573) stromatolite (+plural)
4574) apeman
4575) apemen (plural of previous word)
4576) coelurosaur (+plural)
4577) integumental
4578) integumentary
4579) reconstructable
4580) reconstructive
4581) reconstructor
4582) Egyptological
4583) Egyptologist (+plural)
4584) Tutankhamun (another way of Tutankhamen)
4585) centrefire
4586) varnisher (+plural)
4587) sealers (added plural)
4588) photocomposition
4589) photocurrent
4590) photodegradable
4591) lookism
4592) lookist
4593) essentialism
4594) Jesuitical (+ly)
4595) south-west (fixed: south-West)
4596) south-east (fixed: south-East)
4597) north-east (fixed: north-East)
4598) north-eastern (fixed: north-Eastern)
4599) north-west (fixed: north-West)
4600) north-western (fixed: north-Western)
4601) south-eastern (fixed: south-Eastern)
4602) southwardly (merged into southward)
4603) fatling
4604) Gatlings (added plural - Gatling gun)
4605) stowable (Collins)
4606) outlands (added plural)
4607) photochromic
4608) photochromism
4609) viologen (+plural)
4610) photo-oxidation
4611) irradiance
4612) anti-magnetic
4613) tweeze (+plural +ed +ing)
4614) tweezer (added singular)
4615) post-date (+plural +ed +ing - fixed: postdate)
4616) hybridity
4617) Ludwig's (added the 's)
4618) rotifers (added plural)
4619) multicell (+ed - Collins)
4620) telson (+plural)
4621) setiferous (Collins)
4622) setigerous (Collins - another way of previous word)
4623) Telstar (+'s - name of satellite)
4624) Ishtar (+'s - name)
4625) Qataris (added plural)
4626) VM (+plural - Virtual Machine - Wiktionary)
4627) V-mail
4628) ATMs (added plural)
4629) automagically (Oxford: informal)
4630) autologous
4631) decompensation
4632) decompensated
4633) hepatoma (+plural)
4634) hepatomata (another plural of previous word)
4635) hepatopancreas
4636) mouthpart (+plural)
4637) foregut (+plural)
4638) vitellogenin
4639) xenobiotic (+plural)
4640) recompilable (Wiktionary)
4641) compilatory (Collins)
4642) recompact (+plural +ing +ed - Wiktionary)
4643) GUIs (added plural - Wiktionary)
4644) burnoose (removed as it was American)
4645) burnous (British way of previous word)
4646) buttonhook (+plural)
4647) downfield
4648) Bukowski (+'s - name)
4649) Xbox (+plural - video game console - Wiktionary)
4650) Matroska
4651) FLAC (+'s)
4652) HEVC (+'s)
4653) MP3s (added plural)
4654) WAV (upper case)
4655) wav (lower case)
4656) terascale (Wiktionary)
4657) teraflop (+plural)
4658) electronvolt (+plural)
4659) eV (abbreviation of previous word)
4660) electro-oculogram
4661) electro-optic (+plural)
4662) electro-optical
4663) agarose
4664) MBA's + MBAs (merged into MBA)
4665) daywork (+er +ers)
4666) parasitize (+plural +ed +ing +ion - IZE)
4667) parasitize (+plural +ed +ing +ion - ISE)
4668) Rothstein (+'s - name)
4669) greenling (+plural)
4670) greenmailer (+plural)
4671) Garner (+'s - name)
4672) Greenaway (+'s - name)
4673) Pietro (name)
4674) annexationist (+plural)
4675) Zululand (name of place)
4676) KwaZulu-Natal (name of place)
4677) Indo-European
4678) Proto-Indo-European
4679) Indic
4680) indican
4681) bilirubin
4682) perimenopause
4683) perimenopausal
4684) undescended
4685) anaphylaxis
4686) telemedicine
4687) psychophysiological
4688) psychodynamic (+plural)
4689) psychodynamically
4690) antipyretic (+plural)
4691) hyperthermia
4692) hyperthermic
4693) bootloaders (added plural - Wiktionary)
4694) UEFI (Wiktionary)
4695) EFI
4696) sportsperson (+plural)
4697) sportspeople (another plural of previous word)
4698) sportswomanship
4699) sportswriter (+plural)
4700) doubletons (added plural)
4701) dumortierite
4702) hydroxyapatite
4703) hydrometric
4704) winemaking
4705) streamflow (+plural)
4706) snowpack (+plural)
4707) snowmobiling + snowmobiled + er + ers (merged into snowmobile) 
4708) groomer (+plural - merged into groom)
4709) snowcat (+plural)
4710) heli-ski (+ing +er +ers)
4711) vrooming (merged into vroom)
4712) Vince's (added the 's)
4713) Neale (name)
4714) frequentative
4715) benefactive (+plural)
4716) grammaticalize (+ed +ion - IZE)
4717) grammaticalise (+ed +ion - ISE)
4718) Walkman (+plural)
4719) Walkmen (another plural of previous word)
4720) microchipping + microchipped (merged into microchip)
4721) woodchip (+plural)
4722) Collingwood (+'s - name)
4723) pavers (added plural)
4724) teahouse (+plural)
4725) Teagarden (+'s - name)
4726) contradance (+plural)
4727) coffee-houses (removed - Oxford: it has no hyphen, two words)
4728) Megan (+'s - name)
4829) megamouth (+'s)
4830) megaphonic (merged into megaphone)
4831) Babygro (+plural - trademark)
4832) babyishly + babyishness (merged into babyish)
4833) girly (another way of girlie + plural)
4834) malaguetta (+plural)
4835) malagueta (+plural - another way of the previous word)
4836) metasomatism
4837) metasomatic
4838) metasomatize (+ed - IZE)
4839) metasomatize (+ed - ISE)
4840) emplaced (merged into emplace)
4841) pluton (+plural)
4842) plutocratically
4943) backport (+plural +ing +ed - Wiktionary)
4944) demoscene (Collins)
4945) autosuggestion
4946) completist (+plural)
4947) autostereoscopy
4948) autosaves (merged into  autosave)
4949) haplotype (+plural)
4950) cardio (Oxford: mass noun informal)
4951) goalball
4952) Paralympic (+plural)
4953) Paralympian (+plural)
4954) goalhanger (+plural)
4955) Kristine (name)
4956) Hawaiian's (added the 's)
4957) pseudosciences (added plural)
4958) pseudoscientists (added plural)
4959) Scientologist (+plural)
4960) symbology
4961) hagiology
4962) hagiological
4963) hagiologist (+plural)
4964) demonize (+plural +ed +ing - IZE)
4965) demonise (+plural +ed +ing - ISE)
4966) demonization ( - IZE)
4967) demonisation ( - ISE)
4968) infographics (added plural)
4969) infolded
4970) infomania
4971) infomaniac (+plural)
4972) infomediaries (added plural)
4973) infomercial (removed since it is chiefly American)
4974) infonauts (added plural)
4975) Rhode's (added the 's)
4976) subplot (fixed: sub-plot)
4977) subparallel
4978) subphylum
4979) Eutheria
4980) eutherian (+plural)
4981) lamella
4982) lamellae (plural of previous word)
4983) lamellar
4984) lamellate
4985) lamelliform
4986) lamellose
4987) pHs (added plural)
4988) cuvette (+plural)
4989) acetylcholinesterase
4990) cryoglobulin (+plural)
4991) glucoside (+plural +ic)
4992) geomatic (+plural)
4993) palaeomagnetism
4994) palaeomagnetic
4995) declinism
4996) Pitchford (+'s - name)
4997) footplates (added plural)
4998) diesel-electrics (added plural)
4999) batfish (+plural)
5000) bitterling (+plural)
5001) exoskeletal
5002) exfoliate (+plural +ing +ed +ion)
5003) exfoliative
5004) exfoliator (+plural)
5005) anticellulite (Collins)
5006) endoskeleton (+plural)
5007) endoskeletal
5008) endoscopically
5009) endoscopist (+plural)
5010) gastroscope (+plural +ic)
5011) gastroscopy
5012) bronchoscope (+plural)
5013) bronchoscopy
5014) pneumothorax
5015) reinflate (+plural +ed +ing +ion)
5016) reinflatable
5017) reinforcers (added plural)
5018) rebar
5019) positioner (+plural)
5020) declarant (+plural +'s)
5021) transexual (+plural +'s - another way of transsexual)
5022) noncultivated (Collins)
5023) Rackham (+'s - name)
5024) summiteer (+plural)
5025) orienteer (+plural +ed +ing)
5026) backcrawl
5027) dilophosaurus
5028) earthstar
5029) earthshine
5030) earthlight
5031) earthslip (+plural)
5032) multiplicate (Collins)
5033) multipartite
5034) metazoan (+plural)
5035) sexpartite
5036) partite
5037) slaveholder (+plural)
5038) slaveholding (+plural)
5039) genuflector (+plural)
5040) stomper (+plural)
5041) stompy (+ier +iest)
5042) synth (+plural - Oxford: informal)
5043) synthy
5044) goth (added lower case)
5045) emo (+plural)
5046) emocore (another way of the previous word)
5047) deflexed
5048) inflexed
5049) hypoallergenic (fixed: hypo-allergenic)
5050) cosmeceutical (+plural)
5051) glycolic
5052) glycollic
5053) retinol
5054) Strawson (+'s - name)
5055) squillion (+plural - Oxford: informal)
5056) squillionaire (+plural - Oxford: informal)
5057) squinancywort
5058) greenweed
5059) candytuft (+plural)
5060) coralberry (+plural)
5061) narcissuses (plural of narcissus)
5062) narcissi (another plural of narcissus)
5063) narcolepsy
5064) narcoleptics (added plural)
5065) hyperreal
5066) hyperrealism
5067) hyperrealist
5068) hyperrealistic
5069) hyperreality
5070) phantasmagorical
5071) stum (+plural +ing +ed)
5072) washtub (+plural)
5073) twin-tub (+plural)
5074) twirly (+ier +iest)
5075) gumshield (+plural)
5076) Flagler (+'s - name)
5077) flagitious (+ly +ness)
5078) suppositious (+ly +ness)
5079) confessant (+'s +plural)
5080) skilfish (+plural)
5081) skijoring
5082) skijorer (+plural)
5083) clingstone (+plural)
5084) clingfish (+plural)
5085) beachy (+ier +iest +ness)
5086) creamware
5087) objectivism
5088) objectivist (+plural)
5089) objectivistic
5090) Bayes (+'s - name)
5091) Jaynes (+'s - name)
5092) deerhound (+plural)
5093) saluki (+plural)
5094) Ibizan (Ibizan hound)
5095) Tartarus (+'s - name)
5096) Tartarean
5097) Ixion (+'s - name)
5098) Hera (+'s - name)
5099) Cronus (+'s - name)
5100) Kronos (+'s - name)
5101) Gaea (+'s - name - another way of Gaia)
5102) busway (+plural)
5103) Jakarta's (added the 's)
5104) premiere (+plural +ed +ing)
5105) Pusan (name)
5106) bedsock (+plural)
5107) copywriting
5108) signwriter (+plural)
5109) signwriting
5110) vinyls (added plural)
5111) housepainter (+plural - Collins)
5112) blowbacks (added plural)
5113) unendingness (merged into unending)
5114) trackway (+plural)
5115) souterrain (+plural)
5116) Epipalaeolithic
5117) epiphenomenal
5118) epinician
5119) unendowed
5120) unenergetic
5121) unengaged
5122) unenjoyable
5123) un-English
5124) unenlightenment
5125) ectoderm (+al)
5126) endoderm (+al)
5127) tetrapods (added plural)
5128) spastically
5129) spasticity
5130) overactive (fixed: over-active)
5131) overactivity (fixed: over-activity)
5132) Kayla (added 's - name)
5133) holier-than-thou
5134) Dalai (name: Dalai Lama)
5135) Hornblower's (added the 's - name)
5136) holystoning + holystoned (merged into holystone)
5137) hatemonger (+plural)
5138) POWs (added plural - Wiktionary)
5139) Euclid's (added 's - name)
5140) multiparity (Collins)
5141) multiparous
5142) aeroshell
5143) headliner (+plural - merged into headline)
5144) indie (+plural)
5145) metalhead (+plural - Oxford: informal)
5146) headbanger's (added the 's)
5147) popster (+'s +plural - Oxford: informal)
5148) frontman
5149) frontmen (plural of previous word)
5150) folkie (+'s +plural - Oxford: informal)
5151) melodicism
5152) Seeger (+'s - name)
5153) nonissue (Collins)
5154) Junoesque
5155) humoresque (+plural)
5156) Sinbad (+'s - another way of Sindbad)
5157) Dillingham (name)
5158) Gordonstoun (+'s - name)
5159) Junr (abbreviation in names)
5160) juneberry (+plural)
5161) liana (+plural)
5162) liane (+plural - another way of the previous word)
5163) antifeedant (+plural)
5164) bioassay (+plural)
5165) UCLA (+'s - University of California in Los Angeles)
5166) physico-chemical
5167) extropy
5168) extropian (+plural)
5169) transhumanism
5170) transhumanist
5171) Fukuyama (name)
5172) world-view (removed: it is two words in Oxford)
5173) balk (+plural +ed)
5174) Maddock (+'s - name)
5175) transhistorical
5176) alchemic (merged into alchemy)
5177) alchemize (+s +ed +ing - IZE)
5178) alchemise (+s +ed +ing - ISE)
5179) ultrafast (Collins)
5180) ultrafastidious (Collins)
5181) ultrafiltration
5182) ultradian
5183) ultracritical
5184) fermionic (Collins)
5185) ultracentrifuged (merged into ultracentrifuge)
5186) ultracentrifugal
5187) ultracentrifugation
5188) overthrust (+ing)
5189) fluorometer (+plural)
5190) fluorometric
5191) fluorometrically
5192) fluorometry
5193) misconstruction (+plural)
5194) construction's (fix: removed it - it had dictionary codes)
5195) intersubjective (+ly)
5196) intersubjectivity
5197) intertextuality
5198) intertextual
5199) intertextually
5200) dialogic
5201) dialogical
5202) sourcebook (+plural)
5203) Matilda's (added the 's - name)
5204) workshopping + workshopped (merged into workshop)
5205) Jonah's (added the 's - name)
5206) monologic
5207) monological
5208) monologize (+plural +ing +ed - IZE)
5209) monologise (+plural +ing +ed - ISE)
5210) worksite (+plural)
5211) tenace (+plural)
5212) breadstick (+plural)
5213) breadwinning
5214) SWOT (SWOT analysis)
5215) shapewear
5216) trigraph (+plural)
5217) adverbials (added plural)
5218) j (letter)
5219) w (letter)
5220) graphemes (added plural)
5221) graphemic (+plural)
5222) graphemically
5223) misperceiving
5224) misperception (+plural)
5225) enlace (+ed)
5226) autochromes (added plural)
5227) microaerophilic
5228) microaerophile (+plural)
5229) recombinants (added plural)
5230) unswollen (Collins)
5231) DNase
5232) RNase
5233) resuspend (+plural +ed +ing +ion)
5234) triturate (+plural +ed +ing +ion)
5235) triturator (+plural)
5236) triumphally (merged into triumphal)
5237) triumphalists (added plural)
5238) zander
5239) marshalship
5240) wing-tip (fixed: removed since in Oxford it is two words)
5241) wingsuit
5242) airboats (added plural)
5243) AdWords (Google AdWords)
5244) ECTS (European Credit Transfer Scheme)
5245) plug-compatible
5246) plugboard (+plural)
5247) autosuggestion (fixed: auto-suggestion)
5248) autotune
5249) JPG (upper case)
5250) jpg (lower case)
5251) autoxidized (merged into autoxidize - IZE)
5252) autoxidised (merged into autoxidise - ISE)
5253) Transylvanian
5254) transwoman
5255) transwomen (plural of previous word)
5256) appropriacy (+plural)
5257) Wiktionary
5258) auditorial
5259) Deems (+'s - name)
5260) deemster (+plural)
5261) houseroom
5262) abdominoplasty (+plural)
5263) mammaplasty (+plural - Collins)
5264) mammoplasty (+plural - Collins - another way of previous word)
5265) extracorporeal
5266) lithotripsy
5267) lithotripter (+plural)
5268) lithotriptor (+plural - another way of previous word)
5269) lithotriptic
5270) facelifts (added plural)
5271) injectables (added plural)
5272) Botox (+ed - trademark)
5273) antipruritic (+plural)
5274) nonsteroidal (Collins)
5275) nonaspirin (Collins)
5276) antitussive
5277) premedication
5278) premaxillary
5279) prematch
5280) premaster (+plural +ing +ed)
5281) prelusive
5282) prem (+plural)
5283) hypothermal
5284) hypothermic
5285) zoophobia
5286) bacteriophobia
5287) cyberphobic's (added the ´s)
5288) cyberphobes (added plural)
5290) computerphobia (Collins)
5291) realpolitik (fix: it was in upper case)
5292) Talleyrand (+'s - name)
5293) fuehrer (+plural +'s - another way for führer)
5294) faunal
5295) faunistic
5296) fraternalism (+'s)
5297) pledgee
5298) Kelk (name)
5299) aerosphere
5300) aesthetician (+plural +'s)
5301) badware
5302) von (now in lower case - fixed: Von)
5303) baculovirus (+plural)
5304) baculum
5305) bacula (plural of previous word)
5306) ctenoid
5307) aviculture
5308) avicultural
5309) aviculturalist (+plural)
5310) aviculturist (+plural)
5311) cubital
5312) cubitus
5313) kilobase (+plural)
5314) hydrogel (+plural)
5315) hydrofracturing
5316) hydrofluorocarbon (+plural)
5317) HFC (+plural - abbreviation of previous word)
5318) irrigator (+plural)
5319) megalitres (added plural)
5320) tympanic
5321) petrosal (+plural)
5322) semilunar
5323) semi-literate
5324) semi-literacy
5325) Malayalam
5326) MiB (Wikipedia)
5327) KiB (Wikipedia)
5328) GiB (Wikipedia)
5329) TiB (Wikipedia)
5330) PiB (Wikipedia)
5331) EiB (Wikipedia)
5332) ZiB (Wikipedia)
5333) YiB (Wikipedia)
5334) unsharp + unsharpness
5335) draughtproof (+ing)
5336) testiculate
5337) eel-like
5338) eely
5339) underprepared
5340) underpopulation
5341) midtowns + midtown's (removed: American)
5342) teriyaki
5343) butternut (+plural)
5344) movie (+plural - removed: chiefly American)
5345) groovily + grooviness (merged into groovy)
5346) dharma
5347) cherubically
5348) fifty-fifty
5349) figgy (+ier +iest)
5350) fightbacks (added plural)
5351) Tynecastle (+'s - name)
5352) fighter-bomber (+plural)
5353) frontlet (+plural)
5354) depigment (+ed)
5355) depigmentation
5356) vitiligo
5357) leucoderma
5358) catenative (+plural)
5359) non-finite
5360) prepose (+s +ing +ed)
5361) backshift
5362) unsurveyed
5363) subpopulation (Collins)
5364) unsusceptible
5365) unsusceptibility
5366) Westbury (+'s - name)
5367) theming (merged into theme)
5368) urbanist (+plural)
5369) inliers (added plural)
5370) Cimmerian (+plural +'s)
5371) orogeny (+plural)
5372) orogenesis
5373) orogenic
5374) orogen (+plural)
5375) ultrastructure (Collins)
5376) ultrastructural (Collins)
5377) Proterozoic
5378) Precambrian
5379) Archaean
5380) Priscoan
5381) prismatically
5382) starburst (+plural)
5383) blocky (+ier +iest)
5384) aragonitic (Collins)
5385) unsprayed
5386) unstabler + unstablest + unstableness (merged into unstable)
5387) microarchitecture (+plural - Wiktionary)
5388) Celeron (Intel)
5389) i3 (Intel)
5390) i5 (Intel)
5391) i7 (Intel)
5392) LGA (Intel)
5393) Broadwell (+'s - name)
5394) Haswell (+'s - name)
5395) Krzanich (+'s - name)
5396) Skylake (+'s - name)
5397) transcriptome (+plural)
5398) cybrid (Collins)
5399) shoaled + shoaling (merged into shoal)
5400) boneyard (Oxford: informal)
5401) nuthouse (Oxford: informal)
5402) hypochondriacal
5403) pedicular (Collins)
5404) immunocompromised
5405) humoral
5406) bloodlust (fixed: blood-lust)
5407) upthrust (+s +ing +ed)
5408) bloodletting (fixed: blood-letting)
5409) Elaine's (added the 's - name)
5410) alphafetoprotein
5411) unconjugated (Collins)
5412) immunohistochemistry (Collins)
5413) orthotic (+plural)
5414) orthotist (+plural)
5415) POP3
5416) POP
5417) RFC
5418) brainwork
5419) brainily (merged into brainy)
5420) caseloads (added plural)
5421) gromwell (+plural)
5422) odiferous (another way of odoriferous)
5423) Anubis (+'s - name)
5424) cupholders (added plural)
5425) telecourse (+plural - Collins)
5426) baserunner (+plural - Collins)
5427) angiographer (+plural)
5428) angiographic
5429) angiographically
5430) angioneurotic
5431) angiotensin
5432) hydrocortisone
5433) glucocorticoid (+plural)
5434) agonists (added plural)
5435) univalents (added plural)
5436) nonhomologous (Collins)
5437) Hansard (+plural +'s)
5438) denotative
5439) reverberantly
5440) reverberative
5441) reverberator
5442) reverberatory
5443) filmic
5444) film-going
5445) fade-up
5446) fade-out (+plural)
5447) fade-in (+plural)
5448) docusoap (+plural)
5449) film-maker's (added the 's)
5450) featurette (+plural)
5451) chromakeys + chromakeying + chromakeyed (merged into chromakey)
5452) beleaguerment
5453) transport's (added the 's)
5454) adversarially (merged into adversarial)
5455) adverbials (added plural)
5456) commissarial
5457) fantasist
5458) incubi (plural of incubus)
5459) linguistician (+plural)
5460) metamathematics
5461) metamathematician (+plural)
5462) Las (Las Vegas)
5463) dimmable
5464) dimmish
4565) concious (fix: removed, it doesn't exist)
4566) conciousness (fix: removed, it doesn't exist)
4567) subdiscipline (+plural - Collins)
4568) subfield (+plural - Collins)
4569) subduct (+plural +ing +ed)
4570) subeditorial (fixed: sub-editorial)
4571) subeditor (+plural +'s - fixed: sub-editor)
4572) subeditorship (Collins)
4573) netsplit (+plural +ing +ed - IRC - Wiktionary)
4574) biennale (+plural)
4575) documentalist's (added the 's)
4576) ornamentalism
4577) ornamentalist (+plural)
4578) dextrous (+ly +ness - another way of dexterous)
4579) dextral
4580) dextrality
4581) dextrally
4582) apertural
4583) regionalize (+ed +ion -IZE)
4584) regionalise (+ed +ion -ISE)
4585) methodic
4586) lodgepole (lodgepole pine)
4587) patrology
4588) patronal
4589) farouche
4590) barouche (+plural)
4591) Romanic
4592) Vulcanian
4593) Cappadocia
4594) Cappadocian (+plural)
4595) cappellacci
4596) Diego's (added the 's - name)
4597) breadhead (+plural - Oxford: informal)
4598) jackfruit
4599) centuples (added plural)
4600) centrosome (+plural)


V2.22 - 1-MAR-2015
------------------
4601) reupload (+s +ing +ed - Wiktionary)
4602) Purim
4603) purificatory
4604) Hecate (+'s - name)
4605) Selene (+'s - name)
4606) Endymion (+'s - name)
4607) enduro (+plural)
4608) karts (added plural)
4609) karting (merged into kart)
4610) autotest (+plural - Collins)
4611) heathlands (added plural)
4612) euphrasia
4613) sabadilla
4614) smartwatch (+plural)
4615) Engelmann (+'s - name)
4616) blackbutt (+plural)
4617) cordwood
4618) signage
4619) overleveraged
4620) DVR (+plural)
4621) newsfeeds (added plural)
4622) aggregator (+plural)
4623) cladogram (+plural)
4624) cladogenesis
4625) cladogenetic
4626) co-evolution
4627) co-evolutionary
4628) co-evolve (+s +ing +ed)
4629) countertransference
4630) buttercream
4631) candyfloss
4632) eco-warrior (+plural)
4633) phenology
4634) phenological
4635) triglycerides (added plural)
4636) squalene
4637) glycerides (added plural)
4638) hydrocephalic
4639) hydrocephaly
4640) sonography + sonograph + sonographic
4641) Matangi (+'s - name)
4642) synchromesh
4643) diachroneity
4644) diachronically
4645) diachronistic
4646) diachrony
4647) ammonite (added singular)
4648) inserters (added plural)
4649) insertable
4650) turnable (Collins)
4651) mujahideen + mujahedin + mujahidin
4652) semivowel (fixed: semi-vowel)
4653) unmatch (+s +ing +ed - Wiktionary)
4654) unmatchably
4655) uncatchable
4656) Osiris (+'s - name)
4657) Osirian
4658) Horus (+'s - name)
4659) hosannah (another way of hosanna)
4660) taskforce
4661) pisciculture
4662) piscicultural
4663) pisciculturist
4664) Piscean (+plural)
4665) Andromedae
4666) Andromache (+'s - name)
4667) Neoptolemus (+'s - name)
4668) Priam (+'s - name)
4669) Hecuba (+'s - name)
4670) Troilus (+'s - name)
4671) Cressida (+'s - name)
4672) Calchas (+'s - name)
4673) frontwoman
4674) frontwomen (plural of previous word)
4675) kick-turn (+plural)
4676) lipslide (+plural)
4677) fakie (+plural)
4678) unproblematical
4679) unproblematically
4680) progressivist (+plural)
4681) degressive
4682) concessive
4683) bifunctional
4684) functionalistic (Collins)
4685) non-formal
4686) non-fluent
4687) transformative
4688) transformationally
4689) transfixion
4690) transferware
4691) underlease
4692) underinvest (+s +ing +ed)
4693) monogrammatic
4694) sonogram (+plural)
4695) to-dos (added plural - Collins)
4696) tiramisu
4697) cock-a-doodle-doo
4698) cockaded (merged into cockade)
4699) unswayed
4700) home-loving
4701) BFI (abbreviation: British Film Institute)
4702) adland
4703) Adlerian (+plural)
4704) Freudians (added plural)
4705) Freudianism
4706) placeman (added singular)
4707) unobstructive (Collins)
4708) unobserving (Collins)
4709) inconsequentiality (+plural)
4710) inconsequentialness
4711) soundbox
4712) charango (+plural)
4713) roadless (merged into road)
4714)  clearways (added plural)
4715) KDE's (added the 's)
4716) extended-play
4717) EP
4718) shaly (+ier +iest)
4719) shaley (another way of previous word)
4720) outsail (+s +ing +ed)
4721) wiretap (+s +ing +ed)
4722) wiretapper (+plural)
4723) bivalence
4724) quo (status quo)
4725) ROI (abbreviation: Return On Investment)
4726) skyrocket (+s +ing +ed - fixed: sky-rocket)
4727) skysail (+plural)
4728) Brooklyn (+'s - name of city)
4729) snowscape (+plural)
4730) roadmap (removed: Oxford says it is two words)
4731) Citrix (+'s - name of company - Wikipedia)
4732) SEO
4733) relatable
4734) relatability
4735) eco-tax (+plural)
4736) ecotone (+plural)
4737) ecotonal
4738) antiepileptic (Collins)
4739) antianxiety (Collins)
4740) unmedicated (Collins)
4741) non-relative
4742) non-selective
4743) non-combative
4744) e-learning
4745) Aquitaine (+'s - name)
4746) hawklike
4747) waterlog (+s +ing +ed)
4748) Carolingians (added plural)
4749) genotoxic (Collins)
4750) carbon-neutral
4751) biocoenosis
4752) biocoenoses (plural of previous word)
4753) elevational
4754) facies
4755) antialiasing
4756) antifungal (Collins)
4757) antiballistic
4758) antibubble
4759) pharma (+plural)
4760) anovulant (+plural)
4761) calmative (+plural)
4762) tricyclic (+plural)
4763) bioavailability
4764) bioavailable
4765) monocyclic
4766) membered
4767) memberless
4768) leptin
4769) parvovirus (+plural)
4770) leptospirosis
4771) postulatory
4772) posturers (added plural)
4773) kleptocrat (+plural)
4774) kleptocracy (+plural)
4775) kleptocratic
4776) kleptoparasite (+plural)
4776) kleptoparasitic
4777) kleptoparasitism
4778) libitum (ad libitum)
4779) unsized
4780) unrevised
4781) unrevoked
4782) unreversed
4783) unrestored
4784) bling + blingy + blingier + blingiest
4785) bling-bling (another way of previous word)
4786) burkas (added plural)
4787) burkha (+plural - another way of previous word)
4788) burqa (+plural - another way of previous word)
4789) chadors (added plural)
4790) headscarves (added plural)
4791) crowdfund (+plural)
4792) deleverage (+s +ed +ing - Wiktionary)
4793) delexical
4794) earbud (+plural)
4795) earbash (+ing - Oxford: informal)
4796) emoji (+plural)
4797) geocache (+s +ing +ed +er)
4798) googles + googling (merged into google)
4799) googleable
4800) googlable (another way of previous word)
4801) weblog (+plural + er)
4802) underwire (+s +ed)
4803) pat-down
4804) Islamophobia
4805) Islamophobe
4806) Islamophobic
4807) Islamists (added plural)
4808) Wikileaks (+'s - Wikipedia)
4809) Oxfordian (+plural)
4810) unfermented
4811) unrented (Collins)
4812) unparented (Collins)
4813) untormented (Collins)
4814) unaugmented (Collins)
4815) teleplay (+plural)
4816) telepresence
4817) televiewers (added plural)
4818) monocrystalline
4819) sugarbird (+plural)
4820) flockmaster
4821) teleports + teleported (merged into teleport)
4822) gyroscopically
4823) ISS (Collins - abbreviation: International Space Station)
4824) spacewalking + spacewalked (merged into spacewalk)
4825) spacefarer (+plural)
4826) film-making
4827) archaebacterium
4828) archaebacteria (plural of previous word)
4829) eubacterium
4830) eubacteria (plural of previous word)
4831) biliously (merged into bilious)
4832) trilinear
4833) affectless
4834) affectlessness
4835) immunosuppression
4836) immunosuppressant (+plural)
4837) immunosuppressed
4838) immunosorbent
4839) quantitation (+plural)
4840) quantitate (+ed)
4841) monochord (+plural)
4842) pentachord
4843) subdominant
4844) ISPs (added plural)
4845) subduable
4846) subduer
4847) Belford (+'s - name of place)
4848) seamount (+plural)
4849) seamfree
4850) seamers (added plural)
4851) puckery
4852) pintuck (+plural)
4853) hangup (+plural - removed since Oxford says it is two words)
4854) hyperlinking (merged into hyperlink)
4855) Hitlers (added plural)
4856) Hitlerian
4857) Hitlerism
4858) Hitlerite (+plural)
4859) Megachiroptera
4860) megachiropteran (+plural)
4861) multiyear (Collins)
4862) bancassurance
4863) bankassurance (another way of previous word)
4864) bancassurer (+plural)
4865) Allianz (+'s - name)
4866) Kieran (name)
4867) banausic
4868) haricot (+plural)
4869) contrarian (+plural)
4870) contrarianism
4871) quinquagenarian
4872) majoritarian (+plural)
4873) majoritarianism
4874) Gibraltarian (+plural)
4875) Raith's (added the 's - name)
4876) bioprivacy (Collins)
4877) Obamacare
4878) apices (another plural of apex)
4879) apetalous
4880) stipule (+plural)
4881) stipular
4882) unsubmerged (Collins)
4883) Brookes (+'s - name)
4884) Bodleian (Bodleian Library)
4885) Ashmolean (Ashmolean Museum)
4886) Sheldonian (Sheldonian Theatre)
4887) sculpturesque
4888) freewheels (added plural)
4889) freewheeler
4890) passible
4891) passibility
4892) Athanasius (+'s - name)
4893) discarnate
4894) Eileen's (added the 's - name)
4895) Platonism
4896) Platonist (+plural)
4897) logicism
4898) intuitionists (added plural)
4899) intuitionism
4900) intuitionalism (another way of previous word)
4901) compresence
4902) compresent
4903) immoralism
4904) immoralist
4905) Thrasymachus (+'s - name - Wikipedia)
4906) absurdism
4907) Chaplin's (added the 's - name)
4908) absurdist (+plural)
4909) Cartesians (added plural)
4910) Descartes's (added the 's - name)
4911) Cartesianism
4912) defeasibility
4913) defeasibly
4914) counterclaiming (merged into counterclaim)
4915) multithread
4916) Timberlake (name)
4917) offstage (fixed: off-stage)
4918) onstage (fixed: on-stage)
4919) lovelock (+plural)
4920) lovelily
4921) jubbly
4922) vestiture (+plural)
4923) excommunicator (+plural)
4924) excommunicatory
4925) ex-con (+plural - Oxford: informal)
4926) bilboes
4927) pathognomonic
4928) Holmes's (added the 's - name)
4929) shrink-wrap (+s +ing +ed)
4930) break-bulk
4931) multitaskers (added plural)
4932) multitasks (merged into multitask)
4933) multitouch
4934) multituberculate (+plural)
4935) monotreme (+plural)
4936) condylarth (+plural)
4937) condyloma (+plural)
4938) condylomata (another plural of previous word)
4939) condylomatous
4940) CVA (abbreviation: cerebrovascular accident)
4941) STDs (added plural)
4942) reverb (+plural)
4943) handclap (+plural +ing)
4944) popish (+ly)
4945) trad (Oxford: informal)
4946) theremin (+plural)
4947) cinnarizine (Collins)
4948) cyclizine (Collins)
4949) promethazine
4950) amoxycillin
4951) amoxicillin (another way of previous word)
4952) ampicillin
4953) erythromycin
4954) dishwashing
4955) dishy (+ier +iest)
4956) telegenic
4957) telefilm (+plural)
4958) Gucci's (added the 's - name)
4959) clinginess (merged into clingy)
4960) annoyingness
4961) balibuntal
4962) trampette (Collins)
4963) solanaceous
4964) bioacoustics
4965) James's (added the 's - name)
4966) bioaccumulate (+s +ing +ed +ion)
4967) Lucas's (added the 's - name)
4968) sapiens (Homo sapiens)
4969) assibilate (+ed +ion)
4970) rhotic
4971) rhoticity
4972) postvocalic
4973) tics (added plural)
4974) pic (+plural - Oxford: informal)
4975) digicam (+plural)
4976) digipak
4977) untruly (Kevin Atkinson wordlist 60)
4978) unremarked (Kevin Atkinson wordlist 60)
4979) unshaped (Kevin Atkinson wordlist 60)
4980) unposed
4981) mustardy
4982) chorography
4983) chorographer (+plural)
4984) chorographic
4985) geostrategic
4986) geostrategy
4987) Africanism
4988) Africanist
4989) Africanize (+s +ed +ing +ion -IZE)
4990) Africanise (+s +ed +ing +ion -ISE)
4991) bezoar (+plural)
4992) lungworm (+plural)
4993) furunculosis
4994) interdigital
4995) OCRs + OCRing + OCRed (merged into OCR - Wiktionary)
4996) versicle (+plural)
4997) responsory (+plural)
4998) graduals (added plural)
4999) gradiometer
5000) altazimuth
5001) gradualistic
5002) dragline (+plural)
5003) bucketwheel (+plural)
5004) dozers (added plural)
5005) deckhouse (+plural)
5006) shotted
5007) lionfish (+plural - Collins)
5008) triggerfish (+plural)
5009) amidship (added singular)
5010) baggywrinkle
5011) overemployment (Collins)
5012) foozle (+s +ing +ed - Oxford: informal)
5013) forewing (+plural)
5014) beeswing
5015) haltere (+plural)
5016) imaginal
5017) instar (+plural)
5018) hindwings (added plural)
5019) alisphenoid (+plural)
5020) glenoid
5021) fossa
5022) glenohumeral
5023) hyperextend (+s +ing +ed +ion)
5024) dystonia
5025) dystonic
5026) embolization (-IZE)
5027) embolisation (-ISE)
5028) thrombolysis
5029) thrombolytic
5030) ischaemia
5031) ischaemic
5032) autolysis
5033) autolytic
5034) lysosome (+plural)
5035) lysosomal
5036) angulate (+s +ing +ed)
5037) angulation (+plural)
5038) cerebriform (Collins)
5039) Pott (+'s - name)
5040) calcalkaline
5041) laurustinus
5042) desex (+s +ing +ed)
5043) deservingness
5044) lolcat (+plural)
5045) mash-ups (added plural)
5046) Chekhov's (added 's - name)
5047) masjid (+plural)
5048) gurdwara (+plural)
5049) microloan (+plural - Collins)
5050) muggle (+plural - Oxford: informal)
5051) multiverse (+plural)
5052) meme (+plural)
5053) memetic
5054) declass (+ed)
5055) cosplay (+s +ing +ed +er +ers)
5056) COSPAR (abbreviation: Committee on Space Research)
5057) meh (Oxford: informal)
5058) orc (+plural)
5059) orcish
5060) hobbit (added plural +'s)
5061) Gandalf (+'s - name - Wikipedia)
5062) Frodo (+'s - name - Wikipedia)
5063) Tolkien (+'s - name)
5064) Tolkienesque
5065) overshare (+s +ing +ed +er +ers)
5066) grassquit (+plural)
5067) bananaquit (+plural)
5068) soulmate (+plural)
5069) Rastafarian (+plural)
5070) Rastafarianism
5071) Anthropocene
5072) wish-list (removed - Oxford says it is two words)
5073) webisode (+plural)
5074) vuvuzela (+plural)
5075) unfriends (merged into unfriend)
5076) usernames (added plural)
5077) handhelds (added plural)
5078) lammergeier
5079) lammergeyer (another way of previous word)
5080) Fraktur
5081) thunderbird (+plural +'s - lower case - mythical bird)
5082) thickheads (added plural)
5083) Asgard (+'s - name)
5084) Camelopardalis (name of place)
5085) Centaurus (name of place)
5086) aquarobics
5087) divemaster (+plural)
5088) freestyling + freestyled + freestyler + freestylers (merged into freestyle)
5089) biscotti
5090) codebase (+plural - Wiktionary)
5091) postmortem (fixed: post-mortem)
5092) codeless (Collins)
5093) undebated (Collins)
5094) firebase
5095) winery (added singular)
5096) wineskins (added plural)
5097) unshrunk
5098) Winesap (+plural)
5099) nock (+s +ing +ed)
5100) keyserver (+plural - Wiktionary)
5101) QA (abbreviation: Quality assurance)
5102) passphrase (+plural - Wiktionary)
5103) GnuPG's (added the 's)
5104) PostScript (trademark)
5105) Fri (abbreviation: Friday)
5106) Jun (abbreviation: June)
5107) Jul (abbreviation: July)
5108) Sep (abbreviation: September)


V2.23 - 1-APR-2015
------------------
5109) Aintree (name)
5110) ainhum
5111) cholelithiasis
5112) cholecystitis
5113) pancreatitis
5114) cardiomyopathy
5115) pericardial
5116) mammal-like
5117) mammalogy (+ist +ists)
5118) Merriam's (added the 's - name)
5119) supinate (+s +ing +ed +ion)
5120) pronate (+s +ing +ed +ion)
5121) pronator (+plural)
5122) metacentres (added plural)
5123) acrocentric (Collins)
5124) submetacentric (Collins)
5125) Malory (+'s - name)
5126) malperformance
5127) loti (currency)
5128) maloti (plural of previous word)
5129) Lothario (+plural)
5130) butterbur (+'s - Collins)
5131) institute's (added the 's)
5132) extremophile (+plural)
5133) archaebacterial
5134) toxigenicity (Collins)
5135) botulinum (botulinum toxin)
5136) variola
5137) variolar
5138) variolous
5139) Mather (name)
5140) Smoot (+'s - name)
5141) RISC
5142) click-through (+plural)
5143) clientelism
5144) followee (+plural)
5145) aggressivity (Collins)
5146) ballistically
5147) hydroforming (Collins)
5148) SourceForge (+'s)
5149) logon (+plural)
5150) logouts (added plural)
5151) logoff (+plural)
5152) newfound
5153) Harington (name)
5154) sightlessness
5155) Arkansans (added plural - Kevin Atkinson wordlist 60)
5156) Axis (The Axis - alliance of Germany and Italy WWII - Kevin wordlist 60)
5157) Bayamón (name of city - Kevin Atkinson wordlist 60)
5158) Camelots (added plural - Kevin Atkinson wordlist 60)
5159) Carib (+plural)
5160) Devonians (added plural)
5161) cockboat
5162) backwinding + backwinded (merged into backwind)
5163) ripstop
5164) longstop
5165) half-centuries (added plural)
5166) Barclays (+'s - name - Wikipedia)
5167) peggy (+plural - slang)
5168) Peggy (+'s - name)
5169) Chekhovian (Kevin Atkinson wordlist 60)
5170) Comoran (Kevin Atkinson wordlist 60)
5171) Darwinist (+plural - Kevin Atkinson wordlist 60)
5172) Fräulein (Kevin Atkinson wordlist 60)
5173) Galaxy (The Galaxy - uppercase - Kevin Atkinson wordlist 60)
5174) Greenlandic (Kevin Atkinson wordlist 60)
5175) gorgon's (added the 's)
5176) multiheaded (Collins)
5177) Grozny (+'s - name - Kevin Atkinson wordlist 60)
5178) Ivorian (+plural - Kevin Atkinson wordlist 60)
5179) Nepalis (added plural - Kevin Atkinson wordlist 60)
5180) Gurkhali
5181) Gorkhali (another way of previous word)
5182) Vallejo's (added the 's - name)
5183) vallecula
5184) valleculae (plural of previous word)
5185) Nibelungenlied
5186) Nibelung (+plural)
5187) Nibelungen (another plural of previous word)
5188) Burgundians (added plural)
5189) Cassidy (+'s - name - Wikipedia)
5190) Longmire (+'s - name - Wikipedia)
5191) Eurocard (Wikipedia)
5192) lightwork (+er +ers - Wiktionary)
5193) Kenney (name - Wikipedia)
5194) Medina's (added the 's - name)
5195) exclusivism
5196) exclusivist (+plural)
5197) excogitate (+s +ing +ed +ion)
5198) antidisestablishmentarian (+plural)
5199) exceptionalism
5200) ecclesial
5201) interreligious (Collins)
5202) penates
5203) penannular
5204) zoomorphic
5205) zoomorphism
5206) biomorphs (added plural)
5207) biomorph's (added the 's)
5208) biomorphic
5209) Dawkins (+'s - name)
5210) Aristotelians (added plural)
5211) Sacrobosco (+'s - name - Wikipedia)
5212) Paracelsus (+'s - name)
5213) MKV + mkv
5214) AVI
5215) FLV
5216) MK3D + mk3d
5217) MKA + mka
5218) MKS
5219) streamable (Wiktionary)
5220) Matroska's (added the 's)
5221) MOV
5222) Vorbis (+'s - Wikipedia)
5223) ogg + Ogg
5224) oga
5225) WMA
5226) AAC
5227) ambisonic (+plural)
5228) DirectShow (+'s - Wikipedia)
5229) Netherhampton (name)
5230) Phillips's (added the 's - name)
5231) touch-sensitive
5232) microchannel (+plural - Wiktionary)
5233) photoresist (+plural)
5234) photolithography
5235) photolitho (another way of previous word)
5236) photolithographic
5237) photolithographically
5238) microminiature (Collins)
5239) oligonucleotide (+plural)
5240) polynucleotide (+plural)
5241) polynuclear
5242) Woburn (+'s - name)
5243) acrylamide
5244) toxicant (+plural)
5245) immunogenetic (+plural - Collins)
5246) immunogenetical (Collins)
5247) browntop
5248) androsterone
5249) oestradiol
5250) oestrogenic
5251) progestin (+plural)
5252) translocations (added plural)
5253) mistranslate
5254) quadrate (+plural)
5255) trabecula
5256) trabeculae (plural of previous word)
5257) trabecular
5258) trabeculate
5259) intergrowth (+plural)
5260) intergrow
5261) intergrew (past of previous word)
5262) intergrown (past participle of intergrow)
5263) limonite (+ic)
5264) pseudomorph (+plural +ic)
5265) pseudomorphism
5266) pseudomorphous
5267) Charron (name - Wikipedia)
5268) Theophrastus (+'s - name)
5269) Heraclitus (+'s - name)
5270) Parmenides (+'s - name)
5271) Bentham (name)
5272) Kircher (+'s - name - Wikipedia)
5273) Cetacea
5274) Odontoceti
5275) odontocete (+plural)
5276) Mysticeti
5277) mysticete (+plural)
5278) rorqual (+plural)
5279) Bryde (+'s - name - Wikipedia)
5280) Hywel (name - Wikipedia)
5281) Antin (+'s - name - Wikipedia)
5282) medusa (lowercase - +plural)
5283) medusae (lowercase - another plural of previous word)
5284) planula
5285) planulae (plural of previous word)
5286) apoptosis
5287) apoptotic
5288) radioiodine (Collins)
5289) gastrocolic
5290) apoprotein (+plural)
5291) waterborne (fixed: water-borne)
5292) firstborn (fixed: first-born + added 's)
5393) aposematic
5394) aposematism
5395) herbarium
5396) herbaria (plural of previous word)
5397) angiospermous
5398) gymnospermous
5399) monocot (+plural)
5400) dicot (+plural)
5401) dicrotic
5402) phosphodiesterase
5403) methotrexate
5404) HPV (abbreviation: Human papillomavirus)
5405) kindergartener (+plural - Kevin Atkinson wordlist 40)
5406) Fulghum (+'s - name - Wikipedia)
5407) Audra (+'s - name - Wikipedia)
5408) overparted
5409) Leonardo's (added the 's - name)
5410) overpaint (+s +ing +ed)
5411) Baconian (+plural)
5412) inductivism
5413) inductivist (+'s +plural)
5414) consequentialism
5415) consequentialist (+plural)
5416) utilitarians (added plural)
5417) Bosphorus (another way for Bosporus)
5418) deontology
5419) deontological
5420) deontologist (+plural)
5421) PHP (+'s)
5422) php (lower case)
5423) CGI
5424) Ibsen's (added the 's - name)
5425) Gerhart (+'s - name)
5426) Crébillon (name - Wikipedia)
5427) Henrik (+'s - name)
5428) Flaubert (+'s - name)
5429) flaunter
5430) flaunty
5431) Pouncey (+'s - name - Wikipedia)
5432) Byblos (+'s - name)
5433) phospholipase
5434) linolenic (linolenic acid)
5435) chitosan (Collins)
5436) oropharynx
5437) oropharynges + oropharynxes (plural ways of previous word)
5438) oropharyngeal
5439) fungiform
5440) vallate (Collins)
5441) adventitia
5442) adventitial
5443) Adventism
5444) Pentecostals (added plural + changed to upper case)
5445) Pentecostalism
5446) acceptation (+plural)
5447) Riegl (+'s - name - Wikipedia)
5448) Venkova (name - Wikipedia)
5449) admass
5450) Lifford (name)
5451) ambisexual (+ly)
5452) cufflink (+plural)
5453) Paula (+'s - name)
5454) monobrow (+ed - Oxford: informal)
5455) Hayek (+'s - name)
5456) Darger (+'s - name - Wikipedia)
5457) anomalistic
5458) ascidian (+plural)
5459) Cloney (+'s - name - Wikipedia)
5460) Walney (+'s - name - Wikipedia)
5461) razorfish (+plural)
5462) APC (abbreviation: Armoured Personnel Carrier)
5463) Bukrin (name)
5464) Otway (+'s - name)
5465) Amfreville (name)
5466) defenceman
5467) defencemen (plural of previous word)
5468) commodification
5469) Killarney (+'s - name)
5470) peloton (+plural)
5471) bittersweet (fixed: bitter-sweet)
5472) bittersweetness (fixed: bitter-sweetness)
5473) blueliner
5474) multisensory
5475) Bayern (name)
5476) Hašek (+'s - name - Wikipedia)
5477) multiplet (+plural)
5478) diborane
5479) boride (+plural)
5480) borane (+plural)
5481) intermetallic (Collins)
5482) nanostructures (added plural)
5483) nanostructured (merged into nanostructure - Wiktionary)
5484) silicide (+plural)
5485) hepatocyte (+plural)
5486) immunoglobulin (+plural)
5487) cellularity
5488) pleomorphism
5489) pleomorphic
5490) eosinophilic
5491) mucinous
5492) eosinophil (+plural)
5493) chargee
5494) chargeback (+plural)
5495) velodrome (+plural)
5496) McGlynn (+'s - name - Wikipedia)
5497) Daytona (+'s - name)
5498) McRae (+'s - name - Wikipedia)
5499) Nemechek (+'s - name - Wikipedia)
5500) dayside
5501) nightside
5502) astrobleme (+plural)
5503) Pentecost
5504) polyacrylamide
5505) ethidium (ethidium bromide)
5506) Timor (+'s - country)
5507) Timorese
5508) timocracy (+plural)
5509) bacchanalian (in lower case it has other meaning)
5510) archegonium
5511) archegonia (plural of previous word)
5512) Leopold's (added the 's - name)
5513) archducal
5514) archduchy (+plural)
5515) Stringfield (+'s - name - Wikipedia)
5516) Lumberton (+'s - name - Wikipedia)
5517) Revenna (name - Wikipedia)
5518) exopolitics (Wiktionary)
5519) Moulton (+'s - name - Wikipedia)
5520) Marden (+'s - name - Wikipedia)
5521) Westrum (+'s - name - Wikipedia)
5522) Isadora (+'s - name)
5523) adoptee (+plural)
5524) Zawadski (name)
5525) Gavron (+'s - name - Wikipedia)
5526) chromatid (+plural)
5527) anaphase
5528) telophase
5529) prophase
5530) tankage
5531) Gunnar's (added the 's - name)
5532) birdshot
5533) Garda
5534) Gardai (plural of previous word)
5535) Moyross (name - Wikipedia)
5536) unjaded (Collins)
5537) over-bright
5538) glowy (+ier +iest - Oxford: informal)
5539) gloxinia (+plural)
5540) achimenes
5541) cohosh
5542) folic (folic acid)
5543) pteroylglutamic (pteroylglutamic acid)
5544) pterosaur (added singular)
5545) chalicothere (+plural)
5546) glucagon
5547) fluctuant
5548) interpretability
5549) bacteriostat (+plural)
5550) bacteriostasis
5551) bacteriostatic
5552) bacteriostatically
5553) sterilant (+plural)
5554) Vincentian (+plural)
5555) Lazarist (+plural)
5556) Perryville (+'s - name - Wikipedia)
5557) isethionic (isethionic acid)
5558) isethionate
5559) isentropic
5560) Chalmers (+'s - name - Wikipedia)
5561) microelectronically (Collins)
5562) electropolar (Collins)
5563) Hershey's (added the 's - name)
5564) Simson (+'s - name - Wikipedia)
5565) Pathan (+plural)
5566) Chaldean (+plural)
5567) Anatolians (added plural)
5568) napoleon (+plural - allow lower case)
5569) neologist (+plural)
5570) neologize (+s +ing +ed - IZE)
5571) neologise (+s +ing +ed - ISE)
5572) sonics
5573) spaceport (+plural)
5574) Kourou (+'s - name)
5575) nullifidian
5576) uglily
5577) uglify (+s +ing +ed +ion +ly)
5578) streetscape (+plural)
5579) Trowbridge (+'s - name)
5580) Molesworth (+'s - name - Wikipedia)
5581) Nicholls (+'s - name - Wikipedia)
5582) Eindhoven (+'s - name)
5583) Bluefields (+'s - name)
5584) Belém (+'s - name of city)
5585) belemnite (+plural)
5586) Flandrian
5587) auriferous
5588) downwelling (+plural)
5589) upwellings (added plural)
5590) nearshore (+s +ing +ed)
5591) solitarily (merged into solitary)
5592) limitary
5593) limitative
5594) textualist (+plural)
5595) textualism
5596) Latham (+'s - name - Wikipedia)
5597) Humphreys (+'s - name - Wikipedia)
5598) osmotically
5599) Eliza (+'s - name)
5600) Hoberman (+'s - name - Wikipedia)
5601) semipermeable
5602) hydrocolloid (+plural)
5603) subprocess (+plural - Wiktionary)
5604) Dwayne (name - Wikipedia)
5605) updatable
5606) Rappaport (+'s - name - Wikipedia)
5607) upcycling + upcycles (merged into upcycle)
5608) Sheldrick (+'s - name - Wikipedia)
5609) upcoast
5610) upcasts (added plural)
5611) upcase (+s +ed)
5612) downcase (+s +ed)
5613) Westerman (+'s - name - Wikipedia)
5614) Scart + SCART
5615) roll-outs (added plural)
5616) Oberth (+'s - name - Wikipedia)
5617) Emme (+'s - name - Wikipedia)
5618) ESR
5619) Jacklyn (name - Wikipedia)
5620) GCC (+'s)
5621) mudflap (+plural)
5622) misexplanation (+plural - Wiktionary)
5623) misexplain (+s +ing +ed - Wiktionary)
5624) Beatty (+'s - name)
5625) Delphine (+'s - name - Wikipedia)
5626) misqualify (+s +ing +ed - Wiktionary)
5627) cocreates + cocreating + cocreated (merged into cocreate - Wiktionary)
5628) cocreator (+plural - Wiktionary)
5629) cocreation (Wiktionary)
5630) subclauses (added plural)
5631) standard's (added the 's)
5632) vert (+plural)
5633) Tallis (+'s - name)
5634) Flodden (name)
5635) Moreton (+'s - name)
5636) Suffolks (added plural)
5637) Pius (+'s - name)
5638) Wulfsige (name - Wikipedia)
5639) Bardstown (+'s - name - Wikipedia)
5640) Asser (+'s - name - Wikipedia)
5641) Dermot (+'s - name - Wikipedia)
5642) gramophonic
5643) dropout (+plural)
5644) trimerous
5645) pentamerous
5646) tetramerous
5647) cotoneaster
5648) rowan (+plural - in lower case - rowan tree)
5649) broadleaf
5650) whitebeam (+plural)
5651) agapanthus
5652) pelargonium (+plural)
5653) goosegrass
5654) bedstraws (added plural)
5655) Bedu (another term for Bedouin)
5656) McHale (+'s - name - Wikipedia)
5657) Aegon (+'s - name - Wikipedia)
5658) pochard (+plural)
5659) Pocahontas (+'s - name)
5660) Aragon (+'s - name)
5661) Zara (+'s - name - Wikipedia)
5662) crappie (+plural)
5663) flappy (+ier +iest)
5664) giftware
5665) Pattaya (+'s - name)
5666) Chemnitz (+'s - name)
5667) Austerlitz (+'s - name)
5668) Meara (name)
5669) anemophilous
5670) anemophily
5671) hydrophilous
5672) hydrophily
5673) hydrophobicity
5674) intermembrane (Collins)
5675) Zenobia (+'s - name)
5676) cymbidium (+plural)
5677) boneset
5678) knitbone
5679) intarsia
5680) Hendrick (+'s - name - Wikipedia)
5681) panstick
5682) denitrify (+s +ing +ed +ion)
5683) bioreactor (+plural)
5684) wasabi
5685) cruciferous
5686) somniferous
5687) melliferous
5688) umbellifer (+plural)
5689) umbelliferous
5690) bottlescrews (added plural)
5691) forestay
5692) staysail (+plural)
5693) turnbuckle (+plural)
5694) Cowell (+'s - name - Wikipedia)
5695) turnround (another way of turnaround)
5696) Landon (+'s - name)
5697) yuan (allow lower case too - not name in lower case)
5698) Baudot (+'s - name - Wikipedia)
5699) Baudrillard (+'s - name)
5700) Saavedra (+'s - name - Wikipedia)


V2.24 - 1-MAY-2015
------------------
5701) Hopson (+'s - name - Wikipedia)
5702) Coolaney (name - Wikipedia)
5703) Barkley (name - Wikipedia)
5704) hydroid (+plural)
5705) polypoid
5706) theca
5707) thecae (plural of previous word)
5708) thecate
5709) dinoflagellate (+plural)
5710) jobseeker (+'s +plural)
5711) wreaker
5712) Frederica (+'s - name - Wikipedia)
5713) Krista (+'s - name - Wikipedia)
5714) agoraphobe (+plural)
5715) Heworth (name - Wikipedia)
5716) Crothers (+'s - name - Wikipedia)
5717) anorgasmic
5718) anorthosite
5719) uncirculated
5720) Dante's (added the 's - name)
5721) baht (currency of Thailand)
5722) gossamery
5723) Friel (+'s - name - Wikipedia)
5724) grume (+plural)
5725) Duchamp (+'s - name)
5726) balmoral (+plural)
5727) Balmoral (+'s - name)
5728) bluchers
5729) cachectic
5730) cachexia
5731) dyspnoea
5732) cyanosis
5733) cyanotic
5734) lymphadenopathy
5735) splenomegaly
5736) sarcoid (+plural)
5737) immunocompetent
5738) immunocompetence
5739) autoconfigure (+s +ing +ed +ion - Wiktionary)
5740) ultra-thin
5741) Brookhaven (+'s - name)
5742) Brooklands (+'s - name)
5743) Newlands (+'s - name)
5744) interfix (+plural +ing +ed - Wiktionary)
5745) Theodoric (+'s - name)
5746) Ostrogoth (+plural)
5747) Ostrogothic
5748) Goths (added plural)
5749) goths (added plural - lower case)
5750) Crimeans (added plural)
5751) Visigothic
5752) Justinian (+'s - name)
5753) Dalmatia (+'s - name)
5754) Dalmatian (+plural - fix: now in upper case)
5755) Doberman (removed: Chiefly American)
5756) Dobermann (+plural)
5757) Pomeranian (+plural)
5758) Akita (+plural)
5759) Raby (+'s - name - Wikipedia)
5760) Akkadian (+plural)
5761) Akkad (+'s - name)
5762) Dakotas (added plural)
5763) Siouan
5764) undefeatable
5765) Kimble (+'s - name - Wikipedia)
5766) inkjet (inkjet printer)
5767) draftsman
5768) draftsmen (plural of previous word)
5768) egosurfer (+plural - merged into egosurf - Wiktionary)
5769) themself
5770) Themistocles (+'s - name)
5771) Ki-moon (+'s - name)
5772) Bevan (+'s - name)
5773) bevatron (+plural)
5774) radiotherapeutic
5775) repopulation (merged into repopulate)
5776) Liguria (name of place)
5777) Ligurian (+plural)
5778) Surya
5779) appealable
5780) reviewable
5781) reviewal
5782) incompletion
5783) Montaigne's (added the 's - name)
5784) inexhaustibility
5785) Bessey (+'s - name - Wikipedia)
5786) incomputable
5787) agrimony (+plural)
5788) chokecherry
5789) nandina
5790) Afrocentric (Kevin Atkinson wordlist 60)
5791) Afrocentrism
5792) Afrocentrist (+plural)
5793) Barthes (+'s - name - Kevin Atkinson wordlist 60)
5794) bartend (+plural +ing +ed)
5795) parascend (+ing +er +ers)
5796) waterski (+s +ing +ed +er +ers)
5797) Windermere (+'s - name)
5798) ringbone
5799) ringbolt
5800) ringback
5801) ringbark (+s +ing +ed)
5802) ploughland
5803) Pembrokeshire's (added the 's - name)
5804) thorned + thornless (merged into thorn)
5805) thornlike
5806) thornproof
5807) Thorn (+'s - name)
5808) Glenhead (name - Wikipedia)
5809) Cockett (name - Wikipedia)
5810) overstorey (+plural)
5811) overstress (+s +ing +ed)
5812) overnutrition (Collins)
5813) commodify (+plural +ing +ed)
5814) ACW (abbreviation: aircraftwoman)
5815) passcode
5816) correctitude
5817) Overdale (name - Wikipedia)
5818) Pleasington (name - Wikipedia)
5819) Fulford (name - Wikipedia)
5820) Rosenallis (name - Wikipedia)
5821) Butterworth (+'s - name - Wikipedia)
5822) ca (abbreviation)
5823) CA (abbreviation)
5824) washland
5825) Heligoland (+'s - name)
5826) heliocentrically
5827) Copernicus's (added the 's - name)
5828) Shermer (+'s - name - Wikipedia)
5829) Nicolaus (+'s - name)
5830) geocentrism
5831) geobotany
5832) geobotanical
5833) geobotanist (+plural)
5834) phytosociology (Collins)
5835) arctic (also in lower case for other use)
5836) nipplewort
5837) lungwort
5838) lunisolar
5839) geodynamical (Collins)
5840) semimajor (semimajor axis)
5841) semimetals (added plural)
5842) metalloids (added plural)
5843) butanol
5844) thymol
5845) fusel (fusel oil)
5846) fuseway (+plural)
5847) fuscous
5848) McCann (+'s - name - Wikipedia)
5849) Eamon (+'s - name)
5850) ruckus
5851) Kattegat (name)
5852) katsura (+plural)
5853) crenulate (+ed)
5854) crenulation (+plural)
5855) papillated (Collins)
5856) denticulate (+ed)
5857) unretouched (Collins)
5858) ornithoid (Collins)
5859) Guardi (+'s - name)
5860) Kaczorowski (+'s - name - Wikipedia)
5861) Redgrave (+'s - name)
5862) IDs (added plural - Wiktionary)
5863) mins (added plural - Wiktionary)
5864) PMC (abbreviation - Collins)
5865) Schenk (+'s - name - Wikipedia)
5866) Afrikanerdom
5867) Broederbond (+'s)
5868) life-force (removed since Oxford says it is two words)
5869) autoreverse (Collins)
5870) autocrime (+plural - Collins)
5871) 3DES (cryptography - Wiktionary)
5872) IEC (International Electrotechnical Commission - Wiktionary)
5873) Framingham (+'s - name)
5874) Mahdia (+'s - name - Wikipedia)
5875) rulemaking
5876) Zoloft (medicament - Wikipedia)
5877) Paxil (medicament - Wikipedia)
5878) antiulcer (Collins)
5879) polyneuropathy
5880) Ecstasy (the drug - upper case - Kevin Atkinson wordlist 60)
5881) Victorianism (Kevin Atkinson wordlist 60)
5882) Forster's (added the 's - name)
5883) Ecuadorean (+plural - another way of Ecuadorian)
5884) ectotherm (+plural +ic)
5885) ectothermy
5886) endotherm (+plural)
5887) endothermy
5888) hominoid (+plural)
5889) frugivore (+plural)
5890) frugivorous
5891) nectarivorous
5892) Iban
5893) IBAN (+plural - abbreviation: International Bank Account Number)
5894) BIC (+plural - abbreviation: Bank Identifier Code - Wiktionary)
5895) outshoot
5896) outshot (past of previous word)
5897) subacute
5898) haematogenous
5899) extrasystole (+plural)
5900) counterirritant (+plural)
5901) counterirritation
5902) antilogy (+plural)
5903) antilog (added singular)
5904) anti-lock
5905) Lexus (+'s - name of car - Wikipedia)
5906) outdrink (+s +ing)
5907) outdrank (past of previous word)
5908) outdrunk (past participle of word before previous word)
5909) outdrive (+s +ing)
5910) outdrove (past of previous word)
5911) outdriven (past participle of word before previous word)
5912) Carrington (+'s - name)
5913) Beckmann (+'s - name)
5914) McClintock (+'s - name)
5915) McCullers (+'s - name)
5916) McEnroe (+'s - name)
5917) substring (+plural - Wiktionary)
5918) awk (+plural - Oxford: informal)
5919) Gloria's (added the 's - name)
5920) logistician (+plural - Wiktionary)
5921) pseudoword (+plural - Wiktionary)
5922) phonotactic (+plural)
5923) associability
5924) Cayley (+'s - name)
5925) associateships (added plural)
5926) vocables (added plural)
5927) trigram (+plural)
5928) bigram (+plural)
5929) phonogram (+plural)
5930) encipherment
5931) doornails (added plural)
5932) Ebbinghaus (+'s - name - Wikipedia)
5933) housebuilder (+plural)
5934) Halstead (+'s - name - Wikipedia)
5935) forcemeat
5936) power-hungry
5937) Douglas's (added the 's - name)
5938) Menelaus (+'s - name)
5939) Spyckerelle (name of friend)
5940) Filiep (name of friend)
5941) piecrust (piecrust table)
5942) Christie's (added the 's - name)
5943) buy-back (+plural)
5944) Charlie's (added the 's - name)
5945) shrimping (merged into shrimp)
5946) Padstow (name - Wikipedia)
5947) FY (abbreviation: British: Financial Year)
5948) FX
5949) FWIW (Oxford: informal: For What It's Worth)
5950) fuzzy-wuzzy (+plural - Oxford: informal and offensive)
5951) Heckmann (+'s - name - Wikipedia)
5952) bassline (+plural)
5953) mellophone
5954) Etherington (+'s - name - Wikipedia)
5955) lynchpin (+plural - another way of linchpin)
5956) Allcock (+'s - name - Wikipedia)
5957) preplan (+s +ing +ed - Collins)
5958) chamade (Collins)
5959) prepolymer (+plural)
5960) siloxane (+plural)
5961) silane (+plural)
5962) penetrant (+plural)
5963) berk (+plural - Kevin Atkinson wordlist 60)
5964) billhook (+plural - Kevin Atkinson wordlist 60)
5965) Rawiri (name - Wikipedia)
5966) billfish (+plural)
5967) mesopelagic
5968) benthos
5969) benthic
5970) satrapy (+plural)
5971) Persepolis (+'s - name)
5972) perseverate (+s +ing +ed +ion)
5973) Bitterman (+'s - name - Wikipedia)
5974) stressless (merged into stress)
5975) stressor (+plural)
5976) STRESS (computer programming language)
5977) underdress (+s +ing +ed)
5978) incandesce (+s +ing +ed)
5979) Soay (+'s - name)
5980) soaraway
5981) Ryanair (+'s - name - Wikipedia)
5982) soapily (merged into soapy)
5983) Rennie (+'s - name)
5984) countertrade
5985) countervalue
5986) counterweigh (+s +ing +ed)
5987) Zazzo (+'s - name - Wikipedia)
5988) Tynan (name - Wikipedia)
5989) Mephisto (+'s - name - Wikipedia)
5990) Mephistophelian
5991) Mephistophelean
5992) Cynewulf (+'s - name)
5993) Bentham (added the 's - name)
5994) retranslate (+s +ing +ed)
5995) retranslation (+plural)
5996) Teilhard (+'s - name)
5997) Loeb (+'s - name - Wikipedia)
5998) amu (abbreviation: Atomic mass unit)
5999) bigha (+plural)
6000) Warminster (+'s - name - Wikipedia)
6001) Durnford (+'s - name - Wikipedia)
6002) Darell (name - Wikipedia)
6003) Ancaster (+'s - name - Wikipedia)
6004) Waterval (name - Wikipedia)
6005) Balla (+'s - name - Wikipedia)
6006) Malton (+'s - name - Wikipedia)
6007) Croke (+'s - name - Wikipedia)
6008) Moorhead (+'s - name)
6009) Clemente (+'s - name)
6010) sophomoric
6011) Salford's (added the 's - name)
6012) skint (Oxford: informal)
6013) Hemphill (+'s - name - Wikipedia)
6014) skintight
6015) Cassie (+'s - name - Wikipedia)
6016) hued + hueless (merged into hue)
6017) kung + fu (kung fu)
6018) buckbean (+plural)
6019) bogbean (+plural - another way of previous word)
6020) cloudberry (+plural)
6021) Grahamstown (+'s - name - Wikipedia)
6022) airburst (+plural)
6023) flip-flopping + flip-flopped (merged into flip-flop)
6024) frack (+s +ing +ed +er +ers)
6025) hazmat (+plural)
6026) overthink (+s +ing)
6027) overthought (past participle of previous word)
6028) paraglider (+plural)
6029) paraglide (+s +ing +ed)
6030) paywall (+s +ed)
6031) photobomb (+s +ing +ed +er +ers)
6032) fave (+plural - Oxford: informal)
6033) plastique
6034) polyamory
6035) polyamorist
6036) polyamorous
6037) remortgage (+s +ing +ed)
6038) reorg (+plural - Oxford: informal)
6039) Vinson (+'s - name)
6040) satay
6041) scrunchier + scrunchiest + scrunchy
6042) scrunchie (+plural)
6043) slumdog (+plural - Wiktionary)
6044) spams (merged into spam)
6045) stoners (added plural)
6046) truthiness (Oxford: informal)
6047) Colbert's (added the 's - name)
6048) twerked (merged into twerk)
6049) widescreens (added plural)
6050) DMCA (abbreviation: Digital Millennium Copyright Act)
6051) DHSS (abbreviation: Department of Health and Social Security)
6052) Affero (GNU Affero General Public License)
6053) AGPL
6054) Stallman (+'s - name - Wikipedia)
6055) Stallmann (+'s - name - Wikipedia)
6056) relicense (+s +ing +ed)
6057) Hibbard (+'s - name - Wikipedia)
6058) Geraldton (+'s - name)
6059) demi-glace (another way of demi-glaze)
6060) Hippolyta (+'s - name - Wikipedia)
6061) Engberg (+'s - name - Wikipedia)
6062) Aldiss (+'s - name)
6063) Carnell (+'s - name - Wikipedia)
6064) Stubbs (+'s - name - Wikipedia)
6065) Buckland (+'s - name)
6066) globe-like
6067) globoid
6068) globose
6069) perinuclear
6070) microfilament (+plural)
6071) poliovirus (+plural)
6072) echovirus (+plural)
6073) Coxsackie (Coxsackie virus)
6074) Kaposi (+'s - name)
6075) cytomegalovirus
6076) anoxia
6077) anoxic
6078) toxoplasmosis
6079) Harrison's (added the 's - name)
6080) unsubjugated
6081) unsubdued
6082) unstylish
6083) unstuffed
6084) Coogan (+'s - name - Wikipedia)
6085) unstudiedly
6086) Armani (+'s - name)
6087) unstring (+s +ing)
6088) unpressed
6089) prestressed
6090) prestressing
6091) unpresuming
6092) Pollyanna (+plural)
6093) Pollyannaish
6094) Pollyannaism
6095) Mattson (+'s - name - Wikipedia)
6096) unpresentable
6097) Lyotard (+'s - name)
6098) unprescribed
6099) unpreparedness
6100) Harbor (name: Pearl Harbor)
6101) unpremeditatedly
6102) Ullmann (+'s - name - Wikipedia)
6103) UN's (added the 's - United Nations)
6104) unprintably
6105) Carson's (added the 's - name)
6106) Millie's (added the 's - name)
6107) Alexandra (+'s - name -  Wikipedia)
6108) ataraxy
6109) ataraxia (another way of previous word)
6110) ataractic
6111) ataraxic
6112) chiromancy
6113) cartomancy
6114) archaeoastronomy
6115) antitumour
6116) archaea
6117) archaean (+plural - in lower case - biology related)
6118) predawn
6119) Marvin's (added the 's - name)
6120) Harrigan (+'s - name - Wikipedia)
6121) Goldie (+'s - name - Wikipedia)
6122) sporophyte (+plural)
6123) sporophytic
6124) sporangium
6125) sporangia (plural of previous word)
6126) sporangial
6127) avgas
6128) dustproof (Collins)
6129) Bingley (+'s - name - Wikipedia)
6130) Malmesbury (name - Wikipedia)
6131) rebirth (+s +ing +ed +er +ers)
6132) meshwork
6133) mesial (+ly)
6134) crus
6135) crura (plural of previous word)
6136) neoplasia
6137) cingulum
6138) cingula (plural of previous word)
6140) cingulate
6141) commissural
6142) acetabulum
6143) acetabula
6144) cheekpiece (+plural)
6145) Frederickson (+'s - name - Wikipedia)
6146) Gonubie (name - Wikipedia)
6147) lytic
6148) lytically
6149) Sudetenland (name)
6150) Baltistan (+'s - name)
6151) Karakoram
6152) karaka
6153) fescue (+plural)
6154) orchardist (+plural)
6155) rootstocks (added plural)
6156) poisonwood (Collins)
6157) rootsy (+ier +iest)
6158) rootworm (+plural)
6159) nerd's (added the 's)
6160) nerdiness
6161) nerdish
6162) nerdishness
6163) Mahoney (+'s - name - Wikipedia)
6164) Pasadena (+'s - name)
6165) technobabble (Oxford: informal)
6166) Pekar (+'s - name - Wikipedia)
6167) teratogen (+plural +ic)
6168) teratogenicity
6169) teratological
6170) teratologist (+plural)
6171) zeaxanthin
6172) chloroplastic (Collins)
6173) helotage
6174) helotism
6175) helotry
6176) Messenia (+'s - name - Wikipedia)
6177) unfree
6178) unfreedom
6179) Keane's (added the 's - name)
6180) condoners (added plural)
6181) undersow (+s +ing +ed)
6182) undersown (past participle of previous word)
6183) understaffing
6184) engrailed
6185) invected
6186) keyer (+plural)
6187) keyless
6188) Everman (+'s - name - Wikipedia)
6189) Bharati (+'s - name - Wikipedia)
6190) Durga (+'s - name)
6191) Waterberg (name - Wikipedia)
6192) Durey (+'s - name)
6194) Saint-Tropez (+'s - name)
6195) revalorization (Collins -IZE)
6196) revalorisation (Collins -ISE)
6197) Ainsley (+'s - name - Wikipedia)
6198) Matsuyama (name)
6199) matsuri
6200) scablands
6201) Mengele (+'s - name - Wikipedia)
6202) Neuburg (name - Wikipedia)
6203) clientship
6204) clanship
6205) mafia (+plural - lower case - generic Mafia)
6206) Lucian's (added the 's - name)
6207) contrabandist (+plural)
6208) contrabasses (added plural)
6209) autocatalysis
6210) autocatalyst (+plural)
6211) autocatalytic
6212) reprography
6213) reprographer (+plural)
6214) reprographic
6215) cosmography (+plural)
6216) cosmographer (+plural)
6217) cosmographic
6218) cosmographical
6219) Strieber (+'s - name - Wikipedia)
6220) Randle (+'s - name - Wikipedia)
6221) Symington (+'s - name - Wikipedia)
6222) Hellyer (+'s - name - Wikipedia)
6223) landrace (+plural)
6224) admix (+s +ing +ed)
6225) landrail
6226) landplane (+plural)
6227) Corsham (+'s - name - Wikipedia)
6228) lordship (+plural - also in lower case)
6229) signorial (Collins)
6230) signoria (Collins)
6231) Denbigh (name - Wikipedia)
6232) malar
6233) zygomatic (zygomatic bone)
6234) hyperpigmentation
6235) erythematosus (lupus erythematosus)
6236) dermatomyositis
6237) scleroderma
6238) polymyositis
6239) carnitine (Collins)
6240) polymyalgia
6241) rheumatica (polymyalgia rheumatica)
6242) arthralgia
6243) vasculitis
6244) vasculitides (plural of previous word)
6245) myalgia
6246) myalgic
6247) resplendency
6248) demurrable
6249) demurrer (+plural)
6250) potentiate (+plural)
6251) cannabinoid (+plural)
6252) tetrahydrocannabinol
6253) digoxin
6254) glycoside (+plural)
6255) elastin
6256) UTC (Collins - abbreviation: Universal Time Coordinated)
6257) CET (abbreviation: Central European Time)
6258) bimillenary (+plural)
6259) unimodal
6260) extravert (another way for extrovert)
6261) extraversion (another way for extroversion)
6262) underserved
6263) undersecretary (+plural)
6264) frontbencher (+plural)
6265) snubbers (added plural)
6266) clubber (+plural - Oxford: informal)
6267) clozapine
6268) myocarditis
6269) carphology
6270) Elvington (name - Wikipedia)
6271) Lakenheath (+'s - name - Wikipedia)
6272) drysuit (+plural)
6273) Balharry (+'s - name - Wikipedia)
6274) spinneys (added plural)
6275) Schoharie (name - Wikipedia)
6276) maquette (+plural)
6277) Schutte (+'s - name - Wikipedia)
6278) Puget (+'s - name - Wikipedia)
6279) Mapuche (+plural)
6280) Araucanian (+plural)
6281) Gaeltacht (name)
6282) Ramsden (+'s - name - Wikipedia)
6283) Xcode (+'s - Wikipedia)
6284) freeboots + freebooting + freebooted (merged into freeboot)
6285) Blackthorne (+'s - name - Wikipedia)


V2.25 - 1-JUN-2015
------------------
6286) lux
6287) lx (abbreviation of previous word)
6288) illuminance (+plural)
6289) fluoxetine
6290) luminaire (+plural)
6291) chromaticity
6292) nestlings (added plural)
6293) winkler (+plural)
6294) Breydon (+'s - name - Wikipedia)
6295) Netherfield (name - Wikipedia)
6296) Bintley (name - Wikipedia)
6297) Kircubbin (name - Wikipedia)
6298) trail's (added the 's)
6299) Rossendale (+'s - name - Wikipedia)
6300) Gillian (+'s - name - Wikipedia)
6301) sheepshead
6302) waymark (+s +ing +ed +er +ers)
6303) Hawkshead (name - Wikipedia)
6304) Sperrin (name - Wikipedia)
6305) Evette (+'s - name - Wikipedia)
6306) Donnie (+'s - name - Wikipedia)
6307) Maddy (+'s - name - Wikipedia)
6308) surtout
6309) Wilder (+'s - name - Wikipedia)
6310) Cassady (+'s - name - Wikipedia)
6311) Wootton (+'s - name - Wikipedia)
6312) Marisol (+'s - name - Wikipedia)
6313) Mencken (+'s - name - Wikipedia)
6314) Denpasar (+'s - name - Wikipedia)
6315) Tybee (+'s - name - Wikipedia)
6316) Kurtz (+'s - name - Wikipedia)
6317) Smithville (+'s - name - Wikipedia)
6318) Dothan (+'s - name - Wikipedia)
6319) Fogg (+'s - name - Wikipedia)
6320) Flintoff's (added the 's - name)
6321) batsmanship
6322) batrachian
6323) anuran (+plural)
6324) branchia
6325) branchiae (plural of previous word)
6326) branchial
6327) Janvier (+'s - name - Wikipedia)
6328) unjointed
6329) craniate (+plural)
6330) out-group (+plural)
6331) cephalochordate (+plural)
6332) urochordate (+plural)
6333) tunicate (+plural +ed)
6334) plumose
6335) glabrous
6336) labellum
6337) labella (plural of previous word)
6338) diapause (+ing)
6339) diapedesis
6340) Waller (+'s - name - Wikipedia)
6341) polymorphonuclear
6342) leucocyte (+plural +ic)
6343) immunoreactive (Collins)
6344) phagocytosis
6345) phagocytize (+s +ing +ed - IZE)
6346) phagocytize (+s +ing +ed - ISE)
6347) phagocytose (+ed)
6348) neutrophilic
6349) chronobiologist's (added the 's)
6350) outpaces (added plural)
6351) Arber (+'s - name - Wikipedia)
6352) Columba (name)
6353) Columbae
6354) outmuscle (+s +ing +ed)
6355) outbreed (+ing)
6356) outbred (past and past participle of previous word)
6357) carnivalesque
6358) weblogging (Collins)
6359) orgiastically
6360) orgulous
6361) orgasmically
6362) orgastic
6363) orgastically
6364) vaginismus
6365) vaginoplasty
6366) biohacking
6367) biohacker (+plural)
6368) Gerstein (+'s - name - Wikipedia)
6369) bioaccumulations (added plural)
6370) metabolizes + metabolizing + metabolized (merged into metabolize - IZE)
6371) metabolizable (IZE)
6372) metabolisable (ISE)
6373) metabolizer (+plural - IZE)
6374) metaboliser (+plural - ISE)
6375) evaginate
6376) evagination (+plural)
6377) evert (+s +ing +ed)
6378) eversible
6379) eversion
6380) adenohypophysis (Collins)
6381) adenohypophyses (Collins - plural of previous word)
6382) nuchal
6383) trapezius
6384) trapezii (plural of previous word)
6385) spinose
6386) spinous (another way of previous word)
6387) subnatural (Collins)
6388) subsale (Collins)
6389) subsocial (Collins)
6390) substyle (Collins)
6391) suprarational (Collins)
6392) seminatural (Collins)
6393) supernature (Collins)
6394) supravitally (Collins)
6395) Edentata
6396) Xenarthra (another way of previous word)
6397) xenarthran (+plural)
6398) polyphyletic
6399) paraphyletic
6400) edentate (+plural)
6401) sloths (added plural)
6402) sabretooth
6403) Pleistocene
6404) australopithecine (+plural)
6405) Pliocene
6406) unconformably
6407) Wiens (name - Wikipedia)
6408) Moine (name - Wikipedia)
6409) Permian
6410) Lindisfarne (+'s - name)
6411) ammoniacal
6412) semelparous (Collins)
6413) cupro-nickel
6414) cupule (+plural)
6415) uniaxial (+ly)
6416) encapsidate (+s +ing +ed +ion)
6417) isotopically
6418) isotopy
6419) triton (+plural +'s)
6420) Triton (+'s - name)
6421) overmedicate (Collins)
6422) overmedication (Collins)
6423) overprocess (Collins)
6424) overpromote (Collins)
6425) overcredulous (Collins)
6426) overedit (Collins)
6427) overtedious (Collins)
6428) Declan (+'s - name - Wikipedia)
6429) electroclash (Collins)
6430) Kandy (name - Wikipedia)
6431) Mandy (+'s - name - Wikipedia)
6432) Nandi (+'s - name - Wikipedia)
6433) bandy-bandy (+plural)
6434) Zack (+'s - name - Wikipedia)
6425) Phobos (name)
6426) Brixton (+'s - name - Wikipedia)
6427) Bettman (+'s - name - Wikipedia)
6428) Schlemiel (+s - name - Wikipedia)
6429) Ursa (Ursa Major)
6430) uroscopy
6431) Theophilus (+'s - name - Wikipedia)
6432) urostyle
6433) kinesis
6434) kineses (plural of previous word)
6435) kinesiology
6436) kinesiological
6437) kinesiologist (+plural)
6438) Samarra (+'s - name - Wikipedia)
6439) symplast (+ic)
6440) plasmodesma
6441) plasmodesmata (plural of previous word)
6442) adpressed
6443) sclerite (+plural)
6444) dolphinarium (+plural)
6445) dolphinaria (another plural of previous word)
6446) bottlenose (dolphin + whale)
6447) bottle-nosed (dolphin + whale)
6448) armamentarium
6449) armamentaria (plural of previous word)
6450) interneurons (plural of interneuron)
6451) interneurones (plural of  interneurone - another way of previous word)
6452) Gurley (+'s - name - Wikipedia)
6453) homonymic
6454) homonymous
6455) homonymy
6456) polysemy
6457) polysemic
6458) polysemous
6459) anonym (+plural)
6460) Oguibe (+'s - name - Wikipedia)
6461) thunderbox (fixed: thunder-box)
6462) Max (+'s - name - Wikipedia)
6463) Turnbull (+'s - name - Wikipedia)
6464) thunderbug (+plural)
6465) Newbridge (name - Wikipedia)
6466) McNally (+'s - name - Wikipedia)
6467) McCaffrey (+'s - name - Wikipedia)
6468) Queally (+'s - name - Wikipedia)
6469) thunderousness
6470) Gilmartin (+'s - name - Wikipedia)
6471) thunderhead (+plural)
6472) thunderfly (+plural)
6473) aftersales
6474) hotlines (added plural)
6475) hotlink (+plural +ing - Wiktionary)
6476) hotlist
6477) apolune
6478) perilune
6479) manoeuvrer (+plural)
6480) Maugham (+'s - name - Wikipedia)
6481) chlorophyte (+plural)
6482) rhodophyte (+plural)
6483) monophyly (Collins)
6484) adnate
6485) connate
6486) calathea (+plural)
6487) beefwood
6488) stocktake (+r +ers)
6489) RAAF (abbreviation for: Royal Australian Air Force)
6490) Mochizuki (+'s - name - Wikipedia)
6491) Eratosthenes's (added the 's - name)
6492) compositeness (merged into composite)
6493) Andie (+'s - name - Wikipedia)
6494) Annabelle (+'s - name - Wikipedia)
6495) Resnick (+'s - name - Wikipedia)
6496) Muthesius (name - Wikipedia)
6497) workboat (+plural)
6498) refloat (+s +ing +ed)
6499) reflexologist (+plural)
6500) reflexible
6501) reflexibility
6502) Zeiss (+'s - name)
6503) Leica (+'s - name - Wikipedia)
6504) Pentax (+'s - name - Wikipedia)
6505) Benoni (+'s - name - Wikipedia)
6506) zeitgeber (+plural)
6507) zein
6508) Evans-Pritchard (+'s - name)
6509) evaporable
6510) hydroperoxide (Collins)
6511) anthracene
6512) pyrene
6513) alizarin
6514) Kossa (+'s - name - Wikipedia)
6515) Raes (name - Wikipedia)
6516) unimolecular
6517) Stern-Volmer (Stern-Volmer relationship: Wikipedia)
6518) unseduced (Collins)
6519) Giuliani (+'s - name - Wikipedia)
6520) unimposingly
6521) Ana (+'s - name - Wikipedia)
6522) biryani + biriyani + biriani (Indian dish)
6523) birthwort (+plural)
6524) birthweight
6525) haematocrit
6526) intraoperative
6527) spectate (+s +ing +ed)
6528) spectatorial
6529) spectatorship
6530) spectinomycin
6531) biogeneric (+plural)
6532) Coughlin (+'s - name - Wikipedia)
6533) equivalency
6534) Bahamians (added plural)
6535) Cameroonian (+plural)
6536) Mariel (+'s - name - Wikipedia)
6537) Savimbi (+'s - name - Wikipedia)
6538) McKenna (+'s - name - Wikipedia)
6539) Robichaud (+'s - name - Wikipedia)
6540) plugger (+plural)
6541) Chuckie (+'s - name - Wikipedia)
6542) Francophile (+plural)
6543) Lennox (+'s - name - Wikipedia)
6544) Sandhurst (+'s - name - Wikipedia)
6545) Anglophilia
6546) Breitinger (+'s - name - Wikipedia)
6547) flatbread
6548) flatbug
6549) Nuttall (+'s - name - Wikipedia)
6550) flyest
6551) flyness
6552) ferruginous
6553) Mery (+'s - name of friend)
6554) Eckard (+'s - name - Wikipedia)
6555) Berberich (+'s - name - Wikipedia)
6556) MS-DOS
6557) bootmaker (+plural)
6558) Brighouse (+'s - name - Wikipedia)
6559) Gupta (+'s - name - Wikipedia)
6560) Sharam (+'s - name - Wikipedia)
6561) bootlicker (+plural - Oxford: informal)
6562) Bloomberg (+'s - name - Wikipedia)
6563) Babs (+'s - name - Wikipedia)
6564) necklet (+plural)
6565) Lismore (+'s - name - Wikipedia)
6566) Netto (+'s - name - Wikipedia)
6567) surjection (+plural)
6568) surjective
6569) permutational
6570) Tattersall (+'s - name - Wikipedia)
6571) Petrov (+'s - name - Wikipedia)
6572) out-half
6573) Mullingar (+'s - name - Wikipedia)
6574) monophyletic
6575) endosymbiosis
6576) endosymbiont (+plural)
6577) endosymbiotic
6578) phototroph (+ic)
6579) anthozoan (+plural)
6580) anthophilous
6581) Anthozoa
6582) Cnidaria
6583) Ctenophora
6584) Placozoa
6585) Echiura
6586) echiuran (+plural)
6587) echiuroid (+plural)
6588) respirometer (+plural)
6589) polarographic
6590) potentiometry (+ic)
6591) Polaroid (+plural - Oxford: trademark)
6592) Jeana (name - Wikipedia)
6593) doublure
6594) Borobudur (+'s - name - Wikipedia)
6595) photogravure (+plural)
6596) Gornik (+'s - name - Wikipedia)
6597) calotype
6598) Lacock (name - Wikipedia)
6599) Ezekiel's (added the 's - name)
6600) Antiguan
6601) antihypertensive
6602) antidiabetic (Collins)
6603) hypotension
6604) multisystem (Collins)
6605) obtention
6606) hypostyle
6607) Ramses (+'s - name - Wikipedia)
6608) Karnak (+'s - name - Wikipedia)
6609) Serlio (+'s - name - Wikipedia)
6610) Iphigenia (+'s - name - Wikipedia)
6611) tetrastyle
6612) porticos (another plural of portico)
6613) hexastyle (+plural)
6614) octastyle
6615) Bernini (+'s - name - Wikipedia)
6616) octavalent
6617) octarchy (+plural)
6618) Gaurav (+'s - name - Wikipedia)
6619) Jamil (+'s - name - Wikipedia)
6620) Joel's (added the 's - name - Wikipedia)
6621) Arriaga (+'s - name - Wikipedia)
6622) Burney (+'s - name - Wikipedia)
6623) Lynette (+'s - name - Wikipedia)
6624) piquet
6625) wagonette + waggonette (+plural)
6626) Rivington (name - Wikipedia)
6627) Harwood (+'s - name - Wikipedia)
6628) Newbould (+'s - name - Wikipedia)
6629) wagonload (+plural)
6630) Lupe (+'s - name - Wikipedia)
6631) wagoner + waggoner (+'s +plural)
6632) Fairburn (+'s - name - Wikipedia)
6633) wagon-lit
6634) wagons-lits (plural of previous word)
6635) Arlette (name - Wikipedia)
6636) Paulette (+'s - name - Wikipedia)
6637) Claudette (+'s - name - Wikipedia)


V2.26 - 1-JUL-2015
------------------
6638) understeer (+s +ing +ed)
6639) oversteer (+s +ing +ed)
6640) WAP
6641) druggy
6642) Tomaselli (+'s - name - Wikipedia)
6643) Parello (+'s - name - Wikipedia)
6644) Dulwich (+'s - name - Wikipedia)
6645) drugget
6646) drug-related
6647) drug-free
6648) Liam (+'s - name - Wikipedia)
6649) Nadelmann (+'s - name - Wikipedia)
6650) Dewhurst (+'s - name - Wikipedia)
6651) Haider (+'s - name - Wikipedia)
6652) Ravenscroft (+'s - name - Wikipedia)
6653) Ede (+'s - name - Wikipedia)
6654) Aguila (+'s - name - Wikipedia)
6655) Bowker (+'s - name - Wikipedia)
6656) dribbly
6657) hyaline
6658) disodium (Collins)
6659) intranuclear (Collins)
6660) svn (lower case)
6661) SVN (upper case)
6662) tarball (+plural - Wiktionary)
6663) gzip (+s +ing +ed - Wiktionary)
6664) Walfield (name - Wikipedia)
6665) Gillmor (+'s - name - Wikipedia)
6666) Pescetti (+'s - name - Wikipedia)
6667) Galoppini (name - Wikipedia)
6668) Beaussier (name - Wikipedia)
6669) Valk (+'s - name - Wikipedia)
6670) CMS (abbreviation: Content Management System)
6671) Henschel (+'s - name - Wikipedia)
6672) Yutaka (+'s - name - Wikipedia)
6673) Philipp (+'s - name - Wikipedia)
6674) Kewisch (+'s - name - Wikipedia)
6675) stargate (+plural - Wiktionary)
6676) teleporter (+plural - Wiktionary)
6677) Redfern (+'s - name - Wikipedia)
6678) Hopkinsville (+'s - name - Wikipedia)
6679) CST (abbreviation: Central Standard Time)
6680) Lynne (+'s - name - Wikipedia)
6681) DipEd
6682) classily
6683) clastic
6684) fissility
6685) decompaction (+plural - Wiktionary)
6686) interlayers (added plural)
6687) sconces (added plural)
6688) scombroid (+plural)
6689) ciguatera
6690) parrotfish (+plural)
6691) parrotbill (+plural)
6692) vegetational
6693) acridine
6694) ammoniated
6695) valerian (+plural - in lower case if not a name)
6696) campanula (+plural)
6697) cannabinol
6698) photoproduct (+plural)
6699) photoprotein
6700) photorealist
6701) photorealistic
6702) anamorphosis
6703) anamorphoses (plural of previous word)
6704) eccentrical (Collins)
6705) gesso
6706) gessoes (plural of previous word)
6707) glass-like
6708) didymium
6709) Pyrex (trademark)
6710) Jena (+'s - name)
6711) cullet
6712) Flemings (added plural)
6713) Bretons (added plural)
6714) Celts (added plural)
6715) Galicians (added plural)
6716) Catalans (added plural)
6717) Asturians (added plural)
6718) Pyrenees
6719) Pyrenean
6720) carbamate (+plural)
6721) organophosphate (+plural)
6722) organochlorine (+plural)
6723) disruptors (added plural)
6724) arbitral
6725) riskless
6726) Shleifer (+'s - name - Wikipedia)
6727) risk-neutral
6728) chargehand (+plural)
6729) Kuster (+'s - name - Wikipedia)
6730) lampstand (Collins)
6731) Auriga (name in space)
6732) MacPherson (+'s - name - Wikipedia)
6733) moxibustion
6734) adaptogenic
6735) Yoda (+'s - name - Wikipedia)
6736) therapeutical
6737) therapeutist (+plural)
6738) glucosamine
6739) nicotinamide
6740) nicotinic (nicotinic acid)
6741) catabolite (+plural)
6742) catachresis
6743) catachreses (plural of previous word)
6744) catachrestic
6745) continuative (+plural)
6746) basilect (+plural +'s)
6747) acrolect's (added the 's)
6748) basilectal
6749) idiolects (added plural)
6750) eggcorns (added plural)
6751) Coren (+'s - name - Wikipedia)
6752) antepenult
6753) penult
6754) boustrophedon
6755) Gellibrand (+'s - name - Wikipedia)
6756) declinational
6757) declaw (+s +ing +ed)
6758) enzootic
6759) epizootic
6760) epizoic
6761) epizoite (+plural)
6762) nutcase (+plural - Oxford: informal)
6763) Lachman (+'s - name - Wikipedia)
6764) Lawrie (+'s - name - Wikipedia)
6765) dispiritingly
6766) Balanchine (+'s - name - Wikipedia)
6767) suffixal (Collins)
6768) suffixion (Collins)
6769) aspectual
6770) aspecting + aspected (merged into aspect)
6771) trine (+plural)
6772) Dorotheus (name - Wikipedia)
6773) midheaven
6774) Torrington (+'s - name - Wikipedia)
6775) beachside
6776) Suttons (name - Wikipedia)
6777) Redcliffe (+'s - name - Wikipedia)
6778) Klandasan (name - Wikipedia)
6779) meathead (+plural - Oxford: informal)
6780) Hitchman (+'s - name - Wikipedia)
6781) Clegg (+'s - name - Wikipedia)
6782) Rodwell (+'s - name - Wikipedia)
6783) Guiley (+'s - name - Wikipedia)
6784) NDE (+plural - Wiktionary - Abbreviation: near-death experience)
6785) dromos
6786) dromoi (plural of previous word)
6787) Bactrian (Bactrian camel)
6788) bacteroid (+plural)
6789) Bactria (name of place)
6790) semiformal
6791) semiweekly
6792) semirural (Collins)
6793) semierect (Collins)
6794) semilethal (Collins)
6795) semifeudal (Collins)
6796) semivocal (Collins)
6797) semideaf (Collins)
6798) semiraw (Collins)
6799) semioval (Collins)
6800) Ashtaroth (+'s - name - Wikipedia)
6801) totemism
6802) totemist (+plural)
6803) totemistic
6804) Gotham (+'s - name - Wikipedia)
6805) Lacan (+'s - name - Wikipedia)
6806) Lacanian (+plural)
6807) Lacanianism
6808) Havelock (+'s - name - Wikipedia)
6809) Bergenfield (+'s - name - Wikipedia)
6810) uprose
6811) uninterruptible
6812) uninterruptedness
6813) uninterestingness
6814) psychobabble
6815) untestable
6816) unrequested
6817) unintellectual
6818) MacDiarmid (+'s - name - Wikipedia)
6819) Kerala (+'s - name - Wikipedia)
6820) arrondissement (+plural)
6821) Chen's (added the 's - name)
6822) judiciary's (added the 's)
6823) arriviste (+plural)
6824) Thackeray (+'s - name - Wikipedia)
6825) arrivisme
6826) midfielders (added plural)
6827) Bullen (+'s - name - Wikipedia)
6828) yielder (+plural)
6829) Yiddishism
6830) Yiddishist (+plural)
6831) anaphylactic
6832) camwood
6833) Camus (+'s - name - Wikipedia)
6834) layshaft (+plural)
6835) driveshaft (+plural)
6836) swingarm (Collins)
6837) Hinkley (+'s - name - Wikipedia)
6838) GTA (Wiktionary)
6839) demerges + demerging (merged into demerge)
6840) demeritorious
6841) Piaget (+'s - name - Wikipedia)
6842) electricals (added plural)
6843) photoelectricity
6844) gas-cooled
6845) Torness (name - Wikipedia)
6846) comminuted
6847) orthopaedist (+plural)
6848) trochanteric (Collins)
6849) trochanteral (Collins)
6850) triacetate
6851) macrolanguage (+plural - Wiktionary)
6852) macrolinguistics (Collins)
6853) protolanguage (+plural)
6854) paralanguage
6855) code-switch (+s +ing +ed)
6856) unilingual
6857) unilingualism
6858) unilingually
6859) Quebecer (+plural)
6860) Ogunquit (+'s - name - Wikipedia)
6861) quebracho (+plural)
6862) bluegums (added plural)
6863) agroforestry
6864) florals (added plural)
6865) selfing + selfed (merged into self)
6866) Brahmana (+plural)
6867) branks (Collins)
6868) araneous (Collins)
6869) rankish (Collins)
6870) rankless (Collins)
6871) rankism (Collins)
6872) crankish (Collins)
6873) Bruegel + Breughel + Brueghel (name)
6874) Cree (+plural - Kevin Atkinson wordlist 60)
6875) dayan
6876) dayanim (plural of previous word)
6877) Frunze (+'s - name - Kevin Atkinson wordlist 60)
6878) Bresson (+'s - name - Wikipedia)
6879) fountainhead (+plural)
6880) Guamanian (+plural - Kevin Atkinson wordlist 60)
6881) Qur'an + Quran (another way of Koran)
6882) Koranic + Qur'anic + Quranic
6883) Lydians (added plural - Kevin Atkinson wordlist 60)
6884) Mede (+plural)
6885) Chicagoland (+plural +'s)
6886) Imola (+'s - name - Wikipedia)
6887) hand-pick (+ed)
6888) Handan (+'s - name - Wikipedia)
6889) hand-painted
6890) Romina (+'s - name - Wikipedia)
6891) fritillary (+plural)
6892) Silverton (+'s - name - Wikipedia)
6893) Tretikov (+'s - name - Wikipedia)
6894) Wikimedia (+'s - name - Wikipedia)
6895) Solapur (+'s - name - Wikipedia)
6896) Akshaya (+'s - name - Wikipedia)
6897) Iyengar (+'s - name - Wikipedia)
6898) triggers (merged into trigger)
6899) added "re" to trigger + triggers + triggering + triggered (Wiktionary)
6900) pinboard (+plural)
6901) Breightmet (name - Wikipedia)
6902) dev (+plural - Wiktionary: abbreviation of developer)
6903) devkit (+plural - Wiktionary)
6904) annulate (+ed)
6905) annulation (+plural)
6906) seta
6907) setae (plural of previous word)
6908) setaceous
6909) setal
6910) clayware (Collins)
6911) epidermoid
6912) gastrula
6913) gastrulae (plural of previous word)
6914) gastrulation
6915) Cottesloe (+'s - name - Wikipedia)
6916) Bonnybridge (name - Wikipedia)
6917) Grangemouth (+'s - name - Wikipedia)
6918) Beckton
6919) printworks
6920) Boroughbridge (name - Wikipedia)
6921) flashover (+plural)
6922) oversaturated (Collins)
6923) teleoperation
6924) teleoperate (+s +ing +ed)
6925) teleoperator
6926) chatterbot (+plural)
6927) Ramona (+'s - name - Wikipedia)
6928) cyberporn
6929) Danni (+'s - name - Wikipedia)
6930) cyberpet (+plural)
6931) Leonard (added the 's - name)
6932) Lolita (+plural)
6933) literality
6934) literalize (-IZE)
6935) literalise (-ISE)
6936) literalist (+plural)
6937) scientism
6938) scientistic
6939) pettifoggery
6940) barratry
6941) barrator (+plural)
6942) barratrous
6943) cusk-eel (+plural)
6944) gorgonian (+plural)
6945) pipefish (+plural)
6946) pipeclay (+s +ing +ed)
6947) trishaw (+plural)
6948) Sharath (+'s - name - Wikipedia)
6949) Penang (+'s - name - Wikipedia)
6950) shophouse (+plural)
6951) Lovett (+'s - name - Wikipedia)
6952) shopman
6953) shopmen (plural of previous word)
6954) shopworker (+plural)
6955) Burnham (+'s - name - Wikipedia)
6956) obstructor (+plural)
6957) Kingsford (+'s - name - Wikipedia)
6958) Jens (+'s - name - Wikipedia)
6959) postglacial
6960) peatland (+plural)
6961) ActiveX
6962) autorecovery (Google)
6963) autorecover (Google)
6964) anonymise (+s +ing +ed - ISE)
6965) anonymize (+s +ing +ed - IZE)
6966) anonymization (-IZE)
6967) anonymisation (-ISE)
6968) Keaton (+'s - name - Wikipedia)
6969) Kinsella (+'s - name - Wikipedia)
6970) Al-Anon
6971) Groff (+'s - name - Wikipedia)
6972) immunotoxic (Collins)
6973) aflatoxin (+plural)
6974) groundnuts (added plural)
6975) groundmass
6976) phenocryst (+plural)
6977) xenocryst (+plural +ic)
6978) apomixis
6979) apomictic
6980) facultative
6981) facultatively
6982) auxotroph (+plural +ic)
6983) thiamin (another way of thiamine)
6984) autocrine
6985) paracrine
6986) chemosensory
6987) sensillum
6988) sensilla (plural of previous word)
6989) campaniform (Collins)
6990) palp (+plural)
6991) palpi (another plural of previous word)
6992) palpus (another way of palp)
6993) palpal
6994) palpability
6995) multicentric
6996) mammographic
6997) deuteranopia
6998) bryology
6999) bryological
7000) bryologist (+plural)
7001) Ruud (+'s - name - Wikipedia)
7002) bryony (+plural)
7003) coralroot (+plural)
7004) thimbleberry (+plural)
7005) bittercress
7006) fireweed
7007) waterweed
7008) silverweed
7009) willowherb
7010) rosebay
7011) Scythia (name)
7012) Scythian (+plural)
7013) Sarmatia (name)
7014) Sarmatian
7015) Ossete (+plural)
7016) Ossetic
7017) Ossetia (name)
7018) Ossetian (+plural)
7019) Chechens (added plural)
7020) Levi's (added the 's - name)
7021) Hannah's (added the 's - name)
7022) browbeater (+plural)
7023) Wolfowitz (+'s - name - Wikipedia)
7024) Broward (name of place)
7025) chaffweed
7026) buffo (+plural)
7027) CRM (abbreviation: Customer Relationship Management)
7028) Paterson (+'s - name - Wikipedia)
7029) croakily
7030) Maggie's (added the 's - name)
7031) okey-dokey
7032) okey-doke (another way of previous word)
7033) Protista
7034) protist (+plural)
7035) protistan (+plural)
7036) protistology
7037) unwalled
7038) embryogenesis
7039) embryogenic
7040) embryogeny
7041) explant (+plural +ed)
7042) explantation
7043) concrescence
7044) concrescent
7045) intermedium
7046) intermedia (plural of previous word)
7047) griot (+plural)
7048) oilstone
7049) cheese-skipper
7050) cheese-cutter (+plural)
7051) cheese-paring
7052) Kidman (+'s - name - Wikipedia)
7053) Sheehan (+'s - name - Wikipedia)
7054) meany (another way of meanie)
7055) capitulum
7056) capitula (plural of previous word)
7057) coracoid
7058) CNS (abbreviation: Central Nervous System)
7059) cnidarian (+plural)
7060) nematocyst (+plural)
7061) lancelet (+plural)
7062) notochord (+plural)


V2.27 - 1-AUG-2015
------------------
7063) Uriarte (+'s - name - Wikipedia)
7064) Traynor (+'s - name - Wikipedia)
7065) dreamwork
7066) Hergenröther (+'s - name - Wikipedia)
7067) Ralph (+'s - name - Wikipedia)
7068) far-distant
7069) far-ranging
7070) Verdana
7071) unextended (Collins)
7072) blobby (+ier +iest)
7073) luderick
7074) Luddites (added plural)
7075) Luddism
7076) Ludditism
7077) pressroom (Collins)
7078) dominators (added plural)
7079) gynarchy (+plural)
7080) Augustine's (added the 's - name)
7081) emmet
7082) mitogen (+plural +ic)
7083) vasoconstriction
7084) vasoconstrictive
7085) vasoconstrictor (+plural)
7086) antidiuretic (antidiuretic hormone)
7087) antidiarrhoeal
7088) anti-emetics (added plural)
7089) inhabitability
7090) ultrarapid (Collins)
7091) ultracompetent (Collins)
7092) multispeed (Collins)
7093) superspeed (Collins)
7094) ultrasafe (Collins)
7095) outspeed (Collins)
7096) counterterror (Collins)
7097) counterthreat (Collins)
7098) counterthrust (Collins)
7099) countereffort (Collins)
7100) counterreformer (Collins)
7101) ultisol (+plural)
7102) oxisol (+plural)
7103) epicentral
7104) transcurrent
7105) rockslide (+plural)
7106) icefall (+plural)
7107) Perry (+'s - name - Wikipedia)
7108) snowshoers (added plural)
7109) après-ski (+ing)
7110) Zermatt (+'s - name - Wikipedia)
7111) barrique (+plural)
7112) cedary (Collins)
7113) unprioritized (Wiktionary: -IZE)
7114) unprioritised (Wiktionary: -ISE)
7115) PDT (abbreviation: Pacific Daylight Time)
7116) daymare (+plural)
7117) daypack (+plural)
7118) daysacks (added plural)
7119) flyblow
7120) flyback
7121) Damietta (+'s - name - Wikipedia)
7122) referentiality
7123) Watten (+'s - name - Wikipedia)
7124) Derrida (+'s - name - Wikipedia)
7125) Stancanelli (name - Wikipedia)
7126) Appiah (+'s - name - Wikipedia)
7127) gastroenterology
7128) gastroenterological
7129) gastroenterologist (+plural)
7130) dysphagia
7131) dysarthria
7132) contrabassoon
7133) endometrium
7134) endometrial
7135) endometriosis
7136) endolymph
7137) endoglossic
7138) exoglossic
7139) agrammatism
7140) agranulocytosis
7141) epiphanic
7142) Azilian
7143) Chalcolithic
7144) Acheulian
7145) agonistic
7146) agonistically
7147) welfarism
7148) welfarist
7149) Bretton (+'s - name - Wikipedia)
7150) Beveridge (+'s - name - Wikipedia)
7151) Metrotown (+'s - name - Wikipedia)
7152) Burnaby (+'s - name - Wikipedia)
7153) Rideau (+'s - name - Wikipedia)
7154) Manulife (+'s - name - Wikipedia)
7155) Ripon (+'s - name - Wikipedia)
7156) Wakefield's (added the 's - name - Wikipedia)
7157) Southwell (+'s - name - Wikipedia)
7158) Devonport (+'s - name - Wikipedia)
7159) Stonehouse (+'s - name - Wikipedia)
7160) Teesside (+'s - name - Wikipedia)
7161) Sarum (+'s - name - Wikipedia)
7162) Sandwell (+'s - name - Wikipedia)
7163) Asaph (+'s - name - Wikipedia)
7164) Ilkley (+'s - name - Wikipedia)
7165) Shipley (+'s - name - Wikipedia)
7166) Herne (+'s - name - Wikipedia)
7167) Carnforth (+'s - name - Wikipedia)
7168) Heysham (+'s - name - Wikipedia)
7169) Otley (+'s - name - Wikipedia)
7170) Harpenden (+'s - name - Wikipedia)
7171) Pendlebury (+'s - name - Wikipedia)
7172) Swinton (+'s - name - Wikipedia)
7173) Hetton-le-Hole (name - Wikipedia)
7174) Houghton-le-Spring (+'s - name - Wikipedia)
7175) Castleford (+'s - name - Wikipedia)
7176) Ossett (+'s - name - Wikipedia)
7177) Pontefract (+'s - name - Wikipedia)
7178) Alresford (+'s - name - Wikipedia)
7179) Tameside (+'s - name - Wikipedia)
7180) Cottingham (+'s - name - Wikipedia)
7181) Brechin (+'s - name - Wikipedia)
7182) Dunkeld (+'s - name - Wikipedia)
7183) Letchworth (+'s - name - Wikipedia)
7184) Earley (+'s - name - Wikipedia)
7185) Woodley (+'s - name - Wikipedia)
7186) Ashgate (+'s - name - Wikipedia)
7187) Kirkwall (+'s - name - Wikipedia)
7188) credentialed + credentialing (merged into credential)
7189) irreverential
7190) Alfie (+'s - name - Wikipedia)
7191) irresponsive
7192) irresponsiveness
7193) non-Aboriginal
7194) non-achiever (+plural)
7195) dehydrocholesterol
7196) cholecalciferol
7197) IU (abbreviation: International Unit)
7198) decimetric
7199) centimetric (Collins)
7200) clast (+plural)
7201) clathrate (+plural)
7202) Blackmore (+'s - name - Wikipedia)
7203) buckytube (+plural)
7204) electrolytical
7205) electroluminescence
7206) immunoassays (added plural)
7207) Menindee (name - Wikipedia)
7208) Macalister (+'s - name - Wikipedia)
7209) Avebury (+'s - name - Wikipedia)
7210) Slieve (name - Wikipedia)
7211) encage (+s +ing +ed)
7212) CFOs (added plural)
7213) antivenene (another way for antivenin)
7214) antivenom (+plural)
7215) Somersby (+'s - name - Wikipedia)
7216) Brownwood (+'s - name - Wikipedia)
7217) Britt (+'s - name - Wikipedia)
7218) antitypical
7219) Prynne (+'s - name - Wikipedia)
7220) anti-inflammatory
7221) antiparasitic (Collins)
7222) anti-intellectual (+plural)
7223) anti-infective
7224) alarmism
7225) Caton (+'s - name - Wikipedia)
7226) unscriptural
7227) salvationist
7228) Salvationist (+plural)
7229) salvationism
7230) Luther's (added the 's - name - Wikipedia)
7231) consubstantial
7232) consubstantiality
7233) Tillard (+'s - name - Wikipedia)
7234) Christology
7235) Christological
7236) Christologically
7237) Bonhoeffer (+'s - name - Wikipedia)
7238) Mariology
7239) Mariological
7240) Mariologist (+plural)
7241) Montfort (+'s - name - Wikipedia)
7242) Cranmer (+'s - name - Wikipedia)
7243) upvote (+s +ing +ed)
7244) downvote (+s +ing +ed)
7245) raptorial
7246) raptorially
7247) avifauna
7248) avifaunal
7249) biomaterials (added plural)
7250) Hampden (+'s - name - Wikipedia)
7251) grapeseed (grapeseed oil)
7252) oaked
7253) monographic
7254) Gifford's (added the 's - name)
7255) Patricia's (added the 's - name)
7256) Jenkins's (added the 's - name)
7257) historiographic
7258) historiographically
7259) situationally
7260) Kildavin (name - Wikipedia)
7261) Stanmore (+'s - name - Wikipedia)
7262) Howth (+'s - name - Wikipedia)
7263) situates (merged into situate)
7264) dumpsite (Collins)
7265) Eudora (+'s - name - Wikipedia)
7266) Pepsi (+plural - Wiktionary)
7267) prefetch (+s +ing +ed)
7268) prefigurative
7269) prefigurement
7270) prefigurations (added plural)
7271) anymore (removed since it is chiefly North American)
7272) blocklist (+plural - Wiktionary)
7273) censorware (Wiktionary)
7274) killfile (+s +ing +ed - Wiktionary)
7275) squee (+s +ing +ed - Oxford: informal)
7276) squeegeed (merged into squeegee)
7277) brookweed
7278) loosestrife
7279) alstroemeria
7280) waltzer (+plural)
7281) dogdom
7282) doggish
7283) doglike
7284) hottie (+plural)
7285) hotty (another way of previous word)
7286) Pavarotti (+'s - name - Wikipedia)
7287) hotters (added plural)
7288) dorsum
7289) dorsa (plural of previous word)
7290) dorsolateral
7291) dorsolaterally
7292) cuticular
7293) abaxial
7294) adaxial
7295) micrographs (added plural)
7296) micrographic
7297) micrographics
7298) microgranite
7299) hydroxyproline (Collins)
7300) microgeneration
7301) microgenerate
7302) co-founders (added plural)
7303) co-found (+ed +ing)
7304) co-education
7305) co-educational
7306) homopolar
7307) tocopherol (+plural)
7308) synergetic
7309) synergic
7310) metaphorist (Collins)
7311) euroterminal (Collins)
7312) starshine (Collins)
7313) anaphrodisiac (+plural)
7314) decalcify (+s +ing +ed)
7315) decalcifier
7316) decalcification
7317) remineralize (+s +ing +ed -IZE)
7318) remineralise (+s +ing +ed -ISE)
7319) remineralization (-IZE)
7320) remineralisation (-ISE)
7321) osteoclast (+plural +ic)
7322) resorb (+s +ing +ed)
7323) resonation
7324) rhetor (+plural)
7325) interactional
7326) retroact (+s +ing +ed)
7327) retroaction
7328) scrollwork
7329) coachroofs (added plural)
7330) redrill (+s +ing +ed - Wiktionary)
7331) foredecks (added plural)
7332) soundcheck (+plural)
7333) soundbar (+plural)
7334) soundalike (+plural)
7335) neep (+plural)
7336) Soundex
7337) soundhole (+plural)
7338) soundclash
7339) dance-off (+plural)
7340) dancercise (-ISE)
7341) dancercize (-IZE)
7342) Discman (Collins)
7343) alveolate
7344) conjunctivae (another plural of conjunctiva)
7345) pericarditis
7346) lymphangitis
7347) subspecialist (+plural - Wiktionary)
7348) non-functioning
7349) non-gendered
7350) non-genetic
7351) non-genetically
7352) non-hostile
7353) biobank (+plural)
7354) biobibliography (+plural)
7355) hydrobiology
7356) biocentrism
7357) biocentric
7358) biocentrist (+plural)
7359) cosmopolis
7360) Heliopolis (name)
7361) spangly (+ier +iest)
7362) blackface
7363) demassify (+s +ing +ed +ion)
7364) dematerialize (+s +ing +ed +ion -IZE)
7365) dematerialise (+s +ing +ed +ion -ISE)
7366) attestable
7367) attestor (+plural)
7368) preparer (+plural)
7369) carboxylic (carboxylic acid)
7370) ethanoic (ethanoic acid)
7371) methanoic (methanoic acid)
7372) methanogenesis
7373) Horace's (added the 's - name - Wikipedia)
7374) badger-baiting
7375) beagler (+plural)
7376) lurchers (added plural)
7377) Higham (+'s - name - Wikipedia)
7378) underfeed (+ing)
7379) droid (+plural)
7380) Bharat (+'s - name)
7381) Bharatanatyam
7382) Bhavnagar (+'s - name)
7383) Antalya (+'s - name)
7384) pharmacotherapy
7385) clonidine (Collins)
7386) reserpine
7387) hydralazine (Collins)
7388) Edith's (added the 's - name - Wikipedia)
7389) firewall's (added the 's)
7390) BSE (Abbreviation: Bovine Spongiform Encephalopathy)
7391) spongiform
7392) redwater
7393) babesiosis
7394) piroplasmosis
7395) pirogue (+plural)
7396) pirk (+plural)
7397) cardoon (+plural)
7398) Calc (OpenOffice/LibreOffice)
7399) chit-chat (+s +ing +ed)
7400) Danielle (+'s - name - Wikipedia)
7401) nitty-gritty (Oxford: informal)
7402) nitrox
7403) Machmeter
7404) Canopus (name in space)
7405) Nibiru (+'s - name of Planet X - Wikipedia)
7406) Aldebaran (name in space)
7407) arctophile (+plural)
7408) arctophilia
7409) arctophilist
7410) arctophily
7411) peckish (Oxford: informal)
7412) Pecksniffian
7413) Dodge (+'s - name - Wikipedia)
7414) pecorino (+plural)
7415) non-medical
7416) EU (added the 's - abbreviation of European Union)
7417) Eu (chemical element)
7418) kidology (Oxford: informal)
7419) iridology
7420) iridologist (+plural)
7421) naturopathy
7422) naturopath (+plural)
7423) naturopathic
7424) naturopathically


V2.28 - 1-SEP-2015
------------------
7425) sloganeer (+plural +ing)
7426) shutdowns (added plural)
7427) slogger (+plural)
7428) Dyson (+'s - name - Wikipedia)
7429) Bronson (+'s - name - Wikipedia)
7430) escargots (added plural)
7431) Otago (+'s - name - Wikipedia)
7432) Margot (+'s - name - Wikipedia)
7433) escapologist (+plural)
7434) ASBO (+plural)
7435) guardee (Oxford: informal)
7436) Denbigh's (added the 's - name - Wikipedia)
7437) Demosthenes
7438) Demosthenic
7439) Dillon's (added the 's - name - Wikipedia)
7440) antiscience (Collins)
7441) technophile (added singular)
7442) technophilia
7443) technophilic
7444) technophobe (+plural)
7445) technopreneur (+plural)
7446) technopreneurial
7447) technospeak
7448) inboxes (added plural)
7449) newspeak
7450) textspeak
7451) forepeak (+plural)
7452) bobsleigh (fixed: bob-sleigh)
7453) Rudman (+'s - name - Wikipedia)
7454) Oxfam
7455) coevality
7456) citriculture
7457) fruticulture (Wiktionary)
7458) orphanism (Collins)
7459) Zephaniah (+'s - name)
7460) suspire
7461) suspiration
7462) subsumable
7463) subsumption
7464) potentiated (merged into potentiate)
7465) biologics (added plural)
7466) monotherapy
7467) diplopia
7468) photophobia
7469) photophobic
7470) uveitis
7471) exophthalmos + exophthalmus + exophthalmia
7472) hepatomegaly
7473) oncologic (Collins)
7474) vomeronasal (Collins)
7475) chemoreceptor (+plural)
7476) chemoreception
7477) bugless (Wiktionary)
7478) unpublish (+s +ing +ed - Wiktionary)
7479) relearnt
7480) epitope (+plural)
7481) epitomic
7482) epitomist
7483) epithelia (plural of epithelium)
7484) mesothelium
7485) mesothelia (plural of previous word)
7486) mesothelial
7487) antrum
7488) antra (plural of previous word)
7489) rhinoplasty (+plural +ic)
7490) gastrostomy (+plural)
7491) cystectomy (+plural)
7492) tetraplegic's (added the 's)
7493) clitoridectomy (+plural)
7494) craniotomy (+plural)
7495) drachmae (another plural of drachma)
7496) Draco (+'s - name)
7497) draconic
7498) Bram (+'s - name - Wikipedia)
7499) mortadella
7500) expellee (+plural)
7501) expeller (+plural)
7502) changelog (+plural - Wiktionary)
7503) OpenSolaris
7504) OpenBSD
7505) autocompletion (Wiktionary)
7506) autodiscovery (Wiktionary)
7507) autodetect (+s +ing +ed - Wiktionary)
7508) autodetection (Wiktionary)
7509) autofit (+s +ing +ed - Wiktionary)
7510) autoformat (+s +ing +ed - Wiktionary)
7511) BSOD (abbreviation: blue screen of death - Wikipedia)
7512) enqueuing + enqueueing
7513) enprint (+plural)
7514) autowind (+s +ing +er +ers)
7515) autowound (past of previous word)
7516) electroconvulsive
7517) electrochemists (added plural)
7518) Hagelstein (+'s - name - Wikipedia)
7519) Borazon (Oxford: trademark)
7520) elastase
7521) eldritch
7522) Eleatic (+plural)
7523) Zeno (+'s - name)
7524) Xenophanes (+'s - name)
7525) Bonapartism
7526) Bonapartist
7527) crowd-pleaser (+plural)
7528) Pelagius (name)
7529) Pelagian (+plural)
7530) Pelagianism
7531) airplane (removed - Chiefly American - GB is: aeroplane)
7532) microglia
7533) microglial
7534) gliosis (Collins)
7535) perivascular
7536) leptomeninges
7537) leptomeningeal
7538) backmost
7539) sternway
7540) cross-link (+s +ing +ed)
7541) cross-linkage
7542) psyllium
7543) psychrophile (+plural)
7544) psychrophilic
7545) thermophile (+plural)
7546) thermophilic
7547) pneumonitis
7548) pneumogastric
7549) pneumoencephalography (+ic)
7550) giantism
7551) acromegaly (+ic)
7552) polycystic
7553) anovulation (Collins)
7554) diverticulosis
7555) divertissement (+plural)
7556) Surtees (+'s - name)
7557) Montreal's (added the 's - name)
7558) Mandelson (+'s - name - Wikipedia)
7559) Criswell (+'s - name - Wikipedia)
7560) scintillatingly
7561) scintigram (+plural)
7562) revaccinate (+s +ing +ed +ion)
7563) Glaister (+'s - name - Wikipedia)
7564) Beckenham (+'s - name - Wikipedia)
7565) elicitor (+plural)
7566) emmenagogue (+plural)
7567) vermifuge
7568) abortifacient
7569) AZT (Oxford: trademark)
7570) ghostbuster (+plural)
7571) speech-writer (+plural)
7572) affordably
7573) lordless
7574) disafforest (+s +ing +ed +ion)
7575) Tweedy (+'s - name - Wikipedia)
7576) Hughes's (added the 's - name - Wikipedia)
7577) disaffirm (+s +ing +ed +ion)
7578) reaggregate (Collins)
7579) unhandled
7580) unhandsome
7581) unhang
7582) unhung (past of previous word)
7583) unhappen
7584) unsearched
7585) unrevealed
7586) SDRAM (Wiktionary)
7587) VRAM (Wiktionary)
7588) NVRAM (+plural - Wiktionary)
7589) RAMs (added plural - Wiktionary)
7590) ROMs (added plural - Wiktionary)
7591) initialism (+plural)
7592) initiand (+plural)
7593) macrodiagonal (Collins)
7594) macrocopy (+plural - Collins)
7595) macrodome (Collins)
7596) antenatally
7597) rostering + rostered (merged into roster)
7598) Kramer's (added the 's - name - Wikipedia)
7599) Melba's (added the 's - name - Wikipedia)
7600) canterbury (+plural - noun - don't confuse with the name)
7601) armoire (+plural)
7602) ropework (Collins)
7603) Tenebrae
7604) Brozman (+'s - name - Wikipedia)
7605) CWO (abbreviation: Chief Warrant Officer)
7606) Cwmbran (+'s - name - Wikipedia)
7607) babyfather (+plural)
7608) baby-faced
7609) baby-doll
7610) patchouli
7611) ylang-ylang
7612) ilang-ilang (another way of previous word)
7613) Yizkor (+plural)
7614) Yishuv
7615) Betjeman (+'s - name)
7616) twitterer (+plural)
7617) twittery
7618) flitter (+s +ing +ed)
7619) twittish
7620) twistor
7621) eigenfrequency (+plural)
7622) Sangiovese
7623) chaptalization (-IZE)
7624) chaptalisation (-ISE)
7625) chaptalize (+s +ing +ed -IZE)
7626) chaptalise (+s +ing +ed -ISE)
7627) Chaplinesque
7628) cryptozoology
7629) cryptozoological
7630) cryptozoologist (+plural)
7631) cryptosporidium
7632) cryptosporidia
7633) coccidia
7634) coccidian
7635) toxoplasma
7636) utero (in utero)
7637) goldeneye (+plural)
7638) alveolars (added plural)
7639) circumcisers (added plural)


V2.29 - 1-OCT-2015
------------------
7640) Emmental + Emmenthal
7641) Brie
7642) ciabatta (+plural)
7643) flatbreads (added plural)
7644) pita (removed as it was American)
7645) pitta (+plural - it is a bread and a bird)
7646) bap (+plural)
7647) Redditor (+plural)
7648) Reddit
7649) subreddit (+plural)
7650) manspreading
7651) MacGyver (+'s - name)
7652) Brexit + Brixit
7653) awestricken (another way of awestruck)
7654) stagehand (fixed: stage-hand)
7655) CVT (abbreviation: Continuously Variable Transmission)
7656) Europass (European Union site)
7657) anything's (added the 's)
7658) everything's (added the 's)
7659) how'd (Wiktionary)
7660) how're (Wiktionary)
7661) can't've (Wiktionary)
7662) couldn't've (Wiktionary)
7663) how've (Wiktionary)
7664) might've (Wiktionary)
7665) mightn't've (Wiktionary)
7666) nothing's (added the 's)
7667) shouldn't've (Wiktionary)
7668) there're (Wiktionary)
7669) this'd (Wiktionary)
7670) wouldn't've (Wiktionary)
7671) arses + arsing (merged into arse)
7672) half-arse (+ing +ed)
7673) anxiolytic (+plural)
7674) antiepileptics (added plural - Wiktionary)
7675) clonazepam
7676) hydrocodone (+plural - Wiktionary)
7677) MAOI
7678) naproxen
7679) oxycodone
7680) SSRI
7681) sublingual (+ally)
7682) ADHD (abbreviation: Attention deficit hyperactivity disorder)
7683) BPD
7684) CBT (abbreviation: Cognitive behavioural therapy)
7685) cyclothymia
7686) cyclothymic
7687) dysmorphia
7688) dysmorphic
7689) dysphoria
7690) dysphoric
7691) dysthymia
7692) dysthymic
7693) NPD
7694) suicidality
7695) backtick (+plural)
7696) configurability (Google)
7697) display's (added the 's)
7698) ECMAScript (+'s - Wiktionary)
7699) microSD (Wikipedia)
7700) programmatically
7701) symlink (+s +ing +ed)
7702) throbber (+plural)
7703) untick (+s +ing +ed - Wiktionary)
7704) transclude (+s +ing +ed - Wiktionary)
7705) wiki's (added the 's)
7706) blog's (added the 's)
7707) blogger's (added the 's)
7708) griefing + griefed (merged into grief - Wiktionary)
7709) griefer (+plural +'s)
7710) hiragana
7711) kana
7712) kanji
7713) katakana
7714) classism
7715) classist (+plural)
7716) heteronormative
7717) heteronormativity
7718) heterophobe (+plural - Wiktionary)
7719) Hopkins's (added the 's - name - Wikipedia)
7720) heterophobia (Wiktionary)
7721) heterophobic (Wiktionary)
7722) heterosexism
7723) heterosexist
7724) intersectionality
7725) intersectional
7726) intersectionalism (Wiktionary)
7727) misandrist (+plural)
7728) patriarchically
7729) transphobe (+plural - Wiktionary)
7730) transphobia
7731) transphobic
7732) aromantic (+plural)
7733) aromanticism (Wiktionary)
7734) demisexual (+plural + ity - Wiktionary)
7735) demisexuality (Wiktionary)
7736) heteroflexible (Collins)
7737) agender
7738) teen's (added the 's - Wikipedia)
7739) androgyne (+plural)
7740) androgynously
7741) bi-gender
7742) bi-gendered
7743) cis
7744) substituent (+plural)
7745) methoxyl
7746) cisgender
7747) cisgendered
7748) gender-fluid
7749) genderqueer (+plural)
7750) trans
7751) autotune (+s +ing +ed +er +ers - merged into autotune)
7752) cancellable (Collins)
7753) miscategorise (+s +ing +ed - Wiktionary)
7754) epilation
7755) epilate (+s +ing +ed)
7756) epilator (+plural)
7757) isopropyl
7758) sequitur (+plural - non sequitur)
7759) trinary (Collins)
7760) unfulfillable
7761) unfulfilling
7762) veganism
7763) fruitarian (+plural)
7764) fruitarianism
7765) non-American
7766) unpair (+s +ing +ed - Wiktionary)
7767) linting (merged into lint - Collins)
7768) genericise (+s +ing +ed - Wiktionary)
7769) geneticize (-IZE)
7770) geneticise (-ISE)
7771) geneticism
7772) repolarization
7773) repolish (+s +ing +ed)
7774) reanalyse (+s +ing +ed)
7775) intraindividual (Wiktionary)
7776) denaturation
7777) photovoltage (Wiktionary)
7778) oftenness
7779) Oftel + OFTEL
7780) superphylum (Collins)
7781) Asteroidea
7782) Ophiuroidea
7783) crossopterygian (+plural)
7784) burrfish (+plural)
7785) catfight (+s +ing)
7786) Felix's (added the 's - name)
7787) baddy (another way of baddie)
7788) Darnley (+'s - name)
7789) azidothymidine
7790) zidovudine
7791) thymidine
7792) nucleoside (+plural)
7793) miscreate (+s +ing +ed)
7794) miscreative
7795) Germain (+'s - name - Wikipedia)
7796) that've (Wiktionary)
7797) there've (Wiktionary)
7798) which've (Wiktionary)
7799) unsupportive
8800) kibibit (+plural - IEC - Wikipedia)
8801) mebibit (+plural - IEC - Wikipedia)
8802) gibibit (+plural - IEC - Wikipedia)
8803) tebibit (+plural - IEC - Wikipedia)
8804) pebibit (+plural - IEC - Wikipedia)
8805) exbibit (+plural - IEC - Wikipedia)
8806) zebibit (+plural - IEC - Wikipedia)
8807) yobibit (+plural - IEC - Wikipedia)
8808) kibibyte (+plural - IEC - Wikipedia)
8809) mebibyte (+plural - IEC - Wikipedia)
8810) gibibyte (+plural - IEC - Wikipedia)
8811) tebibyte (+plural - IEC - Wikipedia)
8812) pebibyte (+plural - IEC - Wikipedia)
8813) exbibyte (+plural - IEC - Wikipedia)
8814) zebibyte (+plural - IEC - Wikipedia)
8815) yobibyte (+plural - IEC - Wikipedia)
8816) anonymiser (+plural - Wiktionary)
8817) backtrace (+s +ing +ed - Wiktionary)
8818) bashism (+plural - Wiktionary)
8819) bi-endian (Wiktionary)
8820) big-endian
8821) little-endian
8822) bytecode (+plural - Wiktionary)
8823) misprice (+s +ing +ed)
8824) noncallable
8825) classful (Wiktionary)
8826) cron
8827) draggability (Wiktionary)
8828) draggable (Wiktionary)
8829) daemonize (+s +ing +ed - Wiktionary)
8830) diff (+s +ing +ed)
8831) endian
8832) idempotence
8833) ‎idempotency
8834) metasyntactic (Wiktionary)
8835) nullable
8836) opcode (+plural)
8837) pageable (Wiktionary)
8838) parallelize (+s +ing +ed +ion -IZE)
8839) parallelise (+s +ing +ed +ion -ISE)
8840) performant (Wiktionary)
8841) procuracy (+plural)
8842) proxying + proxied (merged into proxy - Wiktionary)
8843) stateful
8844) Marston (+'s - name - Wikipedia)
8845) bedplate
8846) templating + templated (Wiktionary)
8847) tokenisation (Wiktionary)
8848) tokeniser (+plural - Wiktionary)
8849) webmail's (added the 's)
8850) unmount (+s +ing +ed)
8851) noninfringement (Collins)
8852) nonrefoulement
8853) refoulement
8854) unmourned
8855) unwarned
8856) unwatchable
8857) resurrectable
8858) paralexia
8859) paralexic
8860) audiotaping + audiotaped (merged into audiotape)
8861) Lenin's (added the 's - name - Wikipedia)
8862) presentism
8863) presentist
8864) presentimental
8865) presentness
8866) presentor
8867) fleuron (+plural)
8868) mouchette
8869) mouchard
8870) psycho-educational
8871) psycho-aesthetic (+plural)
8872) psychicist
8873) psychist
8874) psychicism
8875) unprovided
8876) unprovability
8877) unproud
8878) unprotrusive
8879) unprotesting
8880) unprotestingly
8881) unresting
8882) unrestingly
8883) unrestorable
8884) CLI (abbreviation: Command Line Interface)
8885) Ci (abbreviation: Curie)
8886) CI (abbreviation: several meanings)
8887) minify (+s +ing +ed +ion)
8888) transpile (+s +ing +ed +er +ers - Wiktionary)
8889) transcompile (+s +ing +ed - Wiktionary)
8890) setup (setup file - Collins)
8891) technocentric
8892) technocomplex
8893) technocratically
8894) managerialism
8895) managerialist
8896) technofear
8897) technofreak
8898) technographer
8899) technography
8900) technologism
8901) technomaniac
8902) technomania
8903) technomic
8904) technopark
8905) technopolis
8906) technopolitan
8907) technoscience
8908) technoscientist
8909) technosphere
8910) technostress
8911) technostructures (added plural)
8912) technotronic + technetronic
8913) combatting + combatted (another way of combating and combated)
8914) cross-hair (removed, Oxford says it is two words)
8915) debounce (+s +ing +ed - Wiktionary)
8916) yuck + yuk (Oxford: informal)
8917) gyro (+plural)
8918) repo (+plural - source code repository - Wiktionary)
8919) unopinionated
8920) useable (another way of usable)
8921) unicity
8922) thusly (Oxford: informal)
8923) spandex
8924) buildable (Collins)
8925) buildability (Wiktionary)
8926) primatial
8927) dilatancy
8928) sulphonate (+plural +ing +ed +ion)
8929) sulfonamide (+plural - another way of sulphonamide)
8930) lovebird (+plural - fixed: love-bird)
8931) she-god
8932) she-goat
8933) she-friend
8934) she-dog
8935) she-devil
8936) she-dragon
8937) she-cat
8938) she-ass
8939) Jacopo (+'s - name)
8940) caryopsis
8941) caryopses (plural of previous word)
8942) digitate
8943) digitation
8944) behaviouralists (added plural)
8945) underbred
8946) bioterrorists (added plural)
8947) ironworking
8948) glasswork (+plural - Collins)
8949) Whitehouse (+'s - name - Wikipedia)
8950) Serenity (+'s - name - Wikipedia)
8951) serenities (plural of serenity)
8952) Pinochet (+'s - name - Wikipedia)


V2.30 - 1-NOV-2015
------------------
8953) affordance
8954) roofline (+plural)
8955) ideating + ideated (merged into ideate)
8956) repurpose (+s +ing +ed)
8957) collotype (+plural)
8958) Jones's (added the 's - name - Wikipedia)
8959) biosensor (+plural)
8960) radiotracer
8961) microsphere (+plural)
8962) radiotoxicity
8963) radiotoxic
8964) radiothorium
8965) radiothermal
8966) radiotherapeutically
8967) radiotelex
8968) radioteletype
8969) radiotelemetry
8970) radiotelegraphically
8971) radiosurgery
8972) radiosurgical
8973) radiostrontium
8974) radiosterilized
8975) radiosterilization
8976) radiostereometric
8977) radiosondage
8978) radiosity
8979) radiosensitizer
8980) radiosensitizing
8981) radiosensitization
8982) radiosensitize
8983) radiosensitivity
8984) radiosensitive (+ness)
8985) radioscanning
8986) radioresistant
8987) radioreceptor
8988) radiolabelled
8989) radioland
8990) radiolaria
8991) Sipuncula
8992) sipunculan (+plural)
8993) sipunculid (+plural)
8994) Sipunculus
8995) Coverley (+'s - name - Wikipedia)
8996) Harvey's (added the 's - name - Wikipedia)
8997) colourblock (+ing +ed)
8998) tankini (+plural)
8999) tankodrome
8000) tankette
9001) tankbuster (Oxford: informal)
9002) airgraph
9003) encaustic
9004) Gogh (+'s - name - Wikipedia)
9005) pastegrain
9006) pasteboard-like
9007) headbay
9008) headborough
9009) headcams (added plural)
9010) headbutts (added plural)
9011) Fairfax's (added the 's - name - Wikipedia)
9012) royalism
9013) parliamentarianism
9014) Aquarian (+plural)
9015) libra
9016) librae (plural of previous word)
9017) libidinally
9018) object-libido
9019) libertyless
9020) LIBOR
9021) amylose
9022) crystallizable (-IZE)
9023) crystallisable (-ISE)
9024) unbranched (Collins)
9025) amylopectin
9026) valproic (valproic acid)
9027) Antisthenes (name)
9028) trisectors (added plural)
9029) conchoid
9030) Nicomedes (+'s - name - Wikipedia)
9031) pappus
9032) pappi (plural of previous word)
9033) pappose
9034) Pappus (+'s - name - Wikipedia)
9035) pappy (+plural)
9036) poppyseeds (added plural)
9037) poppywort
9038) popsock (+plural)
9039) Steckling’s (added the 's - name)
9040) Clancarty (+'s - name - Wikipedia)
9041) co-trimoxazole
9042) doxycycline
9043) minocycline
9044) ciprofloxacin (Collins)
9045) sulphamethoxazole + sulfamethoxazole
9046) trimethoprim
9047) trimeter (+plural)
9048) trimetric
9049) trimetrical
9050) trisyllables (added plural)
9051) trisyllabic
9052) accentualist
9053) accentuality
9054) accentor (+plural)
9055) waxbill (+plural)
9056) congeneric
9057) congenerous
9058) primogenital
9059) primogenitary
9060) primogenitive
9061) co-ally (+ed)
9062) co-belligerent (+plural)
9063) co-belligerence
9064) co-branded
9065) co-branding
9066) co-conspirator (+plural)
9067) co-chair (+plural)
9068) co-counselling
9069) Balliol (+'s - name - Wikipedia)
9070) Whitworth
9071) Pitt (+'s - name - Wikipedia)
9072) Pitressin
9073) pitpan (+plural)
9074) touchlines (added plural)
9075) bookaholic (Oxford: informal)
9076) blogrolls (added plural)
9077) noseband
9078) cavesson (+plural)
9079) biostratigrapher (+plural)
9080) biostratigraphic
9081) biostratigraphical
9082) biostratigraphically
9083) macrofossil
9084) macrofungus
9085) macrogametocyte
9086) macrogamete
9087) flagellar
9088) epaxial
9089) unwellness
9090) aitchbone
9091) aitchless
9092) anterogradely
9093) claustrum
9094) claustra
9095) lentiform (lentiform nucleus)
9096) buccinator (+plural)
9097) Bucephalus (name)
9098) anacreontic (+plural)
9099) Anacreon (+'s - name - Wikipedia)
9100) essayistic
9101) recontinuance
9102) recontinue
9103) reconvalescence
9104) reconvalescent
9105) thicko (+plural - Oxford: informal)
9106) thicknesser (+plural)
9107) Castro's (added the 's - name - Wikipedia)
9108) IQs (added plural)
9109) intellectualistic
9110) intellectualist (+plural)
9111) voluntarist
9112) voluntaristic
9113) dogmatise (+s +ing +ed - ISE)
9114) dogmatize (+s +ing +ed - IZE)
9115) dognap (+s +ing +ed +er +ers)
9116) dogology
9117) dogship
9118) dogsleep
9119) dogstones
9120) televisionary (+plural)
9121) broadleaves
9122) broadloomed
9123) duplet (+plural)
9124) Burtenshaw (+'s - name - Wikipedia)
9125) unship (+s +ing +ed)
9126) unshockable
9127) unshockability
9128) unshrinkable
9129) unshrinkability
9130) unshrinkingly
9131) speargun
9132) spearfish (+plural +ing)
9133) lumpfish (+plural)
9134) lumpsucker (+plural)
9135) featherback (+plural)
9136) knifefish (+plural)
9137) jammer (+plural)
9138) woodturning
9139) woodturner (+plural)
9140) woodswallow (+plural)
9141) corella (+plural)
9142) wattlebird (+plural)
9143) geckoes (another plural of gecko)
9144) redox
9145) stibine (Collins)
9146) arsewipe (Oxford: slang)
9147) arsenopyrite
9148) wolframite
9149) axinite (Collins)
9150) apophyllite
9151) heulandite (Collins)
9152) sperrylite (Collins)
9153) thaumasite (Collins)
9154) fezzes (added plural)
9155) fezzed (merged into fez)
9156) blousy (+ier +iest - Collins)
9157) incommensurables (added plural)
9158) incommensurability
9159) incommensurably
9160) workpeople
9161) shewn (Collins)
9162) workperson
9163) workwoman
9164) workwise
9165) Forby (+'s - name - Wikipedia)
9166) componentry
9167) cuirass (+plural)
9168) chiliarch's (added the 's)
9169) supergiant (+plural)
9170) WordPress (+'s - name - Wikipedia)
9171) Verizon (+'s - name - Wikipedia)
9172) Comcast (+'s - name - Wikipedia)
9173) underappreciate (+ed +ion)
9174) tachyon (+plural)
9175) bosonic
9176) supergalactic
9177) transmural
9178) superfuse (+ed +ing)
9179) superfruit
9180) superfortress
9181) superforce
9182) superflux
9183) superfluorescent
9184) floristic (+plural)
9185) floristically
9186) barrenly
9187) barrelhead (+plural)
9188) barrelfish (+plural)
9189) achillea
9190) euphorbia (+plural)
9191) cistus
9192) lavatera
9193) buddleia
9194) accessorial
9195) possessory
9196) digresser (+plural)
9197) dihedrals (added plural)
9198) reflectional + reflexional
9199) reflectionless
9200) reflectogram (+plural)
9201) reflectography
9202) reflectograph
9203) reflectogramme
9204) coetaneous
9205) Castilian (+plural)
9206) pastureland (+plural)
9207) shrubland (+plural - Collins)
9208) cinematographically
9209) Dumont's (added the 's - name - Wikipedia)
9210) Bertolucci (+'s - name)
9211) ICANN (+'s - name - Wikipedia)
9212) DNSSEC (+'s - Wikipedia)
9213) WHOIS (Wikipedia)
9214) headrail (+plural)
9215) Kieran's (added the 's - name - Wikipedia)
9216) Euro-wasp
9217) Eurobank
9218) eurocredit (Collins)
9219) Eurocentrist
9220) IMF (+'s - Oxford: abbreviation: International Monetary Fund)


V2.31 - 1-DEC-2015
------------------
9221) Euroclear
9222) Euroconnector
9223) Euroconvertible
9224) blackcap (+plural)
9225) gonolek (+plural)
9226) shoebill (+plural)
9227) shoeboxes (added plural)
9228) cornmill
9229) ropemaker
9230) ropemanship
9231) ropesight
9232) campanulate
9233) umbo (+plural)
9234) umbones (another plural of previous word)
9235) umbonal
9236) umbonate
9237) invagination (+plural)
9238) invaginate (+ed)
9239) chondrocyte (+plural)
9240) chondrocranium
9241) tessera
9242) tesserae (plural of previous word)
9243) tesseral
9244) colourman
9245) outcross (+s +ing +ed)
9246) outcrying
9247) dependents (added plural)
9248) periodogram
9249) periodograph
9250) periodology
9251) endodontics (Collins)
9252) criminogenic
9253) reconvict (+s +ing +ed +ion)
9254) recopper
9255) reconvoke
9256) reconveyance
9257) reconvertible
9258) reconverge
9259) reconvention
9260) counterpetition (Collins)
9261) reconsult
9262) reconstructiveness
9263) reconstructionary
9264) reconstructional
9265) teleprocessing (Collins)
9266) tiddly (+ier +iest - Oxford: informal)
9267) Tippex + Tipp-Ex (British trademark)
9268) trendsetters (added plural)
9269) disrate (+s +ing +ed)
9270) snakefish (+plural)
9271) lizardfish (+plural)
9272) Gradle (+'s - name - Wikipedia)
9273) Percival (+'s - name - Wikipedia)
9274) Burnage (+'s - name - Wikipedia)
9275) Wexler (+'s - name - Wikipedia)
9276) picturable
9277) Thorson (+'s - name - Wikipedia)
9278) Fass (+'s - name - Wikipedia)
9279) zooplankton
9280) york (+s +ing +ed)
9281) zeolite (+plural +ic)
9282) zeroth
9283) zestfulness
9284) zesty (+ier +iest)
9285) wunderkind (+plural)
9286) wunderkinder (another plural of previous word)
9287) xylene
9288) trichloroethylene
9289) yaks (added plural)
9290) subwindow (+plural - Wiktionary)
9291) multiwindow (+ed)
9292) multiwire
9293) subindex (+plural - Collins)
9294) sub-idea
9295) subindustry (+plural - Collins)
9296) subimpose
9297) subincandescent
9298) subincision
9299) subinfeud
9300) subinfeudation + subenfeudation
9301) subinfeudatory
9302) subfeudatory + subfeudatary
9303) subfeudation
9304) subfertile
9305) subfertility
9306) subfauna
9307) subfactor
9308) subface
9309) subevergreen
9310) suberyl
9311) suberous
9312) suberose
9313) corky (+ier +iest)
9314) subproject (+plural - Collins)
9315) subpubic (Collins)
9316) analysation
9317) analysandum
9318) re-experience
9319) restenosis
9320) fibromuscular (Wiktionary)
9321) endarterectomy (+plural)
9322) cholecystectomy (+plural)
9323) laparoscopy (+plural)
9324) laparoscope
9325) laparoscopic
9326) laparoscopically
9327) laparotomy (+plural)
9328) splenectomy (+plural)
9329) asplenia (Wiktionary)
9330) Excel (+'s - Wikipedia)
9331) O'Farrell (+'s - name - Wikipedia)
9332) Kerensa (name - Wikipedia)
9333) XUL (Wiktionary: XML user interface language)
9334) changeset (+plural - Wiktionary)
9335) Daniel's (added the 's - name - Wikipedia)
9336) IIRC (Wiktionary: informal - If I remember correctly, if I recall correctly)
9337) Stehmann (name - Wikipedia)
9338) theophoric + theophorous
9339) toponymy + toponymics (Collins)
9340) topophilia (Collins)
9341) tapless
9342) tappable
9343) Ginsburg (+'s - name)
9344) panax
9345) underlet (+s +ing)
9346) quickset
9347) Odette (+'s - name)
9348) Debrett (+'s - name)
9349) Rhett (+'s - name - Wikipedia)
9350) comms
9351) commove (+s +ing +ed - Wiktionary)
9352) aidful
9353) unclearly
9354) unclearness
9355) uncleared
9356) sandfly (+plural)
9357) Wickepin (+'s - name - Wikipedia)
9358) Winton  (+'s - name - Wikipedia)
9359) Surman (+'s - name - Wikipedia)
9360) Berna (+'s - name - Wikipedia)
9361) Birk (+'s - name - Wikipedia)
9362) Witts (+'s - name - Wikipedia)
9363) Balrog (+'s - name - Wikipedia)
9364) Cloke (+'s - name - Wikipedia)
9365) xvi's (added the 's)
9366) cybersquatter (+plural)
9367) cybersurf
9368) cybertheft
9369) cyberthreat (+plural)
9370) cyberwarfare
9371) cyberwarrior (+plural)
9372) cyberworld
9373) cybrarian (+plural)
9374) Oshii (+'s - name - Wikipedia)
9375) Floyd's (added the 's - name - Wikipedia)
9376) Gilmour (+'s - name - Wikipedia)
9377) uninvested (Collins)
9378) Ezrin (+'s - name - Wikipedia)
9379) maximalist (+plural)
9380) maximalism
9381) rejectionist (+plural)
9382) ISIS (+'s - name of terrorist group - Wikipedia)
9383) EMI (+'s - name - Wikipedia)
9384) scenography
9385) scenographic
9386) Isis (added the 's - name - Wikipedia)
9387) Scenopegia
9388) Everly (+'s - name - Wikipedia)
9389) bandmates (added plural)
9390) freak-out (Oxford: informal)
9391) freakily
9392) freakiness
9393) weakish
9394) isospin
9395) axisymmetric
9396) apothem (+plural)
9397) coplanarity
9398) nonradiative (Wiktionary)
9399) degradability
9400) degradative
9401) ethylbenzene
9402) nitrobenzene
9403) nitrobacterium
9404) nitrify (+s +ing +ed)
9405) Cortana (+'s - name - Wikipedia)
9406) Ginnie (+'s - name - Wikipedia)
9407) Qualcomm (+'s - name - Wikipedia)
9408) datacard (Collins)
9409) databus (Collins)
9410) undogmatic
9411) undoable (Oxford: informal)
9412) undiversified
9413) undistracted
9414) undisputedly
9415) undissociated
9416) cycloaddition (+plural)
9417) interpolar (Collins)
9418) meristem (+plural)
9419) meristematic
9420) periderm
9421) peridermal
9422) phellogen
9423) Diemen (+'s - name)
9424) aerostatics
9425) macromutation
9426) micromutation
9427) micromotor
9428) micromorphology
9429) micromorphological
9430) micromorph (+plural)
9431) micromolar
9432) micromole (+plural)
9433) micromodule
9434) micromodelling + micromodeling
9435) micromodel
9436) microminiaturization + microminiaturisation
9437) photoelectronics
9438) Chambers (+'s - name - Wikipedia)
9439) photoelectromagnetic + photomagnetoelectric
9440) photomask
9441) pellicle
9442) pellicular
9443) interceptive
9444) multilayered (+ness)
9445) macrospore
9446) heterosporous (Collins)
9447) Protoctista
9448) protoctist (+plural)
9449) prokaryote + procaryote
9450) prokaryotic
9451) homologues (added plural)
9452) extrametrical (Collins)
9453) riffage
9454) illuminati
9455) Illuminati
9456) illuminism
9457) illuminist (+plural)
9458) catechumen (+plural)
9459) Eucharistical
9460) Euboea (name of place)
9461) eubacterial
9462) thunderless (Collins)
9463) pleximetry
9464) pleximeter
9465) plexor + plessor
9466) obturator (+plural)
9467) arthroscopy
9468) debridement
9469) debride (+ed)
9470) micro-environment (+plural)
9471) autecology
9472) autecological
9473) caatinga
9474) acetobacter
9475) mycotoxin (+plural)
9476) osteoderm (+plural)
9477) osteodontokeratic
9478) osteodermal
9479) osteodentine
9480) osteodystrophic
9481) osteodystrophy
9482) hyperparathyroidism
9483) hyperparathyroid
9484) hyperparasites (added plural)
9485) hyperphagia
9486) hypersomnia (Collins)
9487) hypersomnolence (Collins)
9488) postinflammatory
9489) postinfectious
9490) postinfection
9491) postillation
9492) postilioned
9493) buckhound (+plural)
9494) staghound (+plural)
9495) elkhound (+plural)
9496) Anglosphere
9497) Anglophony + Anglophonie
9498) Francophonie
9499) Francophonia
9500) disintegrative
9501) disintegrator (+plural)
9502) microcapsules (added plural)
9503) sprayable
9504) droppable
9505) starchitects (added plural)
9506) Baez (+'s - name)
9507) sexvirate
9508) sexuparous
9509) sexuate (+ed)
9510) Mondrian's (added the 's - name - Wikipedia)
9511) naloxone
9512) naltrexone
9513) oxycodone's (added the 's)
9514) opioids (added plural)
9515) radiopaque + radio-opaque
9516) radiopacity
9517) psionic (+plural)
9518) psionically
9519) psilothrum
9520) psilophyte
9521) psilomelane
9522) psilocybin
9523) serotonergic
9524) serotype (+plural)
9525) serotypic
9526) pneumococcus
9527) pneumococci (plural of previous word)
9528) pneumococcal
9529) basophilia
9530) nunchaku (+plural)
9531) Zyrian
9532) zythum
9533) hippogriff + hippogryph (+plural of both)
9534) manticore (+plural)
9535) mantic
9536) geomantical + geomantic


V2.32 - 1-JAN-2016
------------------
9537) Horatian
9538) explicator
9539) explicatory
9540) Palladian
9541) Palladianism
9542) Piccadilly
9543) Orcadian (+plural)
9544) Trumland (name)
9545) Caithness (+'s - name - Wikipedia)
9546) Rousay (+'s - name - Wikipedia)
9547) Shapinsay (+'s - name - Wikipedia)
9548) Oddie (+'s - name - Wikipedia)
9549) Govan (+'s - name - Wikipedia)
9550) Diane's (added the 's - name - Wikipedia)
9551) Lynch (+'s - name - Wikipedia)
9552) Selwyn's (added the 's - name - Wikipedia)
9553) Camilla's (added the 's - name - Wikipedia)
9554) overanalysis
9555) overanalyse (+s +ing +ed)
9556) diegesis
9557) diegeses (plural of previous word)
9558) diegetic
9559) Harring (+'s - name - Wikipedia)
9560) dieffenbachia (+plural)
9561) horticulturalist (+plural)
9562) unreviewed
9563) Wikipedians (added plural)
9564) unversioned (Wiktionary)
9565) autoconfirm (+s +ing +ed +ion - Wiktionary)
9566) vandalistic
9567) vandalistically
9568) HTML's (added the 's + uppercase - Wikipedia)
9569) wikiquette (Wiktionary)
9570) reasonability
9571) cherry-pick (+s +ing +ed)
9572) Reagle (+'s - name - Wikipedia)
9573) BlackBerry (+s +ing +ed)
9574) wolfberry (+plural)
9575) bloodletter
9576) bloodish
9577) bloodshedder
9578) tear-stained
9579) bloodshedding
9580) blepharitis
9581) keratitis
9582) iritis
9583) atherosclerotic
9584) atheroma
9585) atheromatous
9586) distensibility
9587) distensible
9588) haemodynamic
9589) haemodynamically
9590) haemodynamics
9591) photoreceptors (added plural)
9592) electromedical
9593) ferrites (added plural)
9594) ferritic
9595) martensite (+ic)
9596) martempering
9597) martempered
9598) martellato
9599) Martello (+plural)
9600) Napoleonism
9601) Napoleonically
9602) Napoleonist
9603) outrightly
9604) cutie (+plural - Oxford: informal)
9605) Marilyn's (added the 's - name - Wikipedia)
9606) geniculate
9607) antennule (+plural)
9608) concavo-convex
9609) prosome
9610) copepod (+plural)
9611) mantid (+plural)
9612) prothorax
9613) prothoracic
9614) prothonotaryship
9615) protonotary + prothonotary (+plural of both)
9616) protonosphere
9617) geocorona
9618) geocoronal
9619) geocratic
9620) geocyclic
9621) cadastral
9622) occurrent
9623) Washingtonian (+plural)
9624) Washingtonologist
9625) IPs (added plural - Wiktionary)
9626) homepage's (added the 's - Wikipedia)
9627) telepathing + telepathed (merged into telepath - Wiktionary)
9628) interdimensional (Wiktionary)
9629) Carrolton (name of place - Wikipedia)
9630) Galactica (+'s - name of TV series - Wikipedia)
9631) Okrand (+'s - name - Wikipedia)
9632) Ragnar (+'s - name - Wikipedia)
9633) Redbeard (+'s - name - Wikipedia)
9634) Kristy (+'s - name - Wikipedia)
9635) Schulman (+'s - name - Wikipedia)
9636) adverted + adverting (merged into advert)
9637) aftertaste (fixed: after-taste)
9638) albedos (added plural)
9639) aldehydes (added plural)
9640) Angelus + angelus
9641) Angelou (+'s - name - Wikipedia)
9642) Corbin (+'s - name - Wikipedia)
9643) apostolate (+plural)
9644) charism (+plural)
9645) constraining (merged into constrain)
9646) Smithers (+'s - name - Wikipedia)
9647) Dostoyevsky (+'s - name - Wikipedia)
9648) fusionist (+plural)
9649) fusionism
9650) Greenock (+'s - name - Wikipedia)
9651) Crawfurd (+'s - name - Wikipedia)
9652) Gourock (+'s - name - Wikipedia)
9653) greenlet
9654) greenless
9655) Greenlandman
9656) Greenlandish
9657) Greenlander (+plural)
9658) greenlandite + greenlandit
9659) mezzotint (+s +ing +ed +er +ers)
9660) Morland (+'s - name - Wikipedia)
9661) Glenn's (added the 's - name - Wikipedia)
9662) STI (+plural - Abbreviation: sexually transmitted infection)
9663) papillomavirus
9664) protozoal
9665) protozoon (joined with protozoon's)
9666) ureaplasma (+plural)
9667) mycoplasma (+plural)
9668) mycoplasmata (another plural of previous word)
9669) metronidazole
9670) polyisoprene
9671) isoprene
9672) presymptomatic (Collins)
9673) Salvarsan
9674) arsphenamine
9675) apple-cart (removed, Oxford says it is two words)
9676) auricula (+plural)
9677) avens
9678) Ayurveda
9679) Ayurvedic
9680) backbeat (+plural)
9681) balustraded
9682) baptize (+s +ing +ed - IZE)
9683) birching + birched (merged into birch)
9684) biscuity
9685) blackshirt (+plural - added lowercase)
9686) Jabotinsky (+'s - name - Wikipedia)
9687) Lisa's (added the 's - name - Wikipedia)
9688) Clarkson (+'s - name - Wikipedia)
9689) Dara (+'s - name - Wikipedia)
9690) Christiansen's (added the 's - name - Wikipedia)
9691) kick-boxing
9692) nutraceutical (+plural)
9693) neurobiological
9694) neurobiologist (+plural)
9695) neurobiochemistry
9696) neurobics
9697) neurobehavioural
9698) psychopharmacology
9699) psychopharmacological
9700) psychopharmacologist (+plural)
9701) Enigmail (+'s - name of Mozilla add-on)
9702) misenter (+s +ing +ed - Wiktionary)
9703) Canute (+'s - name - Wikipedia)
9704) rewrap (+s +ing +ed)
9705) logfile (+plural - Wiktionary)
9706) Josie (+'s - name - Wikipedia)
9707) posey (+ier +iest - Oxford: informal)
9708) mag (+plural - Oxford: informal)
9709) mimeo
9710) Berrigan (+'s - name - Wikipedia)
9711) detachability
9712) performative (+plural)
9713) performativity
9714) constative (+plural)
9715) polysiloxane
9716) phenylmethyl (Collins)
9717) GUI's (added the 's - Wikipedia)
9718) rebuffer (+s +ing +ed - Wiktionary)
9719) upsample (+s +ing +ed - Wiktionary)
9720) blockiness (Collins)
9721) Netflix’s (added the 's)
9722) downsample (+s +ing +ed - Wiktionary)
9723) choosable (Wiktionary)
9724) bootboy (+plural)
9725) bootie (added singular)
9726) outsole (+plural)
9727) Thinsulate (Oxford: trademark)
9728) minibreak (+plural)
9729) NASCAR (abbreviation: National Association for Stock Car Auto Racing)
9730) minicell
9731) minicar (+plural)
9732) minichain
9733) minichromosome
9734) minicircle
9735) kinetoplast (+plural)
9736) kinetoscope (+plural)
9737) Leysdown (Wikipedia - name of place)
9738) unrefuted
9739) microscopists (added plural)
9740) photocycle
9741) reorganizational
9742) reorganizationist
9743) reovirus
9744) enteric
9745) ante-mortem
9746) mediastinum
9747) mediastina (plural of previous word)
9748) mediastinal
9749) mediastinoscopy
9750) prereform
9751) postreform (Wiktionary)
9752) tragical
9753) bathypelagic
9754) octopod (+plural)
9755) blastocyst (+plural)
9756) blastula
9757) blastulae (plural of previous word)
9758) denticulation (+plural)
9759) septal
9760) atrioventricular
9761) endocardial
9762) parapophysis
9763) parapolitical
9764) parapolar
9765) parapod
9766) iOS's (added the 's - Wikipedia)
9767) polyamine
9768) polyamidation
9769) polyamide (+plural)
9770) adipic (adipic acid)
9771) adipoceration
9772) adipocere
9773) hypotensive
9774) normotensive
9775) vagus
9776) vagi (plural of previous word)
9777) ventifact (+plural)
9778) erraticism
9779) speech-making
9780) speech-maker (+plural)
9781) countermark (added +s +ing +ed)
9782) sublibrary (+plural - Wiktionary)
9783) subrectangular (Wiktionary)
9784) subsensible (Collins)
9785) suboption (+plural - Wiktionary)
9786) suborganization (+plural - Collins - IZE)
9787) suborganisation (+plural - Collins - ISE)
9788) suboval
9789) subovoid
9790) suboptimum
9791) suboperation (+plural - Wiktionary)
9792) Ascher (+'s - name - Wikipedia)
9793) Plohman (name - Wikipedia)
9794) Gervase (+'s - name - Wikipedia)
9795) Phipps (+'s - name - Wikipedia)
9796) PEP (+plural +'s - abbreviation: Political and Economic Planning)
9797) penectomy
9798) orchiectomy (+plural - Collins)
9799) orchidectomy (+plural - Collins)
9800) curette (+s +ing +ed)
9801) electrodesiccation
9802) osteotome (+plural)
9803) cerumen
9804) zoster (herpes zoster)
9805) photosensitivity
9806) urticaria
9807) omeprazole
9808) bing
9809) Bing (Microsoft)
9810) OneDrive (Microsoft)
9811) Silverlight (Microsoft)
9812) subparagraph (+plural - Collins)
9813) sublicensee (+plural - Wiktionary)
9814) sublicenses (added plural)
9815) Satya (+'s - name - Wikipedia)
9816) Nadella (+'s - name - Wikipedia)
9817) MSDN (Microsoft)
9818) Altair (+'s - name - Wikipedia)
9819) Bellevue (+'s - name - Wikipedia)
9820) rebrands + rebranded (merged into rebrand)
9821) Gates (+'s - name - Wikipedia)
9822) Ballmer (+'s - name - Wikipedia)
9823) Gartner (+'s - name - Wikipedia)
9824) CE
9825) Ce
9826) Klawe (+'s - name - Wikipedia)
9827) Moody (+'s - name - Wikipedia)
9828) write-down (+plural)
9829) TCO (abbreviation: Total cost of Ownership)
9830) videocasts (added plural)
9831) Issaquah (+'s - name - Wikipedia)
9832) anglicized + anglicised (fixed: it was in uppercase)
9833) Gilman (+'s - name - Wikipedia)
9834) regauge (+s +ing +ed)
9835) Sammamish (+'s - name - Wikipedia)
9836) regelate (+ed +ion)
9837) calorifically
9838) smokelessly
9839) tostada + tostado (+plural of both)
9840) tosylate (+plural)
9841) nonactivated (Collins)
9842) adrenergic
9843) histaminic
9844) muscarinic
9845) hyperreactivity (Collins)
9846) submucosal
9847) muscularis
9848) tunica
9849) tunicae (plural of previous word)
9850) insectile
9851) insectarium + insectary (+plural of both)
9852) avicularium
9853) avicularia (plural of previous word)
9854) Earlybird (Mozilla)
9855) woodman + woodmen
9856) woodsmoke
9857) workless (+ness)
9858) Wordsworthiana
9859) workpieces (added plural)
9860) bowie (+plural)
9861) bradycardia
9862) caliper (+plural)
9863) caparisoned
9864) carabiniere
9865) carabinieri (plural of previous word)
9866) carboxyl + carboxy
9867) casuals (added plural)
9868) catheterization (-IZE)
9869) catheterisation (-ISE)
9870) bacteriuria
9871) prostatitis
9872) pyelonephritis
9873) pyelonephritic
9874) pyuria
9875) haematuria
9876) cedarwood (Collins)
9877) ceilidhs (added plural)
9878) celeriac
9879) VLC (+'s - name of software)
9880) bidimensional (Wiktionary)
9881) tridimensional (Collins)
9882) four-dimensional
9883) coromandel
9884) lacquerware
9885) lacquerwork
9886) japan (+s +ing +ed - noun + verb)
9887) Jap (+plural - offensive - short for Japanese - Wiktionary)
9888) arse-licking (vulgar slang)
9889) arse-licker (vulgar slang)
9890) postlanding (Collins)
9891) beanfeast (+plural)
9892) believingly
9893) freezingly (Collins)
9894) bobbish (Collins - slang)
9895) breaktime (Collins)
9896) nanofibre (+plural)
9897) oxysulphide
9898) cashpoint (+plural)
9899) classmanship
9900) clicket (Collins)
9901) clubbable
9902) clubbability
9903) costermonger (+plural)
9904) cream-crackered
9905) decenary + decinary + decennary
9906) declinatory
9907) eyeable
9908) faceache (Oxford: informal)
9909) fandabidozi (Collins)
9910) Fenian (+plural)
9911) Fenianism
9912) fibrescope (+plural)
9913) flasket (Collins)
9914) flaysome (Collins)
9915) flyman
9916) flymen (plural of previous word)
9917) foretop (+plural)
9918) forjudge + forejudge
9919) formularise (+s +ing +ed)
9920) forspeak (Collins)
9921) freeload (+s +ing +ed +er +ers)
9922) fundholder (added singular)
9923) garibaldi (+plural)
9924) gastropub (+plural)
9925) gemminess
9926) gyratory (+plural)
9927) elevatory
9928) incubatory
9929) modificatory
9930) pulsatory
9931) clarificatory
9932) denigratory
9933) Obi (+'s - name - Wikipedia)
9934) Obi-Wan (+'s - name - Wikipedia)
9935) Kenobi (+'s - name - Wikipedia)
9936) Anakin (+'s - name - Wikipedia)
9937) Skywalker (+'s - name - Wikipedia)
9938) Jedi (+plural +'s)
9939) Sith (+'s - name - Wikipedia)
9940) McCallum (+'s - name - Wikipedia)
9941) Ewan (+'s - name - Wikipedia)
9942) Portman (+'s - name - Wikipedia)
9943) McDiarmid (+'s - name - Wikipedia)
9944) Lucasfilm (+'s - name - Wikipedia)
9945) workprint
9946) lightsaber (+plural)
9947) Coruscant (+'s - name - Wikipedia)
9948) Palpatine (+'s - name - Wikipedia)
9949) Padmé (+'s - name - Wikipedia)
9950) Colton (+'s - name - Wikipedia)
9951) Chewbacca (+'s - name - Wikipedia)
9952) Kamino (+'s - name - Wikipedia)
9953) refilm (+s +ing +ed - Wiktionary)
9954) animatic (+plural)
9955) pre-visualize + previsualise + previsualize
9956) Shepperton (+'s - name - Wikipedia)
9957) Elstree (+'s - name - Wikipedia)
9958) armlock (+plural)
9959) Gillard (+'s - name - Wikipedia)
9960) Oldman (+'s - name - Wikipedia)
9961) Smithee (+'s - name - Wikipedia)
9962) Grauman (+'s - name - Wikipedia)
9963) Rosenbaum (+'s - name - Wikipedia)
9964) outgross (+s +ing +ed)
9965) Narnia (+'s - name - Wikipedia)
9966) Elsey (+'s - name - Wikipedia)
9967) Gooley (+'s - name - Wikipedia)
9968) Razzie (+'s - name - Wikipedia)
9969) Windu (+'s - name - Wikipedia)
9970) Sidious (+'s - name - Wikipedia)
9971) Leia (+'s - name - Wikipedia)
9972) Han (+'s - name - Wikipedia)
9973) Jaina (+'s - name - Wikipedia)
9974) provene
9975) Bridger (+'s - name - Wikipedia)
9976) Katarn (+'s - name - Wikipedia)
9977) Qui-Gon (+'s - name - Wikipedia)
9978) Organa (+'s - name - Wikipedia)
9979) Jacen (+'s - name - Wikipedia)
9980) unlogical (Wiktionary)
9981) Twichell (+'s - name - Wikipedia)
9982) Corso (+'s - name - Wikipedia)
9983) McKinnon (+'s - name - Wikipedia)
9984) Tomkins (+'s - name - Wikipedia)
9985) Puckett (+'s - name - Wikipedia)
9986) Elmwood (+'s - name - Wikipedia)
9987) Medford (+'s - name - Wikipedia)
9988) Georgie (+'s - name - Wikipedia)
9989) Pedrick (+'s - name - Wikipedia)
9990) Bucksch (name - Wikipedia)
9991) Koelmeyer (name - Wikipedia)
9992) Grandon (+'s - name - Wikipedia)
9993) rammelly + rambly
9994) Fernhout (name - Wikipedia)
9995) xlsx
9996) UX (abbreviation: user experience)
9997) uxoricide
9998) uxoricidal
9999) uxoriously
10000) uxoriousness
10001) uxorilocal
10002) matrilocal
10003) matrilocality
===================================================
en_NZ:
I. Copyright
II. Copying (Licence)
----------------------------

I. Copyright

NZ English Dictionary v0.9 beta - Build 06SEP03
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
NB This is an initial version, please check:
http://lingucomponent.openoffice.org/download_dictionary.html
or
http://www.girlza.com/dictionary/download.html
for a final version, after a little while (no hurry).

This dictionary is based on the en_GB Myspell dictionary 
which in turn was initially based on a subset of the 
original English wordlist created by Kevin Atkinson for 
Pspell and  Aspell and thus is covered by his original 
LGPL licence. 


Introduction
~~~~~~~~~~~~
en_NZ.dic has been altered to include New Zealand places,
including major cities and towns, and major suburbs. It
also contains NZ words, organisations and expressions.

en_NZ.aff has had a few REPlace strings added, but is
basically unchanged.


Acknowledgements
~~~~~~~~~~~~~~~~
Thanks must go to the original creators of the British
dictionary, David Bartlett, Brian Kelk and Andrew Brown.

I wouldn't have started this without seeing the Australian
dictionary, thanks Kelvin Eldridge, Jean Hollis Weber and
David Wilson.

And thank you to all who've contributed to OpenOffice.org.


License
~~~~~~~
This dictionary is covered by the GNU Lesser General Public
License, viewable at http://www.gnu.org/copyleft/lesser.html


Issues
~~~~~~
Many of the proper nouns already in the dictionary do not have
an affix for 's.
All my new words start after the z's of the original dictionary.


Contact
~~~~~~~
Contact Tristan Burtenshaw (hooty@slingshot.co.nz) with any words,
places or other suggestions for the dictionary.



II. Copying

		  GNU LESSER GENERAL PUBLIC LICENSE
		       Version 2.1, February 1999

 Copyright (C) 1991, 1999 Free Software Foundation, Inc.
     59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 Everyone is permitted to copy and distribute verbatim copies
 of this license document, but changing it is not allowed.

[This is the first released version of the Lesser GPL.  It also counts
 as the successor of the GNU Library Public License, version 2, hence
 the version number 2.1.]

			    Preamble

  The licenses for most software are designed to take away your
freedom to share and change it.  By contrast, the GNU General Public
Licenses are intended to guarantee your freedom to share and change
free software--to make sure the software is free for all its users.

  This license, the Lesser General Public License, applies to some
specially designated software packages--typically libraries--of the
Free Software Foundation and other authors who decide to use it.  You
can use it too, but we suggest you first think carefully about whether
this license or the ordinary General Public License is the better
strategy to use in any particular case, based on the explanations below.

  When we speak of free software, we are referring to freedom of use,
not price.  Our General Public Licenses are designed to make sure that
you have the freedom to distribute copies of free software (and charge
for this service if you wish); that you receive source code or can get
it if you want it; that you can change the software and use pieces of
it in new free programs; and that you are informed that you can do
these things.

  To protect your rights, we need to make restrictions that forbid
distributors to deny you these rights or to ask you to surrender these
rights.  These restrictions translate to certain responsibilities for
you if you distribute copies of the library or if you modify it.

  For example, if you distribute copies of the library, whether gratis
or for a fee, you must give the recipients all the rights that we gave
you.  You must make sure that they, too, receive or can get the source
code.  If you link other code with the library, you must provide
complete object files to the recipients, so that they can relink them
with the library after making changes to the library and recompiling
it.  And you must show them these terms so they know their rights.

  We protect your rights with a two-step method: (1) we copyright the
library, and (2) we offer you this license, which gives you legal
permission to copy, distribute and/or modify the library.

  To protect each distributor, we want to make it very clear that
there is no warranty for the free library.  Also, if the library is
modified by someone else and passed on, the recipients should know
that what they have is not the original version, so that the original
author's reputation will not be affected by problems that might be
introduced by others.

  Finally, software patents pose a constant threat to the existence of
any free program.  We wish to make sure that a company cannot
effectively restrict the users of a free program by obtaining a
restrictive license from a patent holder.  Therefore, we insist that
any patent license obtained for a version of the library must be
consistent with the full freedom of use specified in this license.

  Most GNU software, including some libraries, is covered by the
ordinary GNU General Public License.  This license, the GNU Lesser
General Public License, applies to certain designated libraries, and
is quite different from the ordinary General Public License.  We use
this license for certain libraries in order to permit linking those
libraries into non-free programs.

  When a program is linked with a library, whether statically or using
a shared library, the combination of the two is legally speaking a
combined work, a derivative of the original library.  The ordinary
General Public License therefore permits such linking only if the
entire combination fits its criteria of freedom.  The Lesser General
Public License permits more lax criteria for linking other code with
the library.

  We call this license the "Lesser" General Public License because it
does Less to protect the user's freedom than the ordinary General
Public License.  It also provides other free software developers Less
of an advantage over competing non-free programs.  These disadvantages
are the reason we use the ordinary General Public License for many
libraries.  However, the Lesser license provides advantages in certain
special circumstances.

  For example, on rare occasions, there may be a special need to
encourage the widest possible use of a certain library, so that it becomes
a de-facto standard.  To achieve this, non-free programs must be
allowed to use the library.  A more frequent case is that a free
library does the same job as widely used non-free libraries.  In this
case, there is little to gain by limiting the free library to free
software only, so we use the Lesser General Public License.

  In other cases, permission to use a particular library in non-free
programs enables a greater number of people to use a large body of
free software.  For example, permission to use the GNU C Library in
non-free programs enables many more people to use the whole GNU
operating system, as well as its variant, the GNU/Linux operating
system.

  Although the Lesser General Public License is Less protective of the
users' freedom, it does ensure that the user of a program that is
linked with the Library has the freedom and the wherewithal to run
that program using a modified version of the Library.

  The precise terms and conditions for copying, distribution and
modification follow.  Pay close attention to the difference between a
"work based on the library" and a "work that uses the library".  The
former contains code derived from the library, whereas the latter must
be combined with the library in order to run.

		  GNU LESSER GENERAL PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

  0. This License Agreement applies to any software library or other
program which contains a notice placed by the copyright holder or
other authorized party saying it may be distributed under the terms of
this Lesser General Public License (also called "this License").
Each licensee is addressed as "you".

  A "library" means a collection of software functions and/or data
prepared so as to be conveniently linked with application programs
(which use some of those functions and data) to form executables.

  The "Library", below, refers to any such software library or work
which has been distributed under these terms.  A "work based on the
Library" means either the Library or any derivative work under
copyright law: that is to say, a work containing the Library or a
portion of it, either verbatim or with modifications and/or translated
straightforwardly into another language.  (Hereinafter, translation is
included without limitation in the term "modification".)

  "Source code" for a work means the preferred form of the work for
making modifications to it.  For a library, complete source code means
all the source code for all modules it contains, plus any associated
interface definition files, plus the scripts used to control compilation
and installation of the library.

  Activities other than copying, distribution and modification are not
covered by this License; they are outside its scope.  The act of
running a program using the Library is not restricted, and output from
such a program is covered only if its contents constitute a work based
on the Library (independent of the use of the Library in a tool for
writing it).  Whether that is true depends on what the Library does
and what the program that uses the Library does.
  
  1. You may copy and distribute verbatim copies of the Library's
complete source code as you receive it, in any medium, provided that
you conspicuously and appropriately publish on each copy an
appropriate copyright notice and disclaimer of warranty; keep intact
all the notices that refer to this License and to the absence of any
warranty; and distribute a copy of this License along with the
Library.

  You may charge a fee for the physical act of transferring a copy,
and you may at your option offer warranty protection in exchange for a
fee.

  2. You may modify your copy or copies of the Library or any portion
of it, thus forming a work based on the Library, and copy and
distribute such modifications or work under the terms of Section 1
above, provided that you also meet all of these conditions:

    a) The modified work must itself be a software library.

    b) You must cause the files modified to carry prominent notices
    stating that you changed the files and the date of any change.

    c) You must cause the whole of the work to be licensed at no
    charge to all third parties under the terms of this License.

    d) If a facility in the modified Library refers to a function or a
    table of data to be supplied by an application program that uses
    the facility, other than as an argument passed when the facility
    is invoked, then you must make a good faith effort to ensure that,
    in the event an application does not supply such function or
    table, the facility still operates, and performs whatever part of
    its purpose remains meaningful.

    (For example, a function in a library to compute square roots has
    a purpose that is entirely well-defined independent of the
    application.  Therefore, Subsection 2d requires that any
    application-supplied function or table used by this function must
    be optional: if the application does not supply it, the square
    root function must still compute square roots.)

These requirements apply to the modified work as a whole.  If
identifiable sections of that work are not derived from the Library,
and can be reasonably considered independent and separate works in
themselves, then this License, and its terms, do not apply to those
sections when you distribute them as separate works.  But when you
distribute the same sections as part of a whole which is a work based
on the Library, the distribution of the whole must be on the terms of
this License, whose permissions for other licensees extend to the
entire whole, and thus to each and every part regardless of who wrote
it.

Thus, it is not the intent of this section to claim rights or contest
your rights to work written entirely by you; rather, the intent is to
exercise the right to control the distribution of derivative or
collective works based on the Library.

In addition, mere aggregation of another work not based on the Library
with the Library (or with a work based on the Library) on a volume of
a storage or distribution medium does not bring the other work under
the scope of this License.

  3. You may opt to apply the terms of the ordinary GNU General Public
License instead of this License to a given copy of the Library.  To do
this, you must alter all the notices that refer to this License, so
that they refer to the ordinary GNU General Public License, version 2,
instead of to this License.  (If a newer version than version 2 of the
ordinary GNU General Public License has appeared, then you can specify
that version instead if you wish.)  Do not make any other change in
these notices.

  Once this change is made in a given copy, it is irreversible for
that copy, so the ordinary GNU General Public License applies to all
subsequent copies and derivative works made from that copy.

  This option is useful when you wish to copy part of the code of
the Library into a program that is not a library.

  4. You may copy and distribute the Library (or a portion or
derivative of it, under Section 2) in object code or executable form
under the terms of Sections 1 and 2 above provided that you accompany
it with the complete corresponding machine-readable source code, which
must be distributed under the terms of Sections 1 and 2 above on a
medium customarily used for software interchange.

  If distribution of object code is made by offering access to copy
from a designated place, then offering equivalent access to copy the
source code from the same place satisfies the requirement to
distribute the source code, even though third parties are not
compelled to copy the source along with the object code.

  5. A program that contains no derivative of any portion of the
Library, but is designed to work with the Library by being compiled or
linked with it, is called a "work that uses the Library".  Such a
work, in isolation, is not a derivative work of the Library, and
therefore falls outside the scope of this License.

  However, linking a "work that uses the Library" with the Library
creates an executable that is a derivative of the Library (because it
contains portions of the Library), rather than a "work that uses the
library".  The executable is therefore covered by this License.
Section 6 states terms for distribution of such executables.

  When a "work that uses the Library" uses material from a header file
that is part of the Library, the object code for the work may be a
derivative work of the Library even though the source code is not.
Whether this is true is especially significant if the work can be
linked without the Library, or if the work is itself a library.  The
threshold for this to be true is not precisely defined by law.

  If such an object file uses only numerical parameters, data
structure layouts and accessors, and small macros and small inline
functions (ten lines or less in length), then the use of the object
file is unrestricted, regardless of whether it is legally a derivative
work.  (Executables containing this object code plus portions of the
Library will still fall under Section 6.)

  Otherwise, if the work is a derivative of the Library, you may
distribute the object code for the work under the terms of Section 6.
Any executables containing that work also fall under Section 6,
whether or not they are linked directly with the Library itself.

  6. As an exception to the Sections above, you may also combine or
link a "work that uses the Library" with the Library to produce a
work containing portions of the Library, and distribute that work
under terms of your choice, provided that the terms permit
modification of the work for the customer's own use and reverse
engineering for debugging such modifications.

  You must give prominent notice with each copy of the work that the
Library is used in it and that the Library and its use are covered by
this License.  You must supply a copy of this License.  If the work
during execution displays copyright notices, you must include the
copyright notice for the Library among them, as well as a reference
directing the user to the copy of this License.  Also, you must do one
of these things:

    a) Accompany the work with the complete corresponding
    machine-readable source code for the Library including whatever
    changes were used in the work (which must be distributed under
    Sections 1 and 2 above); and, if the work is an executable linked
    with the Library, with the complete machine-readable "work that
    uses the Library", as object code and/or source code, so that the
    user can modify the Library and then relink to produce a modified
    executable containing the modified Library.  (It is understood
    that the user who changes the contents of definitions files in the
    Library will not necessarily be able to recompile the application
    to use the modified definitions.)

    b) Use a suitable shared library mechanism for linking with the
    Library.  A suitable mechanism is one that (1) uses at run time a
    copy of the library already present on the user's computer system,
    rather than copying library functions into the executable, and (2)
    will operate properly with a modified version of the library, if
    the user installs one, as long as the modified version is
    interface-compatible with the version that the work was made with.

    c) Accompany the work with a written offer, valid for at
    least three years, to give the same user the materials
    specified in Subsection 6a, above, for a charge no more
    than the cost of performing this distribution.

    d) If distribution of the work is made by offering access to copy
    from a designated place, offer equivalent access to copy the above
    specified materials from the same place.

    e) Verify that the user has already received a copy of these
    materials or that you have already sent this user a copy.

  For an executable, the required form of the "work that uses the
Library" must include any data and utility programs needed for
reproducing the executable from it.  However, as a special exception,
the materials to be distributed need not include anything that is
normally distributed (in either source or binary form) with the major
components (compiler, kernel, and so on) of the operating system on
which the executable runs, unless that component itself accompanies
the executable.

  It may happen that this requirement contradicts the license
restrictions of other proprietary libraries that do not normally
accompany the operating system.  Such a contradiction means you cannot
use both them and the Library together in an executable that you
distribute.

  7. You may place library facilities that are a work based on the
Library side-by-side in a single library together with other library
facilities not covered by this License, and distribute such a combined
library, provided that the separate distribution of the work based on
the Library and of the other library facilities is otherwise
permitted, and provided that you do these two things:

    a) Accompany the combined library with a copy of the same work
    based on the Library, uncombined with any other library
    facilities.  This must be distributed under the terms of the
    Sections above.

    b) Give prominent notice with the combined library of the fact
    that part of it is a work based on the Library, and explaining
    where to find the accompanying uncombined form of the same work.

  8. You may not copy, modify, sublicense, link with, or distribute
the Library except as expressly provided under this License.  Any
attempt otherwise to copy, modify, sublicense, link with, or
distribute the Library is void, and will automatically terminate your
rights under this License.  However, parties who have received copies,
or rights, from you under this License will not have their licenses
terminated so long as such parties remain in full compliance.

  9. You are not required to accept this License, since you have not
signed it.  However, nothing else grants you permission to modify or
distribute the Library or its derivative works.  These actions are
prohibited by law if you do not accept this License.  Therefore, by
modifying or distributing the Library (or any work based on the
Library), you indicate your acceptance of this License to do so, and
all its terms and conditions for copying, distributing or modifying
the Library or works based on it.

  10. Each time you redistribute the Library (or any work based on the
Library), the recipient automatically receives a license from the
original licensor to copy, distribute, link with or modify the Library
subject to these terms and conditions.  You may not impose any further
restrictions on the recipients' exercise of the rights granted herein.
You are not responsible for enforcing compliance by third parties with
this License.

  11. If, as a consequence of a court judgment or allegation of patent
infringement or for any other reason (not limited to patent issues),
conditions are imposed on you (whether by court order, agreement or
otherwise) that contradict the conditions of this License, they do not
excuse you from the conditions of this License.  If you cannot
distribute so as to satisfy simultaneously your obligations under this
License and any other pertinent obligations, then as a consequence you
may not distribute the Library at all.  For example, if a patent
license would not permit royalty-free redistribution of the Library by
all those who receive copies directly or indirectly through you, then
the only way you could satisfy both it and this License would be to
refrain entirely from distribution of the Library.

If any portion of this section is held invalid or unenforceable under any
particular circumstance, the balance of the section is intended to apply,
and the section as a whole is intended to apply in other circumstances.

It is not the purpose of this section to induce you to infringe any
patents or other property right claims or to contest validity of any
such claims; this section has the sole purpose of protecting the
integrity of the free software distribution system which is
implemented by public license practices.  Many people have made
generous contributions to the wide range of software distributed
through that system in reliance on consistent application of that
system; it is up to the author/donor to decide if he or she is willing
to distribute software through any other system and a licensee cannot
impose that choice.

This section is intended to make thoroughly clear what is believed to
be a consequence of the rest of this License.

  12. If the distribution and/or use of the Library is restricted in
certain countries either by patents or by copyrighted interfaces, the
original copyright holder who places the Library under this License may add
an explicit geographical distribution limitation excluding those countries,
so that distribution is permitted only in or among countries not thus
excluded.  In such case, this License incorporates the limitation as if
written in the body of this License.

  13. The Free Software Foundation may publish revised and/or new
versions of the Lesser General Public License from time to time.
Such new versions will be similar in spirit to the present version,
but may differ in detail to address new problems or concerns.

Each version is given a distinguishing version number.  If the Library
specifies a version number of this License which applies to it and
"any later version", you have the option of following the terms and
conditions either of that version or of any later version published by
the Free Software Foundation.  If the Library does not specify a
license version number, you may choose any version ever published by
the Free Software Foundation.

  14. If you wish to incorporate parts of the Library into other free
programs whose distribution conditions are incompatible with these,
write to the author to ask for permission.  For software which is
copyrighted by the Free Software Foundation, write to the Free
Software Foundation; we sometimes make exceptions for this.  Our
decision will be guided by the two goals of preserving the free status
of all derivatives of our free software and of promoting the sharing
and reuse of software generally.

			    NO WARRANTY

  15. BECAUSE THE LIBRARY IS LICENSED FREE OF CHARGE, THERE IS NO
WARRANTY FOR THE LIBRARY, TO THE EXTENT PERMITTED BY APPLICABLE LAW.
EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT HOLDERS AND/OR
OTHER PARTIES PROVIDE THE LIBRARY "AS IS" WITHOUT WARRANTY OF ANY
KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
PURPOSE.  THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE
LIBRARY IS WITH YOU.  SHOULD THE LIBRARY PROVE DEFECTIVE, YOU ASSUME
THE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION.

  16. IN NO EVENT UNLESS REQUIRED BY APPLICABLE LAW OR AGREED TO IN
WRITING WILL ANY COPYRIGHT HOLDER, OR ANY OTHER PARTY WHO MAY MODIFY
AND/OR REDISTRIBUTE THE LIBRARY AS PERMITTED ABOVE, BE LIABLE TO YOU
FOR DAMAGES, INCLUDING ANY GENERAL, SPECIAL, INCIDENTAL OR
CONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OR INABILITY TO USE THE
LIBRARY (INCLUDING BUT NOT LIMITED TO LOSS OF DATA OR DATA BEING
RENDERED INACCURATE OR LOSSES SUSTAINED BY YOU OR THIRD PARTIES OR A
FAILURE OF THE LIBRARY TO OPERATE WITH ANY OTHER SOFTWARE), EVEN IF
SUCH HOLDER OR OTHER PARTY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
DAMAGES.

		     END OF TERMS AND CONDITIONS

