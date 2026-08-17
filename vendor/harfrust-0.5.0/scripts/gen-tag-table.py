#!/usr/bin/env python3

"""
Generator of the mapping from OpenType tags to BCP 47 tags and vice
versa.

Based on harfbuzz/src/gen-tag-table.py

Input files:
- https://learn.microsoft.com/en-us/typography/opentype/spec/languagetags (the whole page as HTML)
- https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
"""

# TODO: rewrite

import collections
from html.parser import HTMLParser
import html
import io
import itertools
import re
import sys
import unicodedata

if len(sys.argv) != 3:
    print('Usage: ./gen-tag-table.py languagetags.html language-subtag-registry.txt', file=sys.stderr)
    sys.exit(1)


def expect(condition, message=None):
    if not condition:
        if message is None:
            raise AssertionError
        raise AssertionError(message)

DEFAULT_LANGUAGE_SYSTEM = ''

# from http://www-01.sil.org/iso639-3/iso-639-3.tab
ISO_639_3_TO_1 = {
    'aar': 'aa',
    'abk': 'ab',
    'afr': 'af',
    'aka': 'ak',
    'amh': 'am',
    'ara': 'ar',
    'arg': 'an',
    'asm': 'as',
    'ava': 'av',
    'ave': 'ae',
    'aym': 'ay',
    'aze': 'az',
    'bak': 'ba',
    'bam': 'bm',
    'bel': 'be',
    'ben': 'bn',
    'bis': 'bi',
    'bod': 'bo',
    'bos': 'bs',
    'bre': 'br',
    'bul': 'bg',
    'cat': 'ca',
    'ces': 'cs',
    'cha': 'ch',
    'che': 'ce',
    'chu': 'cu',
    'chv': 'cv',
    'cor': 'kw',
    'cos': 'co',
    'cre': 'cr',
    'cym': 'cy',
    'dan': 'da',
    'deu': 'de',
    'div': 'dv',
    'dzo': 'dz',
    'ell': 'el',
    'eng': 'en',
    'epo': 'eo',
    'est': 'et',
    'eus': 'eu',
    'ewe': 'ee',
    'fao': 'fo',
    'fas': 'fa',
    'fij': 'fj',
    'fin': 'fi',
    'fra': 'fr',
    'fry': 'fy',
    'ful': 'ff',
    'gla': 'gd',
    'gle': 'ga',
    'glg': 'gl',
    'glv': 'gv',
    'grn': 'gn',
    'guj': 'gu',
    'hat': 'ht',
    'hau': 'ha',
    'hbs': 'sh',
    'heb': 'he',
    'her': 'hz',
    'hin': 'hi',
    'hmo': 'ho',
    'hrv': 'hr',
    'hun': 'hu',
    'hye': 'hy',
    'ibo': 'ig',
    'ido': 'io',
    'iii': 'ii',
    'iku': 'iu',
    'ile': 'ie',
    'ina': 'ia',
    'ind': 'id',
    'ipk': 'ik',
    'isl': 'is',
    'ita': 'it',
    'jav': 'jv',
    'jpn': 'ja',
    'kal': 'kl',
    'kan': 'kn',
    'kas': 'ks',
    'kat': 'ka',
    'kau': 'kr',
    'kaz': 'kk',
    'khm': 'km',
    'kik': 'ki',
    'kin': 'rw',
    'kir': 'ky',
    'kom': 'kv',
    'kon': 'kg',
    'kor': 'ko',
    'kua': 'kj',
    'kur': 'ku',
    'lao': 'lo',
    'lat': 'la',
    'lav': 'lv',
    'lim': 'li',
    'lin': 'ln',
    'lit': 'lt',
    'ltz': 'lb',
    'lub': 'lu',
    'lug': 'lg',
    'mah': 'mh',
    'mal': 'ml',
    'mar': 'mr',
    'mkd': 'mk',
    'mlg': 'mg',
    'mlt': 'mt',
    'mol': 'mo',
    'mon': 'mn',
    'mri': 'mi',
    'msa': 'ms',
    'mya': 'my',
    'nau': 'na',
    'nav': 'nv',
    'nbl': 'nr',
    'nde': 'nd',
    'ndo': 'ng',
    'nep': 'ne',
    'nld': 'nl',
    'nno': 'nn',
    'nob': 'nb',
    'nor': 'no',
    'nya': 'ny',
    'oci': 'oc',
    'oji': 'oj',
    'ori': 'or',
    'orm': 'om',
    'oss': 'os',
    'pan': 'pa',
    'pli': 'pi',
    'pol': 'pl',
    'por': 'pt',
    'pus': 'ps',
    'que': 'qu',
    'roh': 'rm',
    'ron': 'ro',
    'run': 'rn',
    'rus': 'ru',
    'sag': 'sg',
    'san': 'sa',
    'sin': 'si',
    'slk': 'sk',
    'slv': 'sl',
    'sme': 'se',
    'smo': 'sm',
    'sna': 'sn',
    'snd': 'sd',
    'som': 'so',
    'sot': 'st',
    'spa': 'es',
    'sqi': 'sq',
    'srd': 'sc',
    'srp': 'sr',
    'ssw': 'ss',
    'sun': 'su',
    'swa': 'sw',
    'swe': 'sv',
    'tah': 'ty',
    'tam': 'ta',
    'tat': 'tt',
    'tel': 'te',
    'tgk': 'tg',
    'tgl': 'tl',
    'tha': 'th',
    'tir': 'ti',
    'ton': 'to',
    'tsn': 'tn',
    'tso': 'ts',
    'tuk': 'tk',
    'tur': 'tr',
    'twi': 'tw',
    'uig': 'ug',
    'ukr': 'uk',
    'urd': 'ur',
    'uzb': 'uz',
    'ven': 've',
    'vie': 'vi',
    'vol': 'vo',
    'wln': 'wa',
    'wol': 'wo',
    'xho': 'xh',
    'yid': 'yi',
    'yor': 'yo',
    'zha': 'za',
    'zho': 'zh',
    'zul': 'zu',
}


