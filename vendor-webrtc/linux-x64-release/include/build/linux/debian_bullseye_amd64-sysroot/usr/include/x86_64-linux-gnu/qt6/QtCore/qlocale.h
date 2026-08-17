// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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

Q_CORE_EXPORT size_t qHash(const QLocale &key, size_t seed = 0) noexcept;

class Q_CORE_EXPORT QLocale
{
    Q_GADGET
    friend class QString;
    friend class QByteArray;
    friend class QIntValidator;
    friend class QDoubleValidatorPrivate;
    friend class QTextStream;
    friend class QTextStreamPrivate;

public:
// see qlocale_data_p.h for more info on generated data
// GENERATED PART STARTS HERE
    enum Language : ushort {
        AnyLanguage = 0,
        C = 1,
        Abkhazian = 2,
        Afar = 3,
        Afrikaans = 4,
        Aghem = 5,
        Akan = 6,
        Akkadian = 7,
        Akoose = 8,
        Albanian = 9,
        AmericanSignLanguage = 10,
        Amharic = 11,
        AncientEgyptian = 12,
        AncientGreek = 13,
        Arabic = 14,
        Aragonese = 15,
        Aramaic = 16,
        Armenian = 17,
        Assamese = 18,
        Asturian = 19,
        Asu = 20,
        Atsam = 21,
        Avaric = 22,
        Avestan = 23,
        Aymara = 24,
        Azerbaijani = 25,
        Bafia = 26,
        Balinese = 27,
        Bambara = 28,
        Bamun = 29,
        Bangla = 30,
        Basaa = 31,
        Bashkir = 32,
        Basque = 33,
        BatakToba = 34,
        Belarusian = 35,
        Bemba = 36,
        Bena = 37,
        Bhojpuri = 38,
        Bislama = 39,
        Blin = 40,
        Bodo = 41,
        Bosnian = 42,
        Breton = 43,
        Buginese = 44,
        Bulgarian = 45,
        Burmese = 46,
        Cantonese = 47,
        Catalan = 48,
        Cebuano = 49,
        CentralAtlasTamazight = 50,
        CentralKurdish = 51,
        Chakma = 52,
        Chamorro = 53,
        Chechen = 54,
        Cherokee = 55,
        Chickasaw = 56,
        Chiga = 57,
        Chinese = 58,
        Church = 59,
        Chuvash = 60,
        Colognian = 61,
        Coptic = 62,
        Cornish = 63,
        Corsican = 64,
        Cree = 65,
        Croatian = 66,
        Czech = 67,
        Danish = 68,
        Divehi = 69,
        Dogri = 70,
        Duala = 71,
        Dutch = 72,
        Dzongkha = 73,
        Embu = 74,
        English = 75,
        Erzya = 76,
        Esperanto = 77,
        Estonian = 78,
        Ewe = 79,
        Ewondo = 80,
        Faroese = 81,
        Fijian = 82,
        Filipino = 83,
        Finnish = 84,
        French = 85,
        Friulian = 86,
        Fulah = 87,
        Gaelic = 88,
        Ga = 89,
        Galician = 90,
        Ganda = 91,
        Geez = 92,
        Georgian = 93,
        German = 94,
        Gothic = 95,
        Greek = 96,
        Guarani = 97,
        Gujarati = 98,
        Gusii = 99,
        Haitian = 100,
        Hausa = 101,
        Hawaiian = 102,
        Hebrew = 103,
        Herero = 104,
        Hindi = 105,
        HiriMotu = 106,
        Hungarian = 107,
        Icelandic = 108,
        Ido = 109,
        Igbo = 110,
        InariSami = 111,
        Indonesian = 112,
        Ingush = 113,
        Interlingua = 114,
        Interlingue = 115,
        Inuktitut = 116,
        Inupiaq = 117,
        Irish = 118,
        Italian = 119,
        Japanese = 120,
        Javanese = 121,
        Jju = 122,
        JolaFonyi = 123,
        Kabuverdianu = 124,
        Kabyle = 125,
        Kako = 126,
        Kalaallisut = 127,
        Kalenjin = 128,
        Kamba = 129,
        Kannada = 130,
        Kanuri = 131,
        Kashmiri = 132,
        Kazakh = 133,
        Kenyang = 134,
        Khmer = 135,
        Kiche = 136,
        Kikuyu = 137,
        Kinyarwanda = 138,
        Komi = 139,
        Kongo = 140,
        Konkani = 141,
        Korean = 142,
        Koro = 143,
        KoyraboroSenni = 144,
        KoyraChiini = 145,
        Kpelle = 146,
        Kuanyama = 147,
        Kurdish = 148,
        Kwasio = 149,
        Kyrgyz = 150,
        Lakota = 151,
        Langi = 152,
        Lao = 153,
        Latin = 154,
        Latvian = 155,
        Lezghian = 156,
        Limburgish = 157,
        Lingala = 158,
        LiteraryChinese = 159,
        Lithuanian = 160,
        Lojban = 161,
        LowerSorbian = 162,
        LowGerman = 163,
        LubaKatanga = 164,
        LuleSami = 165,
        Luo = 166,
        Luxembourgish = 167,
        Luyia = 168,
        Macedonian = 169,
        Machame = 170,
        Maithili = 171,
        MakhuwaMeetto = 172,
        Makonde = 173,
        Malagasy = 174,
        Malayalam = 175,
        Malay = 176,
        Maltese = 177,
        Mandingo = 178,
        Manipuri = 179,
        Manx = 180,
        Maori = 181,
        Mapuche = 182,
        Marathi = 183,
        Marshallese = 184,
        Masai = 185,
        Mazanderani = 186,
        Mende = 187,
        Meru = 188,
        Meta = 189,
        Mohawk = 190,
        Mongolian = 191,
        Morisyen = 192,
        Mundang = 193,
        Muscogee = 194,
        Nama = 195,
        NauruLanguage = 196,
        Navajo = 197,
        Ndonga = 198,
        Nepali = 199,
        Newari = 200,
        Ngiemboon = 201,
        Ngomba = 202,
        NigerianPidgin = 203,
        Nko = 204,
        NorthernLuri = 205,
        NorthernSami = 206,
        NorthernSotho = 207,
        NorthNdebele = 208,
        NorwegianBokmal = 209,
        NorwegianNynorsk = 210,
        Nuer = 211,
        Nyanja = 212,
        Nyankole = 213,
        Occitan = 214,
        Odia = 215,
        Ojibwa = 216,
        OldIrish = 217,
        OldNorse = 218,
        OldPersian = 219,
        Oromo = 220,
        Osage = 221,
        Ossetic = 222,
        Pahlavi = 223,
        Palauan = 224,
        Pali = 225,
        Papiamento = 226,
        Pashto = 227,
        Persian = 228,
        Phoenician = 229,
        Polish = 230,
        Portuguese = 231,
        Prussian = 232,
        Punjabi = 233,
        Quechua = 234,
        Romanian = 235,
        Romansh = 236,
        Rombo = 237,
        Rundi = 238,
        Russian = 239,
        Rwa = 240,
        Saho = 241,
        Sakha = 242,
        Samburu = 243,
        Samoan = 244,
        Sango = 245,
        Sangu = 246,
        Sanskrit = 247,
        Santali = 248,
        Sardinian = 249,
        Saurashtra = 250,
        Sena = 251,
        Serbian = 252,
        Shambala = 253,
        Shona = 254,
        SichuanYi = 255,
        Sicilian = 256,
        Sidamo = 257,
        Silesian = 258,
        Sindhi = 259,
        Sinhala = 260,
        SkoltSami = 261,
        Slovak = 262,
        Slovenian = 263,
        Soga = 264,
        Somali = 265,
        SouthernKurdish = 266,
        SouthernSami = 267,
        SouthernSotho = 268,
        SouthNdebele = 269,
        Spanish = 270,
        StandardMoroccanTamazight = 271,
        Sundanese = 272,
        Swahili = 273,
        Swati = 274,
        Swedish = 275,
        SwissGerman = 276,
        Syriac = 277,
        Tachelhit = 278,
        Tahitian = 279,
        TaiDam = 280,
        Taita = 281,
        Tajik = 282,
        Tamil = 283,
        Taroko = 284,
        Tasawaq = 285,
        Tatar = 286,
        Telugu = 287,
        Teso = 288,
        Thai = 289,
        Tibetan = 290,
        Tigre = 291,
        Tigrinya = 292,
        TokelauLanguage = 293,
        TokPisin = 294,
        Tongan = 295,
        Tsonga = 296,
        Tswana = 297,
        Turkish = 298,
        Turkmen = 299,
        TuvaluLanguage = 300,
        Tyap = 301,
        Ugaritic = 302,
        Ukrainian = 303,
        UpperSorbian = 304,
        Urdu = 305,
        Uyghur = 306,
        Uzbek = 307,
        Vai = 308,
        Venda = 309,
        Vietnamese = 310,
        Volapuk = 311,
        Vunjo = 312,
        Walloon = 313,
        Walser = 314,
        Warlpiri = 315,
        Welsh = 316,
        WesternBalochi = 317,
        WesternFrisian = 318,
        Wolaytta = 319,
        Wolof = 320,
        Xhosa = 321,
        Yangben = 322,
        Yiddish = 323,
        Yoruba = 324,
        Zarma = 325,
        Zhuang = 326,
        Zulu = 327,
        Kaingang = 328,
        Nheengatu = 329,

