/****************************************************************************
**
** Copyright (C) 2020 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

#ifndef QLOCALE_H
#define QLOCALE_H

#include <QtCore/qvariant.h>
#include <QtCore/qstring.h>
#include <QtCore/qobjectdefs.h>
#include <QtCore/qshareddata.h>

QT_BEGIN_NAMESPACE

class QCalendar;
class QDataStream;
class QDate;
class QDateTime;
class QLocale;
class QTime;
class QVariant;
class QTextStream;
class QTextStreamPrivate;

class QLocalePrivate;

Q_CORE_EXPORT uint qHash(const QLocale &key, uint seed = 0) noexcept;

class Q_CORE_EXPORT QLocale
{
    Q_GADGET
    friend class QString;
    friend class QStringRef;
    friend class QByteArray;
    friend class QIntValidator;
    friend class QDoubleValidatorPrivate;
    friend class QTextStream;
    friend class QTextStreamPrivate;

public:
// see qlocale_data_p.h for more info on generated data
// GENERATED PART STARTS HERE
    enum Language {
        AnyLanguage = 0,
        C = 1,
        Abkhazian = 2,
        Oromo = 3,
        Afar = 4,
        Afrikaans = 5,
        Albanian = 6,
        Amharic = 7,
        Arabic = 8,
        Armenian = 9,
        Assamese = 10,
        Aymara = 11,
        Azerbaijani = 12,
        Bashkir = 13,
        Basque = 14,
        Bengali = 15,
        Dzongkha = 16,
#if QT_DEPRECATED_SINCE(5, 15)
        Bihari Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 17,
#endif
        Bislama = 18,
        Breton = 19,
        Bulgarian = 20,
        Burmese = 21,
        Belarusian = 22,
        Khmer = 23,
        Catalan = 24,
        Chinese = 25,
        Corsican = 26,
        Croatian = 27,
        Czech = 28,
        Danish = 29,
        Dutch = 30,
        English = 31,
        Esperanto = 32,
        Estonian = 33,
        Faroese = 34,
        Fijian = 35,
        Finnish = 36,
        French = 37,
        WesternFrisian = 38,
        Gaelic = 39,
        Galician = 40,
        Georgian = 41,
        German = 42,
        Greek = 43,
        Greenlandic = 44,
        Guarani = 45,
        Gujarati = 46,
        Hausa = 47,
        Hebrew = 48,
        Hindi = 49,
        Hungarian = 50,
        Icelandic = 51,
        Indonesian = 52,
        Interlingua = 53,
        Interlingue = 54,
        Inuktitut = 55,
        Inupiak = 56,
        Irish = 57,
        Italian = 58,
        Japanese = 59,
        Javanese = 60,
        Kannada = 61,
        Kashmiri = 62,
        Kazakh = 63,
        Kinyarwanda = 64,
        Kirghiz = 65,
        Korean = 66,
        Kurdish = 67,
        Rundi = 68,
        Lao = 69,
        Latin = 70,
        Latvian = 71,
        Lingala = 72,
        Lithuanian = 73,
        Macedonian = 74,
        Malagasy = 75,
        Malay = 76,
        Malayalam = 77,
        Maltese = 78,
        Maori = 79,
        Marathi = 80,
        Marshallese = 81,
        Mongolian = 82,
        NauruLanguage = 83,
        Nepali = 84,
        NorwegianBokmal = 85,
        Occitan = 86,
        Oriya = 87,
        Pashto = 88,
        Persian = 89,
        Polish = 90,
        Portuguese = 91,
        Punjabi = 92,
        Quechua = 93,
        Romansh = 94,
        Romanian = 95,
        Russian = 96,
        Samoan = 97,
        Sango = 98,
        Sanskrit = 99,
        Serbian = 100,
        Ossetic = 101,
        SouthernSotho = 102,
        Tswana = 103,
        Shona = 104,
        Sindhi = 105,
        Sinhala = 106,
        Swati = 107,
        Slovak = 108,
        Slovenian = 109,
        Somali = 110,
        Spanish = 111,
        Sundanese = 112,
        Swahili = 113,
        Swedish = 114,
        Sardinian = 115,
        Tajik = 116,
        Tamil = 117,
        Tatar = 118,
        Telugu = 119,
        Thai = 120,
        Tibetan = 121,
        Tigrinya = 122,
        Tongan = 123,
        Tsonga = 124,
        Turkish = 125,
        Turkmen = 126,
        Tahitian = 127,
        Uighur = 128,
        Ukrainian = 129,
        Urdu = 130,
        Uzbek = 131,
        Vietnamese = 132,
        Volapuk = 133,
        Welsh = 134,
        Wolof = 135,
        Xhosa = 136,
        Yiddish = 137,
        Yoruba = 138,
        Zhuang = 139,
        Zulu = 140,
        NorwegianNynorsk = 141,
        Bosnian = 142,
        Divehi = 143,
        Manx = 144,
        Cornish = 145,
        Akan = 146,
        Konkani = 147,
        Ga = 148,
        Igbo = 149,
        Kamba = 150,
        Syriac = 151,
        Blin = 152,
        Geez = 153,
        Koro = 154,
        Sidamo = 155,
        Atsam = 156,
        Tigre = 157,
        Jju = 158,
        Friulian = 159,
        Venda = 160,
        Ewe = 161,
        Walamo = 162,
        Hawaiian = 163,
        Tyap = 164,
        Nyanja = 165,
        Filipino = 166,
        SwissGerman = 167,
        SichuanYi = 168,
        Kpelle = 169,
        LowGerman = 170,
        SouthNdebele = 171,
        NorthernSotho = 172,
        NorthernSami = 173,
        Taroko = 174,
        Gusii = 175,
        Taita = 176,
        Fulah = 177,
        Kikuyu = 178,
        Samburu = 179,
        Sena = 180,
        NorthNdebele = 181,
        Rombo = 182,
        Tachelhit = 183,
        Kabyle = 184,
        Nyankole = 185,
        Bena = 186,
        Vunjo = 187,
        Bambara = 188,
        Embu = 189,
        Cherokee = 190,
        Morisyen = 191,
        Makonde = 192,
        Langi = 193,
        Ganda = 194,
        Bemba = 195,
        Kabuverdianu = 196,
        Meru = 197,
        Kalenjin = 198,
        Nama = 199,
        Machame = 200,
        Colognian = 201,
        Masai = 202,
        Soga = 203,
        Luyia = 204,
        Asu = 205,
        Teso = 206,
        Saho = 207,
        KoyraChiini = 208,
        Rwa = 209,
        Luo = 210,
        Chiga = 211,
        CentralMoroccoTamazight = 212,
        KoyraboroSenni = 213,
        Shambala = 214,
        Bodo = 215,
        Avaric = 216,
        Chamorro = 217,
        Chechen = 218,
        Church = 219,
        Chuvash = 220,
        Cree = 221,
        Haitian = 222,
        Herero = 223,
        HiriMotu = 224,
        Kanuri = 225,
        Komi = 226,
        Kongo = 227,
        Kwanyama = 228,
        Limburgish = 229,
        LubaKatanga = 230,
        Luxembourgish = 231,
        Navaho = 232,
        Ndonga = 233,
        Ojibwa = 234,
        Pali = 235,
        Walloon = 236,
        Aghem = 237,
        Basaa = 238,
        Zarma = 239,
        Duala = 240,
        JolaFonyi = 241,
        Ewondo = 242,
        Bafia = 243,
        MakhuwaMeetto = 244,
        Mundang = 245,
        Kwasio = 246,
        Nuer = 247,
        Sakha = 248,
        Sangu = 249,
#if QT_DEPRECATED_SINCE(5, 15)
        CongoSwahili Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 250,
#endif
        Tasawaq = 251,
        Vai = 252,
        Walser = 253,
        Yangben = 254,
        Avestan = 255,
        Asturian = 256,
        Ngomba = 257,
        Kako = 258,
        Meta = 259,
        Ngiemboon = 260,
        Aragonese = 261,
        Akkadian = 262,
        AncientEgyptian = 263,
        AncientGreek = 264,
        Aramaic = 265,
        Balinese = 266,
        Bamun = 267,
        BatakToba = 268,
        Buginese = 269,
#if QT_DEPRECATED_SINCE(5, 15)
        Buhid Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 270,
        Carian Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 271,
#endif
        Chakma = 272,
#if QT_DEPRECATED_SINCE(5, 15)
        ClassicalMandaic Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 273,
#endif
        Coptic = 274,
        Dogri = 275,
#if QT_DEPRECATED_SINCE(5, 15)
        EasternCham Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 276,
        EasternKayah Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 277,
        Etruscan Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 278,
#endif
        Gothic = 279,
#if QT_DEPRECATED_SINCE(5, 15)
        Hanunoo Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 280,
#endif
        Ingush = 281,
#if QT_DEPRECATED_SINCE(5, 15)
        LargeFloweryMiao Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 282,
        Lepcha Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 283,
        Limbu Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 284,
        Lisu Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 285,
        Lu Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 286,
        Lycian Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 287,
        Lydian Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 288,
#endif
        Mandingo = 289,
        Manipuri = 290,
#if QT_DEPRECATED_SINCE(5, 15)
        Meroitic Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 291,
        NorthernThai Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 292,
#endif
        OldIrish = 293,
        OldNorse = 294,
        OldPersian = 295,
#if QT_DEPRECATED_SINCE(5, 15)
        OldTurkish Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 296,
#endif
        Pahlavi = 297,
#if QT_DEPRECATED_SINCE(5, 15)
        Parthian Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 298,
#endif
        Phoenician = 299,
#if QT_DEPRECATED_SINCE(5, 15)
        PrakritLanguage Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 300,
        Rejang Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 301,
        Sabaean Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 302,
        Samaritan Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 303,
#endif
        Santali = 304,
        Saurashtra = 305,
#if QT_DEPRECATED_SINCE(5, 15)
        Sora Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 306,
        Sylheti Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 307,
        Tagbanwa Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 308,
#endif
        TaiDam = 309,
#if QT_DEPRECATED_SINCE(5, 15)
        TaiNua Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 310,
#endif
        Ugaritic = 311,
        Akoose = 312,
        Lakota = 313,
        StandardMoroccanTamazight = 314,
        Mapuche = 315,
        CentralKurdish = 316,
        LowerSorbian = 317,
        UpperSorbian = 318,
        Kenyang = 319,
        Mohawk = 320,
        Nko = 321,
        Prussian = 322,
        Kiche = 323,
        SouthernSami = 324,
        LuleSami = 325,
        InariSami = 326,
        SkoltSami = 327,
        Warlpiri = 328,
#if QT_DEPRECATED_SINCE(5, 15)
        ManichaeanMiddlePersian Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 329,
#endif
        Mende = 330,
#if QT_DEPRECATED_SINCE(5, 15)
        AncientNorthArabian Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 331,
        LinearA Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 332,
        HmongNjua Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 333,
        Ho Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 334,
#endif
        Lezghian = 335,
#if QT_DEPRECATED_SINCE(5, 15)
        Bassa Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 336,
        Mono Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 337,
        TedimChin Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 338,
#endif
        Maithili = 339,
#if QT_DEPRECATED_SINCE(5, 15)
        Ahom Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 340,
#endif
        AmericanSignLanguage = 341,
#if QT_DEPRECATED_SINCE(5, 15)
        ArdhamagadhiPrakrit Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 342,
#endif
        Bhojpuri = 343,
#if QT_DEPRECATED_SINCE(5, 15)
        HieroglyphicLuwian Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 344,
#endif
        LiteraryChinese = 345,
        Mazanderani = 346,
#if QT_DEPRECATED_SINCE(5, 15)
        Mru Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 347,
#endif
        Newari = 348,
        NorthernLuri = 349,
        Palauan = 350,
        Papiamento = 351,
#if QT_DEPRECATED_SINCE(5, 15)
        Saraiki Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 352,
#endif
        TokelauLanguage = 353,
        TokPisin = 354,
        TuvaluLanguage = 355,
#if QT_DEPRECATED_SINCE(5, 15)
        UncodedLanguages Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 356,
#endif
        Cantonese = 357,
        Osage = 358,
#if QT_DEPRECATED_SINCE(5, 15)
        Tangut Q_DECL_ENUMERATOR_DEPRECATED_X("No locale data for this language") = 359,
#endif
        Ido = 360,
        Lojban = 361,
        Sicilian = 362,
        SouthernKurdish = 363,
        WesternBalochi = 364,
        Cebuano = 365,
        Erzya = 366,
        Chickasaw = 367,
        Muscogee = 368,
        Silesian = 369,
        NigerianPidgin = 370,

        Afan = Oromo,
        Bhutani = Dzongkha,
        Byelorussian = Belarusian,
        Cambodian = Khmer,
        Chewa = Nyanja,
        Frisian = WesternFrisian,
        Kurundi = Rundi,
#if QT_DEPRECATED_SINCE(5, 15)
        Moldavian Q_DECL_ENUMERATOR_DEPRECATED_X("Obsolete name, use Romanian") = Romanian,
        Norwegian Q_DECL_ENUMERATOR_DEPRECATED_X("Obsolete name, use NorwegianBokmal") = NorwegianBokmal,
#endif
        RhaetoRomance = Romansh,
#if QT_DEPRECATED_SINCE(5, 15)
        SerboCroatian Q_DECL_ENUMERATOR_DEPRECATED_X("Obsolete name, use Serbian") = Serbian,
        Tagalog Q_DECL_ENUMERATOR_DEPRECATED_X("Obsolete name, use Filipino") = Filipino,
        Twi Q_DECL_ENUMERATOR_DEPRECATED_X("Obsolete name, use Akan") = Akan,
#endif
        Uigur = Uighur,

        LastLanguage = NigerianPidgin
    };

    enum Script {
        AnyScript = 0,
        ArabicScript = 1,
        CyrillicScript = 2,
        DeseretScript = 3,
        GurmukhiScript = 4,
        SimplifiedHanScript = 5,
        TraditionalHanScript = 6,
        LatinScript = 7,
        MongolianScript = 8,
        TifinaghScript = 9,
        ArmenianScript = 10,
        BengaliScript = 11,
        CherokeeScript = 12,
        DevanagariScript = 13,
        EthiopicScript = 14,
        GeorgianScript = 15,
        GreekScript = 16,
        GujaratiScript = 17,
        HebrewScript = 18,
        JapaneseScript = 19,
        KhmerScript = 20,
        KannadaScript = 21,
        KoreanScript = 22,
        LaoScript = 23,
        MalayalamScript = 24,
        MyanmarScript = 25,
        OriyaScript = 26,
        TamilScript = 27,
        TeluguScript = 28,
        ThaanaScript = 29,
        ThaiScript = 30,
        TibetanScript = 31,
        SinhalaScript = 32,
        SyriacScript = 33,
        YiScript = 34,
        VaiScript = 35,
        AvestanScript = 36,
        BalineseScript = 37,
        BamumScript = 38,
        BatakScript = 39,
        BopomofoScript = 40,
        BrahmiScript = 41,
        BugineseScript = 42,
        BuhidScript = 43,
        CanadianAboriginalScript = 44,
        CarianScript = 45,
        ChakmaScript = 46,
        ChamScript = 47,
        CopticScript = 48,
        CypriotScript = 49,
        EgyptianHieroglyphsScript = 50,
        FraserScript = 51,
        GlagoliticScript = 52,
        GothicScript = 53,
        HanScript = 54,
        HangulScript = 55,
        HanunooScript = 56,
        ImperialAramaicScript = 57,
        InscriptionalPahlaviScript = 58,
        InscriptionalParthianScript = 59,
        JavaneseScript = 60,
        KaithiScript = 61,
        KatakanaScript = 62,
        KayahLiScript = 63,
        KharoshthiScript = 64,
        LannaScript = 65,
        LepchaScript = 66,
        LimbuScript = 67,
        LinearBScript = 68,
        LycianScript = 69,
        LydianScript = 70,
        MandaeanScript = 71,
        MeiteiMayekScript = 72,
        MeroiticScript = 73,
        MeroiticCursiveScript = 74,
        NkoScript = 75,
        NewTaiLueScript = 76,
        OghamScript = 77,
        OlChikiScript = 78,
        OldItalicScript = 79,
        OldPersianScript = 80,
        OldSouthArabianScript = 81,
        OrkhonScript = 82,
        OsmanyaScript = 83,
        PhagsPaScript = 84,
        PhoenicianScript = 85,
        PollardPhoneticScript = 86,
        RejangScript = 87,
        RunicScript = 88,
        SamaritanScript = 89,
        SaurashtraScript = 90,
        SharadaScript = 91,
        ShavianScript = 92,
        SoraSompengScript = 93,
        CuneiformScript = 94,
        SundaneseScript = 95,
        SylotiNagriScript = 96,
        TagalogScript = 97,
        TagbanwaScript = 98,
        TaiLeScript = 99,
        TaiVietScript = 100,
        TakriScript = 101,
        UgariticScript = 102,
        BrailleScript = 103,
        HiraganaScript = 104,
        CaucasianAlbanianScript = 105,
        BassaVahScript = 106,
        DuployanScript = 107,
        ElbasanScript = 108,
        GranthaScript = 109,
        PahawhHmongScript = 110,
        KhojkiScript = 111,
        LinearAScript = 112,
        MahajaniScript = 113,
        ManichaeanScript = 114,
        MendeKikakuiScript = 115,
        ModiScript = 116,
        MroScript = 117,
        OldNorthArabianScript = 118,
        NabataeanScript = 119,
        PalmyreneScript = 120,
        PauCinHauScript = 121,
        OldPermicScript = 122,
        PsalterPahlaviScript = 123,
        SiddhamScript = 124,
        KhudawadiScript = 125,
        TirhutaScript = 126,
        VarangKshitiScript = 127,
        AhomScript = 128,
        AnatolianHieroglyphsScript = 129,
        HatranScript = 130,
        MultaniScript = 131,
        OldHungarianScript = 132,
        SignWritingScript = 133,
        AdlamScript = 134,
        BhaiksukiScript = 135,
        MarchenScript = 136,
        NewaScript = 137,
        OsageScript = 138,
        TangutScript = 139,
        HanWithBopomofoScript = 140,
        JamoScript = 141,

        SimplifiedChineseScript = SimplifiedHanScript,
        TraditionalChineseScript = TraditionalHanScript,

        LastScript = JamoScript
    };

    enum Country {
        AnyCountry = 0,
        Afghanistan = 1,
        Albania = 2,
        Algeria = 3,
        AmericanSamoa = 4,
        Andorra = 5,
        Angola = 6,
        Anguilla = 7,
        Antarctica = 8,
        AntiguaAndBarbuda = 9,
        Argentina = 10,
        Armenia = 11,
        Aruba = 12,
        Australia = 13,
        Austria = 14,
        Azerbaijan = 15,
        Bahamas = 16,
        Bahrain = 17,
        Bangladesh = 18,
        Barbados = 19,
        Belarus = 20,
        Belgium = 21,
        Belize = 22,
        Benin = 23,
        Bermuda = 24,
        Bhutan = 25,
        Bolivia = 26,
        BosniaAndHerzegowina = 27,
        Botswana = 28,
        BouvetIsland = 29,
        Brazil = 30,
        BritishIndianOceanTerritory = 31,
        Brunei = 32,
        Bulgaria = 33,
        BurkinaFaso = 34,
        Burundi = 35,
        Cambodia = 36,
        Cameroon = 37,
        Canada = 38,
        CapeVerde = 39,
        CaymanIslands = 40,
        CentralAfricanRepublic = 41,
        Chad = 42,
        Chile = 43,
        China = 44,
        ChristmasIsland = 45,
        CocosIslands = 46,
        Colombia = 47,
        Comoros = 48,
        CongoKinshasa = 49,
        CongoBrazzaville = 50,
        CookIslands = 51,
        CostaRica = 52,
        IvoryCoast = 53,
        Croatia = 54,
        Cuba = 55,
        Cyprus = 56,
        CzechRepublic = 57,
        Denmark = 58,
        Djibouti = 59,
        Dominica = 60,
        DominicanRepublic = 61,
        EastTimor = 62,
        Ecuador = 63,
        Egypt = 64,
        ElSalvador = 65,
        EquatorialGuinea = 66,
        Eritrea = 67,
        Estonia = 68,
        Ethiopia = 69,
        FalklandIslands = 70,
        FaroeIslands = 71,
        Fiji = 72,
        Finland = 73,
        France = 74,
        Guernsey = 75,
        FrenchGuiana = 76,
        FrenchPolynesia = 77,
        FrenchSouthernTerritories = 78,
        Gabon = 79,
        Gambia = 80,
        Georgia = 81,
        Germany = 82,
        Ghana = 83,
        Gibraltar = 84,
        Greece = 85,
        Greenland = 86,
        Grenada = 87,
        Guadeloupe = 88,
        Guam = 89,
        Guatemala = 90,
        Guinea = 91,
        GuineaBissau = 92,
        Guyana = 93,
        Haiti = 94,
        HeardAndMcDonaldIslands = 95,
        Honduras = 96,
        HongKong = 97,
        Hungary = 98,
        Iceland = 99,
        India = 100,
        Indonesia = 101,
        Iran = 102,
        Iraq = 103,
        Ireland = 104,
        Israel = 105,
        Italy = 106,
        Jamaica = 107,
        Japan = 108,
        Jordan = 109,
        Kazakhstan = 110,
        Kenya = 111,
        Kiribati = 112,
        NorthKorea = 113,
        SouthKorea = 114,
        Kuwait = 115,
        Kyrgyzstan = 116,
        Laos = 117,
        Latvia = 118,
        Lebanon = 119,
        Lesotho = 120,
        Liberia = 121,
        Libya = 122,
        Liechtenstein = 123,
        Lithuania = 124,
        Luxembourg = 125,
        Macau = 126,
        Macedonia = 127,
        Madagascar = 128,
        Malawi = 129,
        Malaysia = 130,
        Maldives = 131,
        Mali = 132,
        Malta = 133,
        MarshallIslands = 134,
        Martinique = 135,
        Mauritania = 136,
        Mauritius = 137,
        Mayotte = 138,
        Mexico = 139,
        Micronesia = 140,
        Moldova = 141,
        Monaco = 142,
        Mongolia = 143,
        Montserrat = 144,
        Morocco = 145,
        Mozambique = 146,
        Myanmar = 147,
        Namibia = 148,
        NauruCountry = 149,
        Nepal = 150,
        Netherlands = 151,
        CuraSao = 152,
        NewCaledonia = 153,
        NewZealand = 154,
        Nicaragua = 155,
        Niger = 156,
        Nigeria = 157,
        Niue = 158,
        NorfolkIsland = 159,
        NorthernMarianaIslands = 160,
        Norway = 161,
        Oman = 162,
        Pakistan = 163,
        Palau = 164,
        PalestinianTerritories = 165,
        Panama = 166,
        PapuaNewGuinea = 167,
        Paraguay = 168,
        Peru = 169,
        Philippines = 170,
        Pitcairn = 171,
        Poland = 172,
        Portugal = 173,
        PuertoRico = 174,
        Qatar = 175,
        Reunion = 176,
        Romania = 177,
        Russia = 178,
        Rwanda = 179,
        SaintKittsAndNevis = 180,
        SaintLucia = 181,
        SaintVincentAndTheGrenadines = 182,
        Samoa = 183,
        SanMarino = 184,
        SaoTomeAndPrincipe = 185,
        SaudiArabia = 186,
        Senegal = 187,
        Seychelles = 188,
        SierraLeone = 189,
        Singapore = 190,
        Slovakia = 191,
        Slovenia = 192,
        SolomonIslands = 193,
        Somalia = 194,
        SouthAfrica = 195,
        SouthGeorgiaAndTheSouthSandwichIslands = 196,
        Spain = 197,
        SriLanka = 198,
        SaintHelena = 199,
        SaintPierreAndMiquelon = 200,
        Sudan = 201,
        Suriname = 202,
        SvalbardAndJanMayenIslands = 203,
        Swaziland = 204,
        Sweden = 205,
        Switzerland = 206,
        Syria = 207,
        Taiwan = 208,
        Tajikistan = 209,
        Tanzania = 210,
        Thailand = 211,
        Togo = 212,
        TokelauCountry = 213,
        Tonga = 214,
        TrinidadAndTobago = 215,
        Tunisia = 216,
        Turkey = 217,
        Turkmenistan = 218,
        TurksAndCaicosIslands = 219,
        TuvaluCountry = 220,
        Uganda = 221,
        Ukraine = 222,
        UnitedArabEmirates = 223,
        UnitedKingdom = 224,
        UnitedStates = 225,
        UnitedStatesMinorOutlyingIslands = 226,
        Uruguay = 227,
        Uzbekistan = 228,
        Vanuatu = 229,
        VaticanCityState = 230,
        Venezuela = 231,
        Vietnam = 232,
        BritishVirginIslands = 233,
        UnitedStatesVirginIslands = 234,
        WallisAndFutunaIslands = 235,
        WesternSahara = 236,
        Yemen = 237,
        CanaryIslands = 238,
        Zambia = 239,
        Zimbabwe = 240,
        ClippertonIsland = 241,
        Montenegro = 242,
        Serbia = 243,
        SaintBarthelemy = 244,
        SaintMartin = 245,
        LatinAmerica = 246,
        AscensionIsland = 247,
        AlandIslands = 248,
        DiegoGarcia = 249,
        CeutaAndMelilla = 250,
        IsleOfMan = 251,
        Jersey = 252,
        TristanDaCunha = 253,
        SouthSudan = 254,
        Bonaire = 255,
        SintMaarten = 256,
        Kosovo = 257,
        EuropeanUnion = 258,
        OutlyingOceania = 259,
        World = 260,
        Europe = 261,

        DemocraticRepublicOfCongo = CongoKinshasa,
        DemocraticRepublicOfKorea = NorthKorea,
        LatinAmericaAndTheCaribbean = LatinAmerica,
        PeoplesRepublicOfCongo = CongoBrazzaville,
        RepublicOfKorea = SouthKorea,
        RussianFederation = Russia,
        SyrianArabRepublic = Syria,
#if QT_DEPRECATED_SINCE(5, 15)
        Tokelau Q_DECL_ENUMERATOR_DEPRECATED_X("Obsolete name, use TokelauCountry") = TokelauCountry,
        Tuvalu Q_DECL_ENUMERATOR_DEPRECATED_X("Obsolete name, use TuvaluCountry") = TuvaluCountry,
#endif

        LastCountry = Europe
    };
// GENERATED PART ENDS HERE

    Q_ENUM(Language)
    Q_ENUM(Country)
    Q_ENUM(Script)

    enum MeasurementSystem {
        MetricSystem,
        ImperialUSSystem,
        ImperialUKSystem,
        ImperialSystem = ImperialUSSystem // Qt 4 compatibility
    };
    Q_ENUM(MeasurementSystem)

    enum FormatType { LongFormat, ShortFormat, NarrowFormat };
    enum NumberOption {
        DefaultNumberOptions = 0x0,
        OmitGroupSeparator = 0x01,
        RejectGroupSeparator = 0x02,
        OmitLeadingZeroInExponent = 0x04,
        RejectLeadingZeroInExponent = 0x08,
        IncludeTrailingZeroesAfterDot = 0x10,
        RejectTrailingZeroesAfterDot = 0x20
    };
    Q_DECLARE_FLAGS(NumberOptions, NumberOption)

    enum FloatingPointPrecisionOption {
        FloatingPointShortest = -128
    };

    enum CurrencySymbolFormat {
        CurrencyIsoCode,
        CurrencySymbol,
        CurrencyDisplayName
    };

    enum DataSizeFormat {
        // Single-bit values, for internal use.
        DataSizeBase1000 = 1, // use factors of 1000 instead of IEC's 1024;
        DataSizeSIQuantifiers = 2, // use SI quantifiers instead of IEC ones.

        // Flags values for use in API:
        DataSizeIecFormat = 0, // base 1024, KiB, MiB, GiB, ...
        DataSizeTraditionalFormat = DataSizeSIQuantifiers, // base 1024, kB, MB, GB, ...
        DataSizeSIFormat = DataSizeBase1000 | DataSizeSIQuantifiers // base 1000, kB, MB, GB, ...
    };
    Q_DECLARE_FLAGS(DataSizeFormats, DataSizeFormat)
    Q_FLAG(DataSizeFormats)

    QLocale();
    QLocale(const QString &name);
    QLocale(Language language, Country country = AnyCountry);
    QLocale(Language language, Script script, Country country);
    QLocale(const QLocale &other);
    QLocale &operator=(QLocale &&other) noexcept { swap(other); return *this; }
    QLocale &operator=(const QLocale &other);
    ~QLocale();

    void swap(QLocale &other) noexcept { qSwap(d, other.d); }

    Language language() const;
    Script script() const;
    Country country() const;
    QString name() const;

    QString bcp47Name() const;
    QString nativeLanguageName() const;
    QString nativeCountryName() const;

#if QT_STRINGVIEW_LEVEL < 2
    short toShort(const QString &s, bool *ok = nullptr) const;
    ushort toUShort(const QString &s, bool *ok = nullptr) const;
    int toInt(const QString &s, bool *ok = nullptr) const;
    uint toUInt(const QString &s, bool *ok = nullptr) const;
    long toLong(const QString &s, bool *ok = nullptr) const;
    ulong toULong(const QString &s, bool *ok = nullptr) const;
    qlonglong toLongLong(const QString &s, bool *ok = nullptr) const;
    qulonglong toULongLong(const QString &s, bool *ok = nullptr) const;
    float toFloat(const QString &s, bool *ok = nullptr) const;
    double toDouble(const QString &s, bool *ok = nullptr) const;

    short toShort(const QStringRef &s, bool *ok = nullptr) const;
    ushort toUShort(const QStringRef &s, bool *ok = nullptr) const;
    int toInt(const QStringRef &s, bool *ok = nullptr) const;
    uint toUInt(const QStringRef &s, bool *ok = nullptr) const;
    long toLong(const QStringRef &s, bool *ok = nullptr) const;
    ulong toULong(const QStringRef &s, bool *ok = nullptr) const;
    qlonglong toLongLong(const QStringRef &s, bool *ok = nullptr) const;
    qulonglong toULongLong(const QStringRef &s, bool *ok = nullptr) const;
    float toFloat(const QStringRef &s, bool *ok = nullptr) const;
    double toDouble(const QStringRef &s, bool *ok = nullptr) const;
#endif

    short toShort(QStringView s, bool *ok = nullptr) const;
    ushort toUShort(QStringView s, bool *ok = nullptr) const;
    int toInt(QStringView s, bool *ok = nullptr) const;
    uint toUInt(QStringView s, bool *ok = nullptr) const;
    long toLong(QStringView s, bool *ok = nullptr) const;
    ulong toULong(QStringView s, bool *ok = nullptr) const;
    qlonglong toLongLong(QStringView s, bool *ok = nullptr) const;
    qulonglong toULongLong(QStringView s, bool *ok = nullptr) const;
    float toFloat(QStringView s, bool *ok = nullptr) const;
    double toDouble(QStringView s, bool *ok = nullptr) const;

    QString toString(qlonglong i) const;
    QString toString(qulonglong i) const;
    inline QString toString(long i) const;
    inline QString toString(ulong i) const;
    inline QString toString(short i) const;
    inline QString toString(ushort i) const;
    inline QString toString(int i) const;
    inline QString toString(uint i) const;
    QString toString(double i, char f = 'g', int prec = 6) const;
    inline QString toString(float i, char f = 'g', int prec = 6) const;

#if QT_STRINGVIEW_LEVEL < 2
    QString toString(const QDate &date, const QString &formatStr) const;
    QString toString(const QTime &time, const QString &formatStr) const;
    QString toString(const QDateTime &dateTime, const QString &format) const;
#endif
    QString toString(const QDate &date, QStringView formatStr) const;
    QString toString(const QTime &time, QStringView formatStr) const;
    QString toString(const QDateTime &dateTime, QStringView format) const;
    QString toString(const QDate &date, FormatType format = LongFormat) const;
    QString toString(const QTime &time, FormatType format = LongFormat) const;
    QString toString(const QDateTime &dateTime, FormatType format = LongFormat) const;
    /* Removing default value for `format' is done intentionally,
     * after all tests we will remove non-calendar-aware version of these functions,
     * and add a default value for both calendar instance, and format
     */
    QString toString(const QDate &date, QStringView formatStr, QCalendar cal) const;
    QString toString(const QDate &date, FormatType format, QCalendar cal) const;
    QString toString(const QDateTime &dateTime, FormatType format, QCalendar cal) const;
    QString toString(const QDateTime &dateTime, QStringView formatStr, QCalendar cal) const;

    QString dateFormat(FormatType format = LongFormat) const;
    QString timeFormat(FormatType format = LongFormat) const;
    QString dateTimeFormat(FormatType format = LongFormat) const;