class LanguageTag(object):
    """A BCP 47 language tag.

    Attributes:
        subtags(List[str]): The list of subtags in this tag.
        grandfathered(bool): Whether this tag is grandfathered. If
            ``true``, the entire lowercased tag is the ``language``
            and the other subtag fields are empty.
        language(str): The language subtag.
        script(str): The script subtag.
        region(str): The region subtag.
        variant(str): The variant subtag.

    Args:
        tag(str): A BCP 47 language tag.

    """
    def __init__(self, tag):
        global bcp_47
        self.subtags = tag.lower().split('-')
        self.grandfathered = tag.lower() in bcp_47.grandfathered
        if self.grandfathered:
            self.language = tag.lower()
            self.script = ''
            self.region = ''
            self.variant = ''
        else:
            self.language = self.subtags[0]
            self.script = self._find_first(lambda s: len(s) == 4 and s[0] > '9', self.subtags)
            self.region = self._find_first(lambda s: len(s) == 2 and s[0] > '9' or len(s) == 3 and s[0] <= '9', self.subtags[1:])
            self.variant = self._find_first(lambda s: len(s) > 4 or len(s) == 4 and s[0] <= '9', self.subtags)

    def __str__(self):
        return '-'.join(self.subtags)

    def __repr__(self):
        return 'LanguageTag(%r)' % str(self)

    @staticmethod
    def _find_first(function, sequence):
        try:
            return next(iter(filter(function, sequence)))
        except StopIteration:
            return None

    def is_complex(self):
        """Return whether this tag is too complex to represent as a
        ``LangTag`` in the generated code.

        Complex tags need to be handled in
        ``hb_ot_tags_from_complex_language``.

        Returns:
            Whether this tag is complex.
        """
        return not(len(self.subtags) == 1
                   or self.grandfathered
                   and len(self.subtags[1]) != 3
                   and ot.from_bcp_47[self.subtags[0]] == ot.from_bcp_47[self.language])

    def get_group(self):
        """Return the group into which this tag should be categorized in
        ``hb_ot_tags_from_complex_language``.

        The group is the first letter of the tag, or ``'und'`` if this tag
        should not be matched in a ``switch`` statement in the generated
        code.

        Returns:
            This tag's group.
        """
        return('und'
               if(self.language == 'und'
                  or self.variant in bcp_47.prefixes and len(bcp_47.prefixes[self.variant]) == 1)
               else self.language[0])