        Afan = Oromo,
        Bengali = Bangla,
        Bhutani = Dzongkha,
        Byelorussian = Belarusian,
        Cambodian = Khmer,
        CentralMoroccoTamazight = CentralAtlasTamazight,
        Chewa = Nyanja,
        Frisian = WesternFrisian,
        Greenlandic = Kalaallisut,
        Inupiak = Inupiaq,
        Kirghiz = Kyrgyz,
        Kurundi = Rundi,
        Kwanyama = Kuanyama,
        Navaho = Navajo,
        Oriya = Odia,
        RhaetoRomance = Romansh,
        Uighur = Uyghur,
        Uigur = Uyghur,
        Walamo = Wolaytta,

        LastLanguage = Nheengatu
    };

    enum Script : ushort {
        AnyScript = 0,
        AdlamScript = 1,
        AhomScript = 2,
        AnatolianHieroglyphsScript = 3,
        ArabicScript = 4,
        ArmenianScript = 5,
        AvestanScript = 6,
        BalineseScript = 7,
        BamumScript = 8,
        BanglaScript = 9,
        BassaVahScript = 10,
        BatakScript = 11,
        BhaiksukiScript = 12,
        BopomofoScript = 13,
        BrahmiScript = 14,
        BrailleScript = 15,
        BugineseScript = 16,
        BuhidScript = 17,
        CanadianAboriginalScript = 18,
        CarianScript = 19,
        CaucasianAlbanianScript = 20,
        ChakmaScript = 21,
        ChamScript = 22,
        CherokeeScript = 23,
        CopticScript = 24,
        CuneiformScript = 25,
        CypriotScript = 26,
        CyrillicScript = 27,
        DeseretScript = 28,
        DevanagariScript = 29,
        DuployanScript = 30,
        EgyptianHieroglyphsScript = 31,
        ElbasanScript = 32,
        EthiopicScript = 33,
        FraserScript = 34,
        GeorgianScript = 35,
        GlagoliticScript = 36,
        GothicScript = 37,
        GranthaScript = 38,
        GreekScript = 39,
        GujaratiScript = 40,
        GurmukhiScript = 41,
        HangulScript = 42,
        HanScript = 43,
        HanunooScript = 44,
        HanWithBopomofoScript = 45,
        HatranScript = 46,
        HebrewScript = 47,
        HiraganaScript = 48,
        ImperialAramaicScript = 49,
        InscriptionalPahlaviScript = 50,
        InscriptionalParthianScript = 51,
        JamoScript = 52,
        JapaneseScript = 53,
        JavaneseScript = 54,
        KaithiScript = 55,
        KannadaScript = 56,
        KatakanaScript = 57,
        KayahLiScript = 58,
        KharoshthiScript = 59,
        KhmerScript = 60,
        KhojkiScript = 61,
        KhudawadiScript = 62,
        KoreanScript = 63,
        LannaScript = 64,
        LaoScript = 65,
        LatinScript = 66,
        LepchaScript = 67,
        LimbuScript = 68,
        LinearAScript = 69,
        LinearBScript = 70,
        LycianScript = 71,
        LydianScript = 72,
        MahajaniScript = 73,
        MalayalamScript = 74,
        MandaeanScript = 75,
        ManichaeanScript = 76,
        MarchenScript = 77,
        MeiteiMayekScript = 78,
        MendeScript = 79,
        MeroiticCursiveScript = 80,
        MeroiticScript = 81,
        ModiScript = 82,
        MongolianScript = 83,
        MroScript = 84,
        MultaniScript = 85,
        MyanmarScript = 86,
        NabataeanScript = 87,
        NewaScript = 88,
        NewTaiLueScript = 89,
        NkoScript = 90,
        OdiaScript = 91,
        OghamScript = 92,
        OlChikiScript = 93,
        OldHungarianScript = 94,
        OldItalicScript = 95,
        OldNorthArabianScript = 96,
        OldPermicScript = 97,
        OldPersianScript = 98,
        OldSouthArabianScript = 99,
        OrkhonScript = 100,
        OsageScript = 101,
        OsmanyaScript = 102,
        PahawhHmongScript = 103,
        PalmyreneScript = 104,
        PauCinHauScript = 105,
        PhagsPaScript = 106,
        PhoenicianScript = 107,
        PollardPhoneticScript = 108,
        PsalterPahlaviScript = 109,
        RejangScript = 110,
        RunicScript = 111,
        SamaritanScript = 112,
        SaurashtraScript = 113,
        SharadaScript = 114,
        ShavianScript = 115,
        SiddhamScript = 116,
        SignWritingScript = 117,
        SimplifiedHanScript = 118,
        SinhalaScript = 119,
        SoraSompengScript = 120,
        SundaneseScript = 121,
        SylotiNagriScript = 122,
        SyriacScript = 123,
        TagalogScript = 124,
        TagbanwaScript = 125,
        TaiLeScript = 126,
        TaiVietScript = 127,
        TakriScript = 128,
        TamilScript = 129,
        TangutScript = 130,
        TeluguScript = 131,
        ThaanaScript = 132,
        ThaiScript = 133,
        TibetanScript = 134,
        TifinaghScript = 135,
        TirhutaScript = 136,
        TraditionalHanScript = 137,
        UgariticScript = 138,
        VaiScript = 139,
        VarangKshitiScript = 140,
        YiScript = 141,