#if QT_CONFIG(datestring)
    QDate toDate(const QString &string, FormatType = LongFormat) const;
    QTime toTime(const QString &string, FormatType = LongFormat) const;
    QDateTime toDateTime(const QString &string, FormatType format = LongFormat) const;
    QDate toDate(const QString &string, const QString &format) const;
    QTime toTime(const QString &string, const QString &format) const;
    QDateTime toDateTime(const QString &string, const QString &format) const;
    // Calendar-aware API
    QDate toDate(const QString &string, FormatType format, QCalendar cal) const;
    QDateTime toDateTime(const QString &string, FormatType format, QCalendar cal) const;
    QDate toDate(const QString &string, const QString &format, QCalendar cal) const;
    QDateTime toDateTime(const QString &string, const QString &format, QCalendar cal) const;
# if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X("Calendar is ignored when parsing times")
    QTime toTime(const QString &string, FormatType format, QCalendar cal) const;
    QT_DEPRECATED_X("Calendar is ignored when parsing times")
    QTime toTime(const QString &string, const QString &format, QCalendar cal) const;
# endif // 5.15
#endif

    // ### Qt 6: We need to return QString from these function since
    //           UTF-16 may need surrogate pairs to represent these fields.
    QChar decimalPoint() const;
    QChar groupSeparator() const;
    QChar percent() const;
    QChar zeroDigit() const;
    QChar negativeSign() const;
    QChar positiveSign() const;
    QChar exponential() const;

    QString monthName(int, FormatType format = LongFormat) const;
    QString standaloneMonthName(int, FormatType format = LongFormat) const;
    QString dayName(int, FormatType format = LongFormat) const;
    QString standaloneDayName(int, FormatType format = LongFormat) const;

    Qt::DayOfWeek firstDayOfWeek() const;
    QList<Qt::DayOfWeek> weekdays() const;

    QString amText() const;
    QString pmText() const;

    MeasurementSystem measurementSystem() const;
    QLocale collation() const;
    Qt::LayoutDirection textDirection() const;

    QString toUpper(const QString &str) const;
    QString toLower(const QString &str) const;

    QString currencySymbol(CurrencySymbolFormat = CurrencySymbol) const;
    QString toCurrencyString(qlonglong, const QString &symbol = QString()) const;
    QString toCurrencyString(qulonglong, const QString &symbol = QString()) const;
    inline QString toCurrencyString(short, const QString &symbol = QString()) const;
    inline QString toCurrencyString(ushort, const QString &symbol = QString()) const;
    inline QString toCurrencyString(int, const QString &symbol = QString()) const;
    inline QString toCurrencyString(uint, const QString &symbol = QString()) const;