class OpenTypeRegistryParser(HTMLParser):
    """A parser for the OpenType language system tag registry.

    Attributes:
        header(str): The "last updated" line of the registry.
        names(Mapping[str, str]): A map of language system tags to the
            names they are given in the registry.
        ranks(DefaultDict[str, int]): A map of language system tags to
            numbers. If a single BCP 47 tag corresponds to multiple
            OpenType tags, the tags are ordered in increasing order by
            rank. The rank is based on the number of BCP 47 tags
            associated with a tag, though it may be manually modified.
        to_bcp_47(DefaultDict[str, AbstractSet[str]]): A map of
            OpenType language system tags to sets of BCP 47 tags.
        from_bcp_47(DefaultDict[str, AbstractSet[str]]): ``to_bcp_47``
            inverted. Its values start as unsorted sets;
            ``sort_languages`` converts them to sorted lists.
        from_bcp_47_uninherited (Optional[Dict[str, AbstractSet[str]]]):
            A copy of ``from_bcp_47``. It starts as ``None`` and is
            populated at the beginning of the first call to
            ``inherit_from_macrolanguages``.

    """
    def __init__(self):
        HTMLParser.__init__(self)
        self.header = ''
        self.names = {}
        self.ranks = collections.defaultdict(int)
        self.to_bcp_47 = collections.defaultdict(set)
        self.from_bcp_47 = collections.defaultdict(set)
        self.from_bcp_47_uninherited = None
        # Whether the parser is in a <td> element
        self._td = False
        # Whether the parser ignores the rest of the current <td> element
        self._disengaged = False
        # The text of the <td> elements of the current <tr> element.
        self._current_tr = []

    def handle_starttag (self, tag, attrs):
        if tag == 'a':
            if self._current_tr and not self._disengaged:
                self._current_tr[-1] = ''
                self._disengaged = True
        elif tag == 'br':
            self._disengaged = True
        elif tag == 'meta':
            for attr, value in attrs:
                if attr == 'name' and value == 'updated_at':
                    self.header = self.get_starttag_text ()
                    break
        elif tag == 'td':
            self._td = True
            self._current_tr.append ('')
        elif tag == 'tr':
            self._disengaged = False
            self._current_tr = []

    def handle_endtag (self, tag):
        if tag == 'td':
            self._td = False
            self._disengaged = False
        elif tag == 'tr' and self._current_tr:
            expect (2 <= len (self._current_tr) <= 3)
            name = self._current_tr[0].strip ()
            tag = self._current_tr[1].strip ("\t\n\v\f\r '")
            rank = 0
            if len (tag) > 4:
                expect (tag.endswith (' (deprecated)'), 'ill-formed OpenType tag: %s' % tag)
                name += ' (deprecated)'
                tag = tag.split (' ')[0]
                rank = 1
            self.names[tag] = re.sub (' languages$', '', name)
            if not self._current_tr[2]:
                return
            iso_codes = self._current_tr[2].strip ()
            self.to_bcp_47[tag].update (ISO_639_3_TO_1.get (code, code) for code in iso_codes.replace (' ', '').split (','))
            rank += 2 * len (self.to_bcp_47[tag])
            self.ranks[tag] = rank

    def handle_data (self, data):
        if self._td and not self._disengaged:
            self._current_tr[-1] += data

    def handle_charref(self, name):
        self.handle_data(html.unescape('&#%s;' % name))

    def handle_entityref(self, name):
        self.handle_data(html.unescape('&%s;' % name))

    def parse(self, filename):
        """Parse the OpenType language system tag registry.

        Args:
            filename(str): The file name of the registry.
        """
        with io.open(filename, encoding='utf-8') as f:
            self.feed(f.read())
        expect(self.header)
        for tag, iso_codes in self.to_bcp_47.items():
            for iso_code in iso_codes:
                self.from_bcp_47[iso_code].add(tag)

    def add_language(self, bcp_47_tag, ot_tag):
        """Add a language as if it were in the registry.

        Args:
            bcp_47_tag(str): A BCP 47 tag. If the tag is more than just
                a language subtag, and if the language subtag is a
                macrolanguage, then new languages are added corresponding
                to the macrolanguages' individual languages with the
                remainder of the tag appended.
            ot_tag(str): An OpenType language system tag.
        """
        global bcp_47
        self.to_bcp_47[ot_tag].add(bcp_47_tag)
        self.from_bcp_47[bcp_47_tag].add(ot_tag)
        if bcp_47_tag.lower() not in bcp_47.grandfathered:
            try:
                [macrolanguage, suffix] = bcp_47_tag.split('-', 1)
                if macrolanguage in bcp_47.macrolanguages:
                    s = set()
                    for language in bcp_47.macrolanguages[macrolanguage]:
                        if language.lower() not in bcp_47.grandfathered:
                            s.add('%s-%s' %(language, suffix))
                    bcp_47.macrolanguages['%s-%s' %(macrolanguage, suffix)] = s
            except ValueError:
                pass

    @staticmethod
    def _remove_language(tag_1, dict_1, dict_2):
        for tag_2 in dict_1.pop(tag_1):
            dict_2[tag_2].remove(tag_1)
            if not dict_2[tag_2]:
                del dict_2[tag_2]

    def remove_language_ot(self, ot_tag):
        """Remove an OpenType tag from the registry.

        Args:
            ot_tag(str): An OpenType tag.
        """
        self._remove_language(ot_tag, self.to_bcp_47, self.from_bcp_47)

    def remove_language_bcp_47(self, bcp_47_tag):
        """Remove a BCP 47 tag from the registry.

        Args:
            bcp_47_tag(str): A BCP 47 tag.
        """
        self._remove_language(bcp_47_tag, self.from_bcp_47, self.to_bcp_47)

    def inherit_from_macrolanguages(self):
        """Copy mappings from macrolanguages to individual languages.

        If a BCP 47 tag for an individual mapping has no OpenType
        mapping but its macrolanguage does, the mapping is copied to
        the individual language. For example, als (Tosk Albanian) has no
        explicit mapping, so it inherits from sq (Albanian) the mapping
        to SQI.

        However, if an OpenType tag maps to a BCP 47 macrolanguage and
        some but not all of its individual languages, the mapping is not
        inherited from the macrolanguage to the missing individual
        languages. For example, INUK (Nunavik Inuktitut) is mapped to
        ike (Eastern Canadian Inuktitut) and iu (Inuktitut) but not to
        ikt (Inuinnaqtun, which is an individual language of iu), so
        this method does not add a mapping from ikt to INUK.


        If a BCP 47 tag for a macrolanguage has no OpenType mapping but
        some of its individual languages do, their mappings are copied
        to the macrolanguage.
        """
        global bcp_47
        first_time = self.from_bcp_47_uninherited is None
        if first_time:
            self.from_bcp_47_uninherited = dict(self.from_bcp_47)
        for macrolanguage, languages in dict(bcp_47.macrolanguages).items():
            ot_macrolanguages = {
                ot_macrolanguage for ot_macrolanguage in self.from_bcp_47_uninherited.get(macrolanguage, set ())
            }
            blocked_ot_macrolanguages = set()
            if 'retired code' not in bcp_47.scopes.get(macrolanguage, ''):
                for ot_macrolanguage in ot_macrolanguages:
                    round_trip_macrolanguages = {
                        l for l in self.to_bcp_47[ot_macrolanguage]
                        if 'retired code' not in bcp_47.scopes.get(l, '')
                    }
                    round_trip_languages = {
                        l for l in languages
                        if 'retired code' not in bcp_47.scopes.get(l, '')
                    }
                    intersection = round_trip_macrolanguages & round_trip_languages
                    if intersection and intersection != round_trip_languages:
                        blocked_ot_macrolanguages.add(ot_macrolanguage)
            if ot_macrolanguages:
                for ot_macrolanguage in ot_macrolanguages:
                    if ot_macrolanguage not in blocked_ot_macrolanguages:
                        for language in languages:
                            self.add_language(language, ot_macrolanguage)
                            if not blocked_ot_macrolanguages:
                                self.ranks[ot_macrolanguage] += 1
            elif first_time:
                for language in languages:
                    if language in self.from_bcp_47_uninherited:
                        ot_macrolanguages |= self.from_bcp_47_uninherited[language]
                    else:
                        ot_macrolanguages.clear()
                    if not ot_macrolanguages:
                        break
                for ot_macrolanguage in ot_macrolanguages:
                    self.add_language(macrolanguage, ot_macrolanguage)

    def sort_languages(self):
        """Sort the values of ``from_bcp_47`` in ascending rank order."""
        for language, tags in self.from_bcp_47.items():
            self.from_bcp_47[language] = sorted(tags, key=lambda t:(self.ranks[t] + rank_delta(language, t), t))