        BengaliScript = BanglaScript,
        MendeKikakuiScript = MendeScript,
        OriyaScript = OdiaScript,
        SimplifiedChineseScript = SimplifiedHanScript,
        TraditionalChineseScript = TraditionalHanScript,

        LastScript = YiScript
    };

    // ### Qt 7: Rename to Territory
    enum Country : ushort {
        AnyTerritory = 0,
        Afghanistan = 1,
        AlandIslands = 2,
        Albania = 3,
        Algeria = 4,
        AmericanSamoa = 5,
        Andorra = 6,
        Angola = 7,
        Anguilla = 8,
        Antarctica = 9,
        AntiguaAndBarbuda = 10,
        Argentina = 11,
        Armenia = 12,
        Aruba = 13,
        AscensionIsland = 14,
        Australia = 15,
        Austria = 16,
        Azerbaijan = 17,
        Bahamas = 18,
        Bahrain = 19,
        Bangladesh = 20,
        Barbados = 21,
        Belarus = 22,
        Belgium = 23,
        Belize = 24,
        Benin = 25,
        Bermuda = 26,
        Bhutan = 27,
        Bolivia = 28,
        BosniaAndHerzegovina = 29,
        Botswana = 30,
        BouvetIsland = 31,
        Brazil = 32,
        BritishIndianOceanTerritory = 33,
        BritishVirginIslands = 34,
        Brunei = 35,
        Bulgaria = 36,
        BurkinaFaso = 37,
        Burundi = 38,
        Cambodia = 39,
        Cameroon = 40,
        Canada = 41,
        CanaryIslands = 42,
        CapeVerde = 43,
        CaribbeanNetherlands = 44,
        CaymanIslands = 45,
        CentralAfricanRepublic = 46,
        CeutaAndMelilla = 47,
        Chad = 48,
        Chile = 49,
        China = 50,
        ChristmasIsland = 51,
        ClippertonIsland = 52,
        CocosIslands = 53,
        Colombia = 54,
        Comoros = 55,
        CongoBrazzaville = 56,
        CongoKinshasa = 57,
        CookIslands = 58,
        CostaRica = 59,
        Croatia = 60,
        Cuba = 61,
        Curacao = 62,
        Cyprus = 63,
        Czechia = 64,
        Denmark = 65,
        DiegoGarcia = 66,
        Djibouti = 67,
        Dominica = 68,
        DominicanRepublic = 69,
        Ecuador = 70,
        Egypt = 71,
        ElSalvador = 72,
        EquatorialGuinea = 73,
        Eritrea = 74,
        Estonia = 75,
        Eswatini = 76,
        Ethiopia = 77,
        Europe = 78,
        EuropeanUnion = 79,
        FalklandIslands = 80,
        FaroeIslands = 81,
        Fiji = 82,
        Finland = 83,
        France = 84,
        FrenchGuiana = 85,
        FrenchPolynesia = 86,
        FrenchSouthernTerritories = 87,
        Gabon = 88,
        Gambia = 89,
        Georgia = 90,
        Germany = 91,
        Ghana = 92,
        Gibraltar = 93,
        Greece = 94,
        Greenland = 95,
        Grenada = 96,
        Guadeloupe = 97,
        Guam = 98,
        Guatemala = 99,
        Guernsey = 100,
        GuineaBissau = 101,
        Guinea = 102,
        Guyana = 103,
        Haiti = 104,
        HeardAndMcDonaldIslands = 105,
        Honduras = 106,
        HongKong = 107,
        Hungary = 108,
        Iceland = 109,
        India = 110,
        Indonesia = 111,
        Iran = 112,
        Iraq = 113,
        Ireland = 114,
        IsleOfMan = 115,
        Israel = 116,
        Italy = 117,
        IvoryCoast = 118,
        Jamaica = 119,
        Japan = 120,
        Jersey = 121,
        Jordan = 122,
        Kazakhstan = 123,
        Kenya = 124,
        Kiribati = 125,
        Kosovo = 126,
        Kuwait = 127,
        Kyrgyzstan = 128,
        Laos = 129,
        LatinAmerica = 130,
        Latvia = 131,
        Lebanon = 132,
        Lesotho = 133,
        Liberia = 134,
        Libya = 135,
        Liechtenstein = 136,
        Lithuania = 137,
        Luxembourg = 138,
        Macao = 139,
        Macedonia = 140,
        Madagascar = 141,
        Malawi = 142,
        Malaysia = 143,
        Maldives = 144,
        Mali = 145,
        Malta = 146,
        MarshallIslands = 147,
        Martinique = 148,
        Mauritania = 149,
        Mauritius = 150,
        Mayotte = 151,
        Mexico = 152,
        Micronesia = 153,
        Moldova = 154,
        Monaco = 155,
        Mongolia = 156,
        Montenegro = 157,
        Montserrat = 158,
        Morocco = 159,
        Mozambique = 160,
        Myanmar = 161,
        Namibia = 162,
        NauruTerritory = 163,
        Nepal = 164,
        Netherlands = 165,
        NewCaledonia = 166,
        NewZealand = 167,
        Nicaragua = 168,
        Nigeria = 169,
        Niger = 170,
        Niue = 171,
        NorfolkIsland = 172,
        NorthernMarianaIslands = 173,
        NorthKorea = 174,
        Norway = 175,
        Oman = 176,
        OutlyingOceania = 177,
        Pakistan = 178,
        Palau = 179,
        PalestinianTerritories = 180,
        Panama = 181,
        PapuaNewGuinea = 182,
        Paraguay = 183,
        Peru = 184,
        Philippines = 185,
        Pitcairn = 186,
        Poland = 187,
        Portugal = 188,
        PuertoRico = 189,
        Qatar = 190,
        Reunion = 191,
        Romania = 192,
        Russia = 193,
        Rwanda = 194,
        SaintBarthelemy = 195,
        SaintHelena = 196,
        SaintKittsAndNevis = 197,
        SaintLucia = 198,
        SaintMartin = 199,
        SaintPierreAndMiquelon = 200,
        SaintVincentAndGrenadines = 201,
        Samoa = 202,
        SanMarino = 203,
        SaoTomeAndPrincipe = 204,
        SaudiArabia = 205,
        Senegal = 206,
        Serbia = 207,
        Seychelles = 208,
        SierraLeone = 209,
        Singapore = 210,
        SintMaarten = 211,
        Slovakia = 212,
        Slovenia = 213,
        SolomonIslands = 214,
        Somalia = 215,
        SouthAfrica = 216,
        SouthGeorgiaAndSouthSandwichIslands = 217,
        SouthKorea = 218,
        SouthSudan = 219,
        Spain = 220,
        SriLanka = 221,
        Sudan = 222,
        Suriname = 223,
        SvalbardAndJanMayen = 224,
        Sweden = 225,
        Switzerland = 226,
        Syria = 227,
        Taiwan = 228,
        Tajikistan = 229,
        Tanzania = 230,
        Thailand = 231,
        TimorLeste = 232,
        Togo = 233,
        TokelauTerritory = 234,
        Tonga = 235,
        TrinidadAndTobago = 236,
        TristanDaCunha = 237,
        Tunisia = 238,
        Turkey = 239,
        Turkmenistan = 240,
        TurksAndCaicosIslands = 241,
        TuvaluTerritory = 242,
        Uganda = 243,
        Ukraine = 244,
        UnitedArabEmirates = 245,
        UnitedKingdom = 246,
        UnitedStatesOutlyingIslands = 247,
        UnitedStates = 248,
        UnitedStatesVirginIslands = 249,
        Uruguay = 250,
        Uzbekistan = 251,
        Vanuatu = 252,
        VaticanCity = 253,
        Venezuela = 254,
        Vietnam = 255,
        WallisAndFutuna = 256,
        WesternSahara = 257,
        World = 258,
        Yemen = 259,
        Zambia = 260,
        Zimbabwe = 261,