#if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
    QString toCurrencyString(double, const QString &symbol = QString(), int precision = -1) const;
    inline QString toCurrencyString(float i, const QString &symbol = QString(), int precision = -1) const
    { return toCurrencyString(double(i), symbol, precision); }
#else
    QString toCurrencyString(double, const QString &symbol = QString()) const;
    QString toCurrencyString(double, const QString &symbol, int precision) const;
    inline QString toCurrencyString(float i, const QString &symbol = QString()) const
    { return toCurrencyString(double(i), symbol); }
    inline QString toCurrencyString(float i, const QString &symbol, int precision) const
    { return toCurrencyString(double(i), symbol, precision); }
#endif

#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    QString formattedDataSize(qint64 bytes, int precision = 2, DataSizeFormats format = DataSizeIecFormat);
#endif
    QString formattedDataSize(qint64 bytes, int precision = 2, DataSizeFormats format = DataSizeIecFormat) const;

    QStringList uiLanguages() const;

    bool operator==(const QLocale &other) const;
    bool operator!=(const QLocale &other) const;

    static QString languageToString(Language language);
    static QString countryToString(Country country);
    static QString scriptToString(Script script);
    static void setDefault(const QLocale &locale);

    static QLocale c() { return QLocale(C); }
    static QLocale system();

    static QList<QLocale> matchingLocales(QLocale::Language language, QLocale::Script script, QLocale::Country country);
    static QList<Country> countriesForLanguage(Language lang);

    void setNumberOptions(NumberOptions options);
    NumberOptions numberOptions() const;

    enum QuotationStyle { StandardQuotation, AlternateQuotation };
    QString quoteString(const QString &str, QuotationStyle style = StandardQuotation) const;
    QString quoteString(const QStringRef &str, QuotationStyle style = StandardQuotation) const;

    QString createSeparatedList(const QStringList &strl) const;