ot = OpenTypeRegistryParser()


class BCP47Parser(object):
    """A parser for the BCP 47 subtag registry.

    Attributes:
        header(str): The "File-Date" line of the registry.
        names(Mapping[str, str]): A map of subtags to the names they
            are given in the registry. Each value is a
            ``'\\n'``-separated list of names.
        scopes(Mapping[str, str]): A map of language subtags to strings
            suffixed to language names, including suffixes to explain
            language scopes.
        macrolanguages(DefaultDict[str, AbstractSet[str]]): A map of
            language subtags to the sets of language subtags which
            inherit from them. See
            ``OpenTypeRegistryParser.inherit_from_macrolanguages``.
        prefixes(DefaultDict[str, AbstractSet[str]]): A map of variant
            subtags to their prefixes.
        grandfathered(AbstractSet[str]): The set of grandfathered tags,
            normalized to lowercase.

    """
    def __init__(self):
        self.header = ''
        self.names = {}
        self.scopes = {}
        self.macrolanguages = collections.defaultdict(set)
        self.prefixes = collections.defaultdict(set)
        self.grandfathered = set()

    def parse(self, filename):
        """Parse the BCP 47 subtag registry.

        Args:
            filename(str): The file name of the registry.
        """
        with io.open(filename, encoding='utf-8') as f:
            subtag_type = None
            subtag = None
            deprecated = False
            has_preferred_value = False
            line_buffer = ''
            for line in itertools.chain(f, ['']):
                line = line.rstrip()
                if line.startswith(' '):
                    line_buffer += line[1:]
                    continue
                line, line_buffer = line_buffer, line
                if line.startswith('Type: '):
                    subtag_type = line.split(' ')[1]
                    deprecated = False
                    has_preferred_value = False
                elif line.startswith('Subtag: ') or line.startswith('Tag: '):
                    subtag = line.split(' ')[1]
                    if subtag_type == 'grandfathered':
                        self.grandfathered.add(subtag.lower())
                elif line.startswith('Description: '):
                    description = line.split(' ', 1)[1].replace('(individual language)', '')
                    description = re.sub(r'(\(family\)|\((individual |macro)language\)|languages)$', '',
                                         description)
                    if subtag in self.names:
                        self.names[subtag] += '\n' + description
                    else:
                        self.names[subtag] = description
                elif subtag_type == 'language' or subtag_type == 'grandfathered':
                    if line.startswith('Scope: '):
                        scope = line.split(' ')[1]
                        if scope == 'macrolanguage':
                            scope = ' [macrolanguage]'
                        elif scope == 'collection':
                            scope = ' [collection]'
                        else:
                            continue
                        self.scopes[subtag] = scope
                    elif line.startswith('Deprecated: '):
                        self.scopes[subtag] = '(retired code)' + self.scopes.get(subtag, '')
                        deprecated = True
                    elif deprecated and line.startswith('Comments: see '):
                        # If a subtag is split into multiple replacement subtags,
                        # it essentially represents a macrolanguage.
                        for language in line.replace(',', '').split(' ')[2:]:
                            self._add_macrolanguage(subtag, language)
                    elif line.startswith('Preferred-Value: '):
                        # If a subtag is deprecated in favor of a single replacement subtag,
                        # it is either a dialect or synonym of the preferred subtag. Either
                        # way, it is close enough to the truth to consider the replacement
                        # the macrolanguage of the deprecated language.
                        has_preferred_value = True
                        macrolanguage = line.split(' ')[1]
                        self._add_macrolanguage(macrolanguage, subtag)
                    elif not has_preferred_value and line.startswith('Macrolanguage: '):
                        self._add_macrolanguage(line.split(' ')[1], subtag)
                elif subtag_type == 'variant':
                    if line.startswith('Deprecated: '):
                        self.scopes[subtag] = ' (retired code)' + self.scopes.get(subtag, '')
                    elif line.startswith('Prefix: '):
                        self.prefixes[subtag].add(line.split(' ')[1])
                elif line.startswith('File-Date: '):
                    self.header = line
        expect(self.header)

    def _add_macrolanguage(self, macrolanguage, language):
        global ot
        if language not in ot.from_bcp_47:
            for l in self.macrolanguages.get(language, set()):
                self._add_macrolanguage(macrolanguage, l)
        if macrolanguage not in ot.from_bcp_47:
            for ls in list(self.macrolanguages.values()):
                if macrolanguage in ls:
                    ls.add(language)
                    return
        self.macrolanguages[macrolanguage].add(language)

    def remove_extra_macrolanguages(self):
        """Make every language have at most one macrolanguage."""
        inverted = collections.defaultdict(list)
        for macrolanguage, languages in self.macrolanguages.items():
            for language in languages:
                inverted[language].append(macrolanguage)
        for language, macrolanguages in inverted.items():
            if len(macrolanguages) > 1:
                macrolanguages.sort(key=lambda ml: len(self.macrolanguages[ml]))
                biggest_macrolanguage = macrolanguages.pop()
                for macrolanguage in macrolanguages:
                    self._add_macrolanguage(biggest_macrolanguage, macrolanguage)

    def _get_name_piece(self, subtag):
        """Return the first name of a subtag plus its scope suffix.
        Args:
            subtag (str): A BCP 47 subtag.
        Returns:
            The name form of ``subtag``.
        """
        return self.names[subtag].split('\n')[0] + self.scopes.get(subtag, '')

    def get_name(self, lt):
        """Return the names of the subtags in a language tag.

        Args:
            lt(LanguageTag): A BCP 47 language tag.

        Returns:
            The name form of ``lt``.
        """
        name = self._get_name_piece(lt.language)
        if lt.script:
            name += '; ' + self._get_name_piece(lt.script.title())
        if lt.region:
            name += '; ' + self._get_name_piece(lt.region.upper())
        if lt.variant:
            name += '; ' + self._get_name_piece(lt.variant)
        return name