        AnyCountry = AnyTerritory,
        Bonaire = CaribbeanNetherlands,
        BosniaAndHerzegowina = BosniaAndHerzegovina,
        CuraSao = Curacao,
        CzechRepublic = Czechia,
        DemocraticRepublicOfCongo = CongoKinshasa,
        DemocraticRepublicOfKorea = NorthKorea,
        EastTimor = TimorLeste,
        LatinAmericaAndTheCaribbean = LatinAmerica,
        Macau = Macao,
        NauruCountry = NauruTerritory,
        PeoplesRepublicOfCongo = CongoBrazzaville,
        RepublicOfKorea = SouthKorea,
        RussianFederation = Russia,
        SaintVincentAndTheGrenadines = SaintVincentAndGrenadines,
        SouthGeorgiaAndTheSouthSandwichIslands = SouthGeorgiaAndSouthSandwichIslands,
        SvalbardAndJanMayenIslands = SvalbardAndJanMayen,
        Swaziland = Eswatini,
        SyrianArabRepublic = Syria,
        TokelauCountry = TokelauTerritory,
        TuvaluCountry = TuvaluTerritory,
        UnitedStatesMinorOutlyingIslands = UnitedStatesOutlyingIslands,
        VaticanCityState = VaticanCity,
        WallisAndFutunaIslands = WallisAndFutuna,