private:
    QLocale(QLocalePrivate &dd);
    friend class QLocalePrivate;
    friend class QSystemLocale;
    friend class QCalendarBackend;
    friend class QGregorianCalendar;
    friend Q_CORE_EXPORT uint qHash(const QLocale &key, uint seed) noexcept;

    QSharedDataPointer<QLocalePrivate> d;
};
Q_DECLARE_SHARED(QLocale)
Q_DECLARE_OPERATORS_FOR_FLAGS(QLocale::NumberOptions)

inline QString QLocale::toString(long i) const
    { return toString(qlonglong(i)); }
inline QString QLocale::toString(ulong i) const
    { return toString(qulonglong(i)); }
inline QString QLocale::toString(short i) const
    { return toString(qlonglong(i)); }
inline QString QLocale::toString(ushort i) const
    { return toString(qulonglong(i)); }
inline QString QLocale::toString(int i) const
    { return toString(qlonglong(i)); }
inline QString QLocale::toString(uint i) const
    { return toString(qulonglong(i)); }
inline QString QLocale::toString(float i, char f, int prec) const
    { return toString(double(i), f, prec); }

inline QString QLocale::toCurrencyString(short i, const QString &symbol) const
    { return toCurrencyString(qlonglong(i), symbol); }
inline QString QLocale::toCurrencyString(ushort i, const QString &symbol) const
    { return toCurrencyString(qulonglong(i), symbol); }
inline QString QLocale::toCurrencyString(int i, const QString &symbol) const
{ return toCurrencyString(qlonglong(i), symbol); }
inline QString QLocale::toCurrencyString(uint i, const QString &symbol) const
{ return toCurrencyString(qulonglong(i), symbol); }

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QLocale &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QLocale &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QLocale &);
#endif

QT_END_NAMESPACE

#endif // QLOCALE_H