bcp_47 = BCP47Parser()

ot.parse(sys.argv[1])
bcp_47.parse(sys.argv[2])

ot.add_language('ary', 'MOR')

ot.add_language('ath', 'ATH')

ot.add_language('bai', 'BML')

ot.ranks['BAL'] = ot.ranks['KAR'] + 1

ot.add_language('ber', 'BBR')

ot.remove_language_ot('PGR')
ot.add_language('el-polyton', 'PGR')

bcp_47.names['flm'] = 'Falam Chin'
bcp_47.scopes['flm'] = '(retired code)'
bcp_47.macrolanguages['flm'] = {'cfm'}

ot.ranks['FNE'] = ot.ranks['TNE'] + 1

ot.add_language('und-fonipa', 'IPPH')

ot.add_language('und-fonnapa', 'APPH')

ot.add_language('ga-Latg', 'IRT')

ot.add_language('hy-arevmda', 'HYE')

ot.remove_language_ot('KGE')
ot.add_language('und-Geok', 'KGE')

ot.add_language('kht', 'KHN')
ot.names['KHN'] = ot.names['KHT'] + '(Microsoft fonts)'
ot.ranks['KHN'] = ot.ranks['KHT'] + 1

ot.ranks['LCR'] = ot.ranks['MCR'] + 1