        LastTerritory = Zimbabwe,
        LastCountry = LastTerritory
    };
// GENERATED PART ENDS HERE

    using Territory = Country; // ### Qt 7: reverse

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
    QT_CORE_INLINE_SINCE(6, 4)
    explicit QLocale(const QString &name);
    explicit QLocale(QStringView name);
    QLocale(Language language, Territory territory);
    QLocale(Language language, Script script = AnyScript, Territory territory = AnyTerritory);
    QLocale(const QLocale &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QLocale)
    QLocale &operator=(const QLocale &other);
    ~QLocale();

    void swap(QLocale &other) noexcept { d.swap(other.d); }

    Language language() const;
    Script script() const;
    Territory territory() const;
#if QT_DEPRECATED_SINCE(6, 6)
    QT_DEPRECATED_VERSION_X_6_6("Use territory() instead")
    Country country() const;
#endif
    QString name() const;

    QString bcp47Name() const;
    QString nativeLanguageName() const;
    QString nativeTerritoryName() const;
#if QT_DEPRECATED_SINCE(6, 6)
    QT_DEPRECATED_VERSION_X_6_6("Use nativeTerritoryName() instead")
    QString nativeCountryName() const;
#endif

    short toShort(const QString &s, bool *ok = nullptr) const
    { return toShort(qToStringViewIgnoringNull(s), ok); }
    ushort toUShort(const QString &s, bool *ok = nullptr) const
    { return toUShort(qToStringViewIgnoringNull(s), ok); }
    int toInt(const QString &s, bool *ok = nullptr) const
    { return toInt(qToStringViewIgnoringNull(s), ok); }
    uint toUInt(const QString &s, bool *ok = nullptr) const
    { return toUInt(qToStringViewIgnoringNull(s), ok); }
    long toLong(const QString &s, bool *ok = nullptr) const
    { return toLong(qToStringViewIgnoringNull(s), ok); }
    ulong toULong(const QString &s, bool *ok = nullptr) const
    { return toULong(qToStringViewIgnoringNull(s), ok); }
    qlonglong toLongLong(const QString &s, bool *ok = nullptr) const
    { return toLongLong(qToStringViewIgnoringNull(s), ok); }
    qulonglong toULongLong(const QString &s, bool *ok = nullptr) const
    { return toULongLong(qToStringViewIgnoringNull(s), ok); }
    float toFloat(const QString &s, bool *ok = nullptr) const
    { return toFloat(qToStringViewIgnoringNull(s), ok); }
    double toDouble(const QString &s, bool *ok = nullptr) const
    { return toDouble(qToStringViewIgnoringNull(s), ok); }

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
    QString toString(long i) const { return toString(qlonglong(i)); }
    QString toString(ulong i) const { return toString(qulonglong(i)); }
    QString toString(short i) const { return toString(qlonglong(i)); }
    QString toString(ushort i) const { return toString(qulonglong(i)); }
    QString toString(int i) const { return toString(qlonglong(i)); }
    QString toString(uint i) const { return toString(qulonglong(i)); }
    QString toString(double f, char format = 'g', int precision = 6) const;
    QString toString(float f, char format = 'g', int precision = 6) const
    { return toString(double(f), format, precision); }