ot.names['MAL'] = 'Malayalam Traditional'
ot.ranks['MLR'] += 1

bcp_47.names['mhv'] = 'Arakanese'
bcp_47.scopes['mhv'] = '(retired code)'

# Downstream change due to note for Thailand Mon in Microsoft’s
# page of language tags.
ot.remove_language_ot('MONT')
ot.add_language('mnw', 'MONT')

ot.add_language ('mnw-TH', 'MONT')

ot.add_language('no', 'NOR')

ot.add_language('oc-provenc', 'PRO')

ot.remove_language_ot('QUZ')
ot.add_language('qu', 'QUZ')
ot.add_language('qub', 'QWH')
ot.add_language('qud', 'QVI')
ot.add_language('qug', 'QVI')
ot.add_language('qul', 'QUH')
ot.add_language('qup', 'QVI')
ot.add_language('qur', 'QWH')
ot.add_language('qus', 'QUH')
ot.add_language('quw', 'QVI')
ot.add_language('qux', 'QWH')
ot.add_language('qva', 'QWH')
ot.add_language('qvh', 'QWH')
ot.add_language('qvj', 'QVI')
ot.add_language('qvl', 'QWH')
ot.add_language('qvm', 'QWH')
ot.add_language('qvn', 'QWH')
ot.add_language('qvo', 'QVI')
ot.add_language('qvp', 'QWH')
ot.add_language('qvw', 'QWH')
ot.add_language('qvz', 'QVI')
ot.add_language('qwa', 'QWH')
ot.add_language('qws', 'QWH')
ot.add_language('qxa', 'QWH')
ot.add_language('qxc', 'QWH')
ot.add_language('qxh', 'QWH')
ot.add_language('qxl', 'QVI')
ot.add_language('qxn', 'QWH')
ot.add_language('qxo', 'QWH')
ot.add_language('qxr', 'QVI')
ot.add_language('qxt', 'QWH')
ot.add_language('qxw', 'QWH')

bcp_47.macrolanguages['ro-MD'].add('mo')

ot.remove_language_ot('SYRE')
ot.remove_language_ot('SYRJ')
ot.remove_language_ot('SYRN')
ot.add_language('und-Syre', 'SYRE')
ot.add_language('und-Syrj', 'SYRJ')
ot.add_language('und-Syrn', 'SYRN')

bcp_47.names['xst'] = u"Silt'e"
bcp_47.scopes['xst'] = '(retired code)'
bcp_47.macrolanguages['xst'] = {'stv', 'wle'}

ot.add_language('xwo', 'TOD')

ot.remove_language_ot('ZHH')
ot.remove_language_ot('ZHP')
ot.remove_language_ot('ZHT')
ot.remove_language_ot('ZHTM')
bcp_47.macrolanguages['zh'].remove('lzh')
bcp_47.macrolanguages['zh'].remove('yue')
ot.add_language('zh-Hant-MO', 'ZHH')
ot.add_language('zh-Hant-MO', 'ZHTM')
ot.add_language('zh-Hant-HK', 'ZHH')
ot.add_language('zh-Hans', 'ZHS')
ot.add_language('zh-Hant', 'ZHT')
ot.add_language('zh-HK', 'ZHH')
ot.add_language('zh-MO', 'ZHH')
ot.add_language('zh-MO', 'ZHTM')
ot.add_language('zh-TW', 'ZHT')
ot.add_language('lzh', 'ZHT')
ot.add_language('lzh-Hans', 'ZHS')
ot.add_language('yue', 'ZHH')
ot.add_language('yue-Hans', 'ZHS')


def rank_delta(bcp_47, ot):
    """Return a delta to apply to a BCP 47 tag's rank.

    Most OpenType tags have a constant rank, but a few have ranks that
    depend on the BCP 47 tag.

    Args:
        bcp_47(str): A BCP 47 tag.
        ot(str): An OpenType tag to.

    Returns:
        A number to add to ``ot``'s rank when sorting ``bcp_47``'s
        OpenType equivalents.
    """
    if bcp_47 == 'ak' and ot == 'AKA':
        return -1
    if bcp_47 == 'tw' and ot == 'TWI':
        return -1
    return 0