    // (Can't inline first two: passing by value doesn't work when only forward-declared.)
    QString toString(QDate date, const QString &format) const;
    QString toString(QTime time, const QString &format) const;
    QString toString(const QDateTime &dateTime, const QString &format) const
    { return toString(dateTime, qToStringViewIgnoringNull(format)); }
    QString toString(QDate date, QStringView format) const;
    QString toString(QTime time, QStringView format) const;
    QString toString(const QDateTime &dateTime, QStringView format) const;
    QString toString(QDate date, FormatType format = LongFormat) const;
    QString toString(QTime time, FormatType format = LongFormat) const;
    QString toString(const QDateTime &dateTime, FormatType format = LongFormat) const;
    /* We can't pass a default for QCalendar (its declaration mentions
     * QLocale::FormatType, so it has to #include this header, which thus can't
     * #include its, so we can't instantiate QCalendar() as default). This
     * precludes any default for format, too.
     */
    QString toString(QDate date, QStringView format, QCalendar cal) const;
    QString toString(QDate date, FormatType format, QCalendar cal) const;
    QString toString(const QDateTime &dateTime, FormatType format, QCalendar cal) const;
    QString toString(const QDateTime &dateTime, QStringView format, QCalendar cal) const;

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
#endif

    QString decimalPoint() const;
    QString groupSeparator() const;
    QString percent() const;
    QString zeroDigit() const;
    QString negativeSign() const;
    QString positiveSign() const;
    QString exponential() const;

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
    QString toCurrencyString(short i, const QString &symbol = QString()) const
    { return toCurrencyString(qlonglong(i), symbol); }
    QString toCurrencyString(ushort i, const QString &symbol = QString()) const
    { return toCurrencyString(qulonglong(i), symbol); }
    QString toCurrencyString(int i, const QString &symbol = QString()) const
    { return toCurrencyString(qlonglong(i), symbol); }
    QString toCurrencyString(uint i, const QString &symbol = QString()) const
    { return toCurrencyString(qulonglong(i), symbol); }
    QString toCurrencyString(double, const QString &symbol = QString(), int precision = -1) const;
    QString toCurrencyString(float i, const QString &symbol = QString(), int precision = -1) const
    { return toCurrencyString(double(i), symbol, precision); }

    QString formattedDataSize(qint64 bytes, int precision = 2, DataSizeFormats format = DataSizeIecFormat) const;

    QStringList uiLanguages() const;

    enum LanguageCodeType {
        ISO639Part1 = 1 << 0,
        ISO639Part2B = 1 << 1,
        ISO639Part2T = 1 << 2,
        ISO639Part3 = 1 << 3,
        LegacyLanguageCode = 1 << 15,

        ISO639Part2 = ISO639Part2B | ISO639Part2T,
        ISO639Alpha2 = ISO639Part1,
        ISO639Alpha3 = ISO639Part2 | ISO639Part3,
        ISO639 = ISO639Alpha2 | ISO639Alpha3,

        AnyLanguageCode = -1
    };
    Q_DECLARE_FLAGS(LanguageCodeTypes, LanguageCodeType)

#if QT_CORE_REMOVED_SINCE(6, 3)
    static QString languageToCode(Language language);
    static Language codeToLanguage(QStringView languageCode) noexcept;
#endif
    static QString languageToCode(Language language, LanguageCodeTypes codeTypes = AnyLanguageCode);
    static Language codeToLanguage(QStringView languageCode,
                                   LanguageCodeTypes codeTypes = AnyLanguageCode) noexcept;
    static QString territoryToCode(Territory territory);
    static Territory codeToTerritory(QStringView territoryCode) noexcept;
#if QT_DEPRECATED_SINCE(6, 6)
    QT_DEPRECATED_VERSION_X_6_6("Use territoryToCode(Territory) instead")
    static QString countryToCode(Country country);
    QT_DEPRECATED_VERSION_X_6_6("Use codeToTerritory(QStringView) instead")
    static Country codeToCountry(QStringView countryCode) noexcept;
#endif
    static QString scriptToCode(Script script);
    static Script codeToScript(QStringView scriptCode) noexcept;