disambiguation = {
    'ALT': 'alt',
    'ARK': 'rki',
    'ATH': 'ath',
    'BHI': 'bhb',
    'BLN': 'bjt',
    'BTI': 'beb',
    'CCHN': 'cco',
    'CMR': 'swb',
    'CPP': 'crp',
    'CRR': 'crx',
    'DUJ': 'dwu',
    'ECR': 'crj',
    'HAL': 'cfm',
    'HND': 'hnd',
    'HYE': 'hyw',
    'KIS': 'kqs',
    'LRC': 'bqi',
    'NDB': 'nd',
    'NIS': 'njz',
    'PLG': 'pce',
    'PRO': 'pro',
    'QIN': 'bgr',
    'QUH': 'quh',
    'QVI': 'qvi',
    'QWH': 'qwh',
    'SIG': 'stv',
    'SRB': 'sr',
    'SXT': 'xnj',
    'ZHH': 'zh-HK',
    'ZHS': 'zh-Hans',
    'ZHT': 'zh-Hant',
    'ZHTM': 'zh-Hant-MO',
}

ot.inherit_from_macrolanguages()
bcp_47.remove_extra_macrolanguages()
ot.inherit_from_macrolanguages()
ot.names[DEFAULT_LANGUAGE_SYSTEM] = '*/'
ot.ranks[DEFAULT_LANGUAGE_SYSTEM] = max(ot.ranks.values()) + 1
for tricky_ot_tag in filter(lambda tag: re.match('[A-Z]{3}$', tag), ot.names):
    possible_bcp_47_tag = tricky_ot_tag.lower()
    if possible_bcp_47_tag in bcp_47.names and not ot.from_bcp_47[possible_bcp_47_tag]:
        ot.add_language(possible_bcp_47_tag, DEFAULT_LANGUAGE_SYSTEM)
        bcp_47.macrolanguages[possible_bcp_47_tag] = set()
ot.sort_languages()

print('// WARNING: this file was generated by ../scripts/gen-tag-table.py')
print()
print('use read_fonts::types::Tag;')
print()
print('pub(crate) struct LangTag {')
print('    pub language: &\'static str,')
print('    pub tag: Tag,')
print('}')
print()
print('#[rustfmt::skip]')
print('pub(crate) static OPEN_TYPE_LANGUAGES: &[LangTag] = &[')


def hb_tag(tag):
    if tag == DEFAULT_LANGUAGE_SYSTEM:
        return 'Tag::new(&[0; 4])'
    return 'Tag::new(b\"%s%s%s%s\")' % tuple(('%-4s' % tag)[:4])


def hb_tag2(tag):
    return 'b"%s%s%s%s"' % tuple(('%-4s' % tag)[:4])


def get_variant_set(name):
    """Return a set of variant language names from a name.

    Args:
        name(str): A list of language names from the BCP 47 registry,
            joined on ``'\\n'``.

    Returns:
        A set of normalized language names.
    """
    return set(unicodedata.normalize('NFD', n.replace('\u2019', u"'"))
               .encode('ASCII', 'ignore')
               .strip()
               for n in re.split('[\n(),]', name) if n)


def language_name_intersection(a, b):
    """Return the names in common between two language names.

    Args:
        a(str): A list of language names from the BCP 47 registry,
            joined on ``'\\n'``.
        b(str): A list of language names from the BCP 47 registry,
            joined on ``'\\n'``.

    Returns:
        The normalized language names shared by ``a`` and ``b``.
    """
    return get_variant_set(a).intersection(get_variant_set(b))


def get_matching_language_name(intersection, candidates):
    return next(iter(c for c in candidates if not intersection.isdisjoint(get_variant_set(c))))


def same_tag(bcp_47_tag, ot_tags):
    return len(bcp_47_tag) == 3 and len(ot_tags) == 1 and bcp_47_tag == ot_tags[0].lower()


for language, tags in sorted(ot.from_bcp_47.items()):
    if language == '' or '-' in language:
        continue
    commented_out = same_tag(language, tags)
    for i, tag in enumerate(tags, start=1):
        print('%sLangTag { language: \"%s\", \ttag: %s },' % ('//  ' if commented_out else '    ', language, hb_tag(tag)), end='')
        print(' // ', end='')
        bcp_47_name = bcp_47.names.get(language, '')
        bcp_47_name_candidates = bcp_47_name.split('\n')
        ot_name = ot.names[tag]
        scope = bcp_47.scopes.get(language, '')
        if tag == DEFAULT_LANGUAGE_SYSTEM:
            print(f'{bcp_47_name_candidates[0]}{scope} != {ot.names[language.upper()]}')
        else:
            intersection = language_name_intersection(bcp_47_name, ot_name)
            if not intersection:
                print('%s%s -> %s' % (bcp_47_name_candidates[0], scope, ot_name))
            else:
                name = get_matching_language_name(intersection, bcp_47_name_candidates)
                bcp_47.names[language] = name
                print('%s%s' % (name if len(name) > len(ot_name) else ot_name, scope))