    static QString languageToString(Language language);
    static QString territoryToString(Territory territory);
#if QT_DEPRECATED_SINCE(6, 6)
    QT_DEPRECATED_VERSION_X_6_6("Use territoryToString(Territory) instead")
    static QString countryToString(Country country);
#endif
    static QString scriptToString(Script script);
    static void setDefault(const QLocale &locale);

    static QLocale c() { return QLocale(C); }
    static QLocale system();

    static QList<QLocale> matchingLocales(QLocale::Language language, QLocale::Script script,
                                          QLocale::Territory territory);
#if QT_DEPRECATED_SINCE(6, 6)
    QT_DEPRECATED_VERSION_X_6_6("Query territory() on each entry from matchingLocales() instead")
    static QList<Country> countriesForLanguage(Language lang);
#endif

    void setNumberOptions(NumberOptions options);
    NumberOptions numberOptions() const;

    enum QuotationStyle { StandardQuotation, AlternateQuotation };
    QString quoteString(const QString &str, QuotationStyle style = StandardQuotation) const
    { return quoteString(QStringView(str), style); }
    QString quoteString(QStringView str, QuotationStyle style = StandardQuotation) const;

    QString createSeparatedList(const QStringList &strl) const;

private:
    QLocale(QLocalePrivate &dd);
    bool equals(const QLocale &other) const;
    friend class QLocalePrivate;
    friend class QSystemLocale;
    friend class QCalendarBackend;
    friend class QGregorianCalendar;
    friend Q_CORE_EXPORT size_t qHash(const QLocale &key, size_t seed) noexcept;

    friend bool operator==(const QLocale &lhs, const QLocale &rhs) { return lhs.equals(rhs); }
    friend bool operator!=(const QLocale &lhs, const QLocale &rhs) { return !lhs.equals(rhs); }

    QSharedDataPointer<QLocalePrivate> d;
};
Q_DECLARE_SHARED(QLocale)
Q_DECLARE_OPERATORS_FOR_FLAGS(QLocale::NumberOptions)
Q_DECLARE_OPERATORS_FOR_FLAGS(QLocale::LanguageCodeTypes)

#if QT_CORE_INLINE_IMPL_SINCE(6, 4)
QLocale::QLocale(const QString &name)
    : QLocale(qToStringViewIgnoringNull(name)) {}
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QLocale &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QLocale &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QLocale &);
#endif

QT_END_NAMESPACE

#endif // QLOCALE_H