print('];')
print()


print('fn subtag_matches(language: &str, subtag: &str) -> bool {')
print('    for (i, _) in language.match_indices(subtag) {')
print('        if let Some(c) = language.as_bytes().get(i + subtag.len()) {')
print('            if !c.is_ascii_alphanumeric() {')
print('                return true;')
print('            }')
print('        } else {')
print('            return true;')
print('        }')
print('    }')
print()
print('    false')
print('}')
print()
print('fn lang_matches(language: &str, spec: &str) -> bool {')
print('    if language.starts_with(spec) {')
print('        return language.len() == spec.len() || language.as_bytes().get(spec.len()) == Some(&b\'-\');')
print('    }')
print('')
print('    false')
print('}')
print()
print('fn strncmp(s1: &str, s2: &str, n: usize) -> bool {')
print('    let n1 = core::cmp::min(n, s1.len());')
print('    let n2 = core::cmp::min(n, s2.len());')
print('    s1[..n1] == s2[..n2]')
print('}')
print()
print('/// Converts a multi-subtag BCP 47 language tag to language tags.')
print('pub fn tags_from_complex_language(language: &str, tags: &mut smallvec::SmallVec<[Tag; 3]>) -> bool {')


def print_subtag_matches(subtag, new_line):
    if subtag:
        if new_line:
            print(' && ', end='')
        print('subtag_matches(language, "-%s")' % subtag, end='')


complex_tags = collections.defaultdict(list)
for initial, group in itertools.groupby((lt_tags for lt_tags in [
    (LanguageTag(language), tags)
    for language, tags in sorted(ot.from_bcp_47.items(),
                                 key=lambda i:(-len(i[0]), i[0]))
] if lt_tags[0].is_complex()),
                                        key=lambda lt_tags: lt_tags[0].get_group()):
    complex_tags[initial] += group

for initial, items in sorted(complex_tags.items()):
    if initial != 'und':
        continue

    for lt, tags in items:
        if not tags:
            continue
        if lt.variant in bcp_47.prefixes:
            expect(next(iter(bcp_47.prefixes[lt.variant])) == lt.language,
                   '%s is not a valid prefix of %s' %(lt.language, lt.variant))

        print('    if ', end='')
        print_subtag_matches(lt.script, False)
        print_subtag_matches(lt.region, False)
        print_subtag_matches(lt.variant, False)
        print(' {')
        print('        // %s' % bcp_47.get_name(lt))

        if len(tags) == 1:
            print('        tags.push(%s); // %s' % (hb_tag(tags[0]), ot.names[tags[0]]))
        else:
            print('        let possible_tags = &[')
            for tag in tags:
                print('      %s, // %s' % (hb_tag(tag), ot.names[tag]))
            print('                ];')
            print('      tags.extend_from_slice(possible_tags);')
        print('        return true;')
        print('    }')

print('    match language.as_bytes()[0] {')
for initial, items in sorted(complex_tags.items()):
    if initial == 'und':
        continue

    print("        b'%s' => {" % initial)
    for lt, tags in items:
        if not tags:
            continue
        print('            if ', end='')
        script = lt.script
        region = lt.region
        if lt.grandfathered:
            print('&language[1..] == "%s"' % lt.language[1:], end='')
        else:
            string_literal = lt.language[1:] + '-'
            if script:
                string_literal += script
                script = None
                if region:
                    string_literal += '-' + region
                    region = None
            if string_literal[-1] == '-':
                print('strncmp(&language[1..], "%s", %i)' % (string_literal, len(string_literal)), end='')
            else:
                print('lang_matches(&language[1..], "%s")' % string_literal, end='')

        print_subtag_matches(script, True)
        print_subtag_matches(region, True)
        print_subtag_matches(lt.variant, True)
        print(' {')
        print('                // %s' % bcp_47.get_name(lt))
        if len(tags) == 1:
            print('                tags.push(%s); // %s' % (hb_tag(tags[0]), ot.names[tags[0]]))
        else:
            print('                let possible_tags = &[')
            for tag in tags:
                print('                    %s, // %s' % (hb_tag(tag), ot.names[tag]))
            print('                ];')
            print('                tags.extend_from_slice(possible_tags);')
        print('                return true;')
        print('            }')
    print('        }')
print('        _ => {}')
print('    }')
print('    false')
print('}')
