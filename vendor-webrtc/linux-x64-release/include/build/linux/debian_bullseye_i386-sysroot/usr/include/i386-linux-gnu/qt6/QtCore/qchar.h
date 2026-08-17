// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCHAR_H
#define QCHAR_H

#include <QtCore/qglobal.h>

#include <functional> // for std::hash

QT_BEGIN_NAMESPACE


class QString;

struct QLatin1Char
{
public:
    constexpr inline explicit QLatin1Char(char c) noexcept : ch(c) {}
    constexpr inline char toLatin1() const noexcept { return ch; }
    constexpr inline char16_t unicode() const noexcept { return char16_t(uchar(ch)); }

    friend constexpr inline bool operator==(QLatin1Char lhs, QLatin1Char rhs) noexcept { return lhs.ch == rhs.ch; }
    friend constexpr inline bool operator!=(QLatin1Char lhs, QLatin1Char rhs) noexcept { return lhs.ch != rhs.ch; }
    friend constexpr inline bool operator<=(QLatin1Char lhs, QLatin1Char rhs) noexcept { return lhs.ch <= rhs.ch; }
    friend constexpr inline bool operator>=(QLatin1Char lhs, QLatin1Char rhs) noexcept { return lhs.ch >= rhs.ch; }
    friend constexpr inline bool operator< (QLatin1Char lhs, QLatin1Char rhs) noexcept { return lhs.ch <  rhs.ch; }
    friend constexpr inline bool operator> (QLatin1Char lhs, QLatin1Char rhs) noexcept { return lhs.ch >  rhs.ch; }

    friend constexpr inline bool operator==(char lhs, QLatin1Char rhs) noexcept { return lhs == rhs.toLatin1(); }
    friend constexpr inline bool operator!=(char lhs, QLatin1Char rhs) noexcept { return lhs != rhs.toLatin1(); }
    friend constexpr inline bool operator<=(char lhs, QLatin1Char rhs) noexcept { return lhs <= rhs.toLatin1(); }
    friend constexpr inline bool operator>=(char lhs, QLatin1Char rhs) noexcept { return lhs >= rhs.toLatin1(); }
    friend constexpr inline bool operator< (char lhs, QLatin1Char rhs) noexcept { return lhs <  rhs.toLatin1(); }
    friend constexpr inline bool operator> (char lhs, QLatin1Char rhs) noexcept { return lhs >  rhs.toLatin1(); }

    friend constexpr inline bool operator==(QLatin1Char lhs, char rhs) noexcept { return lhs.toLatin1() == rhs; }
    friend constexpr inline bool operator!=(QLatin1Char lhs, char rhs) noexcept { return lhs.toLatin1() != rhs; }
    friend constexpr inline bool operator<=(QLatin1Char lhs, char rhs) noexcept { return lhs.toLatin1() <= rhs; }
    friend constexpr inline bool operator>=(QLatin1Char lhs, char rhs) noexcept { return lhs.toLatin1() >= rhs; }
    friend constexpr inline bool operator< (QLatin1Char lhs, char rhs) noexcept { return lhs.toLatin1() <  rhs; }
    friend constexpr inline bool operator> (QLatin1Char lhs, char rhs) noexcept { return lhs.toLatin1() >  rhs; }

private:
    char ch;
};

class Q_CORE_EXPORT QChar {
public:
    enum SpecialCharacter {
        Null = 0x0000,
        Tabulation = 0x0009,
        LineFeed = 0x000a,
        FormFeed = 0x000c,
        CarriageReturn = 0x000d,
        Space = 0x0020,
        Nbsp = 0x00a0,
        SoftHyphen = 0x00ad,
        ReplacementCharacter = 0xfffd,
        ObjectReplacementCharacter = 0xfffc,
        ByteOrderMark = 0xfeff,
        ByteOrderSwapped = 0xfffe,
        ParagraphSeparator = 0x2029,
        LineSeparator = 0x2028,
        VisualTabCharacter = 0x2192,
        LastValidCodePoint = 0x10ffff
    };

#ifdef QT_IMPLICIT_QCHAR_CONSTRUCTION
#define QCHAR_MAYBE_IMPLICIT Q_IMPLICIT
#else
#define QCHAR_MAYBE_IMPLICIT explicit
#endif

    constexpr Q_IMPLICIT QChar() noexcept : ucs(0) {}
    constexpr Q_IMPLICIT QChar(ushort rc) noexcept : ucs(rc) {}
    constexpr QCHAR_MAYBE_IMPLICIT QChar(uchar c, uchar r) noexcept : ucs(char16_t((r << 8) | c)) {}
    constexpr Q_IMPLICIT QChar(short rc) noexcept : ucs(char16_t(rc)) {}
    constexpr QCHAR_MAYBE_IMPLICIT QChar(uint rc) noexcept : ucs((Q_ASSERT(rc <= 0xffff), char16_t(rc))) {}
    constexpr QCHAR_MAYBE_IMPLICIT QChar(int rc) noexcept : QChar(uint(rc)) {}
    constexpr Q_IMPLICIT QChar(SpecialCharacter s) noexcept : ucs(char16_t(s)) {}
    constexpr Q_IMPLICIT QChar(QLatin1Char ch) noexcept : ucs(ch.unicode()) {}
    constexpr Q_IMPLICIT QChar(char16_t ch) noexcept : ucs(ch) {}
#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
    constexpr Q_IMPLICIT QChar(wchar_t ch) noexcept : ucs(char16_t(ch)) {}
#endif

#ifndef QT_NO_CAST_FROM_ASCII
    // Always implicit -- allow for 'x' => QChar conversions
    QT_ASCII_CAST_WARN constexpr Q_IMPLICIT QChar(char c) noexcept : ucs(uchar(c)) { }
#ifndef QT_RESTRICTED_CAST_FROM_ASCII
    QT_ASCII_CAST_WARN constexpr QCHAR_MAYBE_IMPLICIT QChar(uchar c) noexcept : ucs(c) { }
#endif
#endif

#undef QCHAR_MAYBE_IMPLICIT

    static constexpr QChar fromUcs2(char16_t c) noexcept { return QChar{c}; }
    static constexpr inline auto fromUcs4(char32_t c) noexcept;

    // Unicode information

    enum Category
    {
        Mark_NonSpacing,          //   Mn
        Mark_SpacingCombining,    //   Mc
        Mark_Enclosing,           //   Me

        Number_DecimalDigit,      //   Nd
        Number_Letter,            //   Nl
        Number_Other,             //   No

        Separator_Space,          //   Zs
        Separator_Line,           //   Zl
        Separator_Paragraph,      //   Zp

        Other_Control,            //   Cc
        Other_Format,             //   Cf
        Other_Surrogate,          //   Cs
        Other_PrivateUse,         //   Co
        Other_NotAssigned,        //   Cn

        Letter_Uppercase,         //   Lu
        Letter_Lowercase,         //   Ll
        Letter_Titlecase,         //   Lt
        Letter_Modifier,          //   Lm
        Letter_Other,             //   Lo

        Punctuation_Connector,    //   Pc
        Punctuation_Dash,         //   Pd
        Punctuation_Open,         //   Ps
        Punctuation_Close,        //   Pe
        Punctuation_InitialQuote, //   Pi
        Punctuation_FinalQuote,   //   Pf
        Punctuation_Other,        //   Po

        Symbol_Math,              //   Sm
        Symbol_Currency,          //   Sc
        Symbol_Modifier,          //   Sk
        Symbol_Other              //   So
    };

    enum Script
    {
        Script_Unknown,
        Script_Inherited,
        Script_Common,

        Script_Latin,
        Script_Greek,
        Script_Cyrillic,
        Script_Armenian,
        Script_Hebrew,
        Script_Arabic,
        Script_Syriac,
        Script_Thaana,
        Script_Devanagari,
        Script_Bengali,
        Script_Gurmukhi,
        Script_Gujarati,
        Script_Oriya,
        Script_Tamil,
        Script_Telugu,
        Script_Kannada,
        Script_Malayalam,
        Script_Sinhala,
        Script_Thai,
        Script_Lao,
        Script_Tibetan,
        Script_Myanmar,
        Script_Georgian,
        Script_Hangul,
        Script_Ethiopic,
        Script_Cherokee,
        Script_CanadianAboriginal,
        Script_Ogham,
        Script_Runic,
        Script_Khmer,
        Script_Mongolian,
        Script_Hiragana,
        Script_Katakana,
        Script_Bopomofo,
        Script_Han,
        Script_Yi,
        Script_OldItalic,
        Script_Gothic,
        Script_Deseret,
        Script_Tagalog,
        Script_Hanunoo,
        Script_Buhid,
        Script_Tagbanwa,
        Script_Coptic,

        // Unicode 4.0 additions
        Script_Limbu,
        Script_TaiLe,
        Script_LinearB,
        Script_Ugaritic,
        Script_Shavian,
        Script_Osmanya,
        Script_Cypriot,
        Script_Braille,

        // Unicode 4.1 additions
        Script_Buginese,
        Script_NewTaiLue,
        Script_Glagolitic,
        Script_Tifinagh,
        Script_SylotiNagri,
        Script_OldPersian,
        Script_Kharoshthi,

        // Unicode 5.0 additions
        Script_Balinese,
        Script_Cuneiform,
        Script_Phoenician,
        Script_PhagsPa,
        Script_Nko,

        // Unicode 5.1 additions
        Script_Sundanese,
        Script_Lepcha,
        Script_OlChiki,
        Script_Vai,
        Script_Saurashtra,
        Script_KayahLi,
        Script_Rejang,
        Script_Lycian,
        Script_Carian,
        Script_Lydian,
        Script_Cham,

        // Unicode 5.2 additions
        Script_TaiTham,
        Script_TaiViet,
        Script_Avestan,
        Script_EgyptianHieroglyphs,
        Script_Samaritan,
        Script_Lisu,
        Script_Bamum,
        Script_Javanese,
        Script_MeeteiMayek,
        Script_ImperialAramaic,
        Script_OldSouthArabian,
        Script_InscriptionalParthian,
        Script_InscriptionalPahlavi,
        Script_OldTurkic,
        Script_Kaithi,

        // Unicode 6.0 additions
        Script_Batak,
        Script_Brahmi,
        Script_Mandaic,

        // Unicode 6.1 additions
        Script_Chakma,
        Script_MeroiticCursive,
        Script_MeroiticHieroglyphs,
        Script_Miao,
        Script_Sharada,
        Script_SoraSompeng,
        Script_Takri,

        // Unicode 7.0 additions
        Script_CaucasianAlbanian,
        Script_BassaVah,
        Script_Duployan,
        Script_Elbasan,
        Script_Grantha,
        Script_PahawhHmong,
        Script_Khojki,
        Script_LinearA,
        Script_Mahajani,
        Script_Manichaean,
        Script_MendeKikakui,
        Script_Modi,
        Script_Mro,
        Script_OldNorthArabian,
        Script_Nabataean,
        Script_Palmyrene,
        Script_PauCinHau,
        Script_OldPermic,
        Script_PsalterPahlavi,
        Script_Siddham,
        Script_Khudawadi,
        Script_Tirhuta,
        Script_WarangCiti,

        // Unicode 8.0 additions
        Script_Ahom,
        Script_AnatolianHieroglyphs,
        Script_Hatran,
        Script_Multani,
        Script_OldHungarian,
        Script_SignWriting,

        // Unicode 9.0 additions
        Script_Adlam,
        Script_Bhaiksuki,
        Script_Marchen,
        Script_Newa,
        Script_Osage,
        Script_Tangut,

        // Unicode 10.0 additions
        Script_MasaramGondi,
        Script_Nushu,
        Script_Soyombo,
        Script_ZanabazarSquare,

        // Unicode 12.1 additions
        Script_Dogra,
        Script_GunjalaGondi,
        Script_HanifiRohingya,
        Script_Makasar,
        Script_Medefaidrin,
        Script_OldSogdian,
        Script_Sogdian,
        Script_Elymaic,
        Script_Nandinagari,
        Script_NyiakengPuachueHmong,
        Script_Wancho,

        // Unicode 13.0 additions
        Script_Chorasmian,
        Script_DivesAkuru,
        Script_KhitanSmallScript,
        Script_Yezidi,

        // Unicode 14.0 additions
        Script_CyproMinoan,
        Script_OldUyghur,
        Script_Tangsa,
        Script_Toto,
        Script_Vithkuqi,

        ScriptCount
    };

    enum Direction
    {
        DirL, DirR, DirEN, DirES, DirET, DirAN, DirCS, DirB, DirS, DirWS, DirON,
        DirLRE, DirLRO, DirAL, DirRLE, DirRLO, DirPDF, DirNSM, DirBN,
        DirLRI, DirRLI, DirFSI, DirPDI
    };

    enum Decomposition
    {
        NoDecomposition,
        Canonical,
        Font,
        NoBreak,
        Initial,
        Medial,
        Final,
        Isolated,
        Circle,
        Super,
        Sub,
        Vertical,
        Wide,
        Narrow,
        Small,
        Square,
        Compat,
        Fraction
    };

    enum JoiningType {
        Joining_None,
        Joining_Causing,
        Joining_Dual,
        Joining_Right,
        Joining_Left,
        Joining_Transparent
    };

    enum CombiningClass
    {
        Combining_BelowLeftAttached       = 200,
        Combining_BelowAttached           = 202,
        Combining_BelowRightAttached      = 204,
        Combining_LeftAttached            = 208,
        Combining_RightAttached           = 210,
        Combining_AboveLeftAttached       = 212,
        Combining_AboveAttached           = 214,
        Combining_AboveRightAttached      = 216,

        Combining_BelowLeft               = 218,
        Combining_Below                   = 220,
        Combining_BelowRight              = 222,
        Combining_Left                    = 224,
        Combining_Right                   = 226,
        Combining_AboveLeft               = 228,
        Combining_Above                   = 230,
        Combining_AboveRight              = 232,

        Combining_DoubleBelow             = 233,
        Combining_DoubleAbove             = 234,
        Combining_IotaSubscript           = 240
    };

    enum UnicodeVersion {
        Unicode_Unassigned,
        Unicode_1_1,
        Unicode_2_0,
        Unicode_2_1_2,
        Unicode_3_0,
        Unicode_3_1,
        Unicode_3_2,
        Unicode_4_0,
        Unicode_4_1,
        Unicode_5_0,
        Unicode_5_1,
        Unicode_5_2,
        Unicode_6_0,
        Unicode_6_1,
        Unicode_6_2,
        Unicode_6_3,
        Unicode_7_0,
        Unicode_8_0,
        Unicode_9_0,
        Unicode_10_0,
        Unicode_11_0,
        Unicode_12_0,
        Unicode_12_1,
        Unicode_13_0,
        Unicode_14_0
    };

    inline Category category() const noexcept { return QChar::category(ucs); }
    inline Direction direction() const noexcept { return QChar::direction(ucs); }
    inline JoiningType joiningType() const noexcept { return QChar::joiningType(ucs); }
    inline unsigned char combiningClass() const noexcept { return QChar::combiningClass(ucs); }

    inline QChar mirroredChar() const noexcept { return QChar(QChar::mirroredChar(ucs)); }
    inline bool hasMirrored() const noexcept { return QChar::hasMirrored(ucs); }

    QString decomposition() const;
    inline Decomposition decompositionTag() const noexcept { return QChar::decompositionTag(ucs); }

    inline int digitValue() const noexcept { return QChar::digitValue(ucs); }
    inline QChar toLower() const noexcept { return QChar(QChar::toLower(ucs)); }
    inline QChar toUpper() const noexcept { return QChar(QChar::toUpper(ucs)); }
    inline QChar toTitleCase() const noexcept { return QChar(QChar::toTitleCase(ucs)); }
    inline QChar toCaseFolded() const noexcept { return QChar(QChar::toCaseFolded(ucs)); }

    inline Script script() const noexcept { return QChar::script(ucs); }

    inline UnicodeVersion unicodeVersion() const noexcept { return QChar::unicodeVersion(ucs); }

    constexpr inline char toLatin1() const noexcept { return ucs > 0xff ? '\0' : char(ucs); }
    constexpr inline char16_t unicode() const noexcept { return ucs; }
    constexpr inline char16_t &unicode() noexcept { return ucs; }

    static constexpr QChar fromLatin1(char c) noexcept { return QLatin1Char(c); }

    constexpr inline bool isNull() const noexcept { return ucs == 0; }

    inline bool isPrint() const noexcept { return QChar::isPrint(ucs); }
    constexpr inline bool isSpace() const noexcept { return QChar::isSpace(ucs); }
    inline bool isMark() const noexcept { return QChar::isMark(ucs); }
    inline bool isPunct() const noexcept { return QChar::isPunct(ucs); }
    inline bool isSymbol() const noexcept { return QChar::isSymbol(ucs); }
    constexpr inline bool isLetter() const noexcept { return QChar::isLetter(ucs); }
    constexpr inline bool isNumber() const noexcept { return QChar::isNumber(ucs); }
    constexpr inline bool isLetterOrNumber() const noexcept { return QChar::isLetterOrNumber(ucs); }
    constexpr inline bool isDigit() const noexcept { return QChar::isDigit(ucs); }
    constexpr inline bool isLower() const noexcept { return QChar::isLower(ucs); }
    constexpr inline bool isUpper() const noexcept { return QChar::isUpper(ucs); }
    constexpr inline bool isTitleCase() const noexcept { return QChar::isTitleCase(ucs); }

    constexpr inline bool isNonCharacter() const noexcept { return QChar::isNonCharacter(ucs); }
    constexpr inline bool isHighSurrogate() const noexcept { return QChar::isHighSurrogate(ucs); }
    constexpr inline bool isLowSurrogate() const noexcept { return QChar::isLowSurrogate(ucs); }
    constexpr inline bool isSurrogate() const noexcept { return QChar::isSurrogate(ucs); }

    constexpr inline uchar cell() const noexcept { return uchar(ucs & 0xff); }
    constexpr inline uchar row() const noexcept { return uchar((ucs>>8)&0xff); }
    constexpr inline void setCell(uchar acell) noexcept { ucs = char16_t((ucs & 0xff00) + acell); }
    constexpr inline void setRow(uchar arow) noexcept { ucs = char16_t((char16_t(arow)<<8) + (ucs&0xff)); }

    static constexpr inline bool isNonCharacter(char32_t ucs4) noexcept
    {
        return ucs4 >= 0xfdd0 && (ucs4 <= 0xfdef || (ucs4 & 0xfffe) == 0xfffe);
    }
    static constexpr inline bool isHighSurrogate(char32_t ucs4) noexcept
    {
        return (ucs4 & 0xfffffc00) == 0xd800; // 0xd800 + up to 1023 (0x3ff)
    }
    static constexpr inline bool isLowSurrogate(char32_t ucs4) noexcept
    {
        return (ucs4 & 0xfffffc00) == 0xdc00; // 0xdc00 + up to 1023 (0x3ff)
    }
    static constexpr inline bool isSurrogate(char32_t ucs4) noexcept
    {
        return (ucs4 - 0xd800u < 2048u);
    }
    static constexpr inline bool requiresSurrogates(char32_t ucs4) noexcept
    {
        return (ucs4 >= 0x10000);
    }
    static constexpr inline char32_t surrogateToUcs4(char16_t high, char16_t low) noexcept
    {
        // 0x010000 through 0x10ffff, provided params are actual high, low surrogates.
        // 0x010000 + ((high - 0xd800) << 10) + (low - 0xdc00), optimized:
        return (char32_t(high)<<10) + low - 0x35fdc00;
    }
    static constexpr inline char32_t surrogateToUcs4(QChar high, QChar low) noexcept
    {
        return surrogateToUcs4(high.ucs, low.ucs);
    }
    static constexpr inline char16_t highSurrogate(char32_t ucs4) noexcept
    {
        return char16_t((ucs4>>10) + 0xd7c0);
    }
    static constexpr inline char16_t lowSurrogate(char32_t ucs4) noexcept
    {
        return char16_t(ucs4%0x400 + 0xdc00);
    }

    static Category QT_FASTCALL category(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static Direction QT_FASTCALL direction(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static JoiningType QT_FASTCALL joiningType(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static unsigned char QT_FASTCALL combiningClass(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;

    static char32_t QT_FASTCALL mirroredChar(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static bool QT_FASTCALL hasMirrored(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;

    static QString QT_FASTCALL decomposition(char32_t ucs4);
    static Decomposition QT_FASTCALL decompositionTag(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;

    static int QT_FASTCALL digitValue(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static char32_t QT_FASTCALL toLower(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static char32_t QT_FASTCALL toUpper(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static char32_t QT_FASTCALL toTitleCase(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static char32_t QT_FASTCALL toCaseFolded(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;

    static Script QT_FASTCALL script(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;

    static UnicodeVersion QT_FASTCALL unicodeVersion(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;

    static UnicodeVersion QT_FASTCALL currentUnicodeVersion() noexcept Q_DECL_CONST_FUNCTION;

    static bool QT_FASTCALL isPrint(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static constexpr inline bool isSpace(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION
    {
        // note that [0x09..0x0d] + 0x85 are exceptional Cc-s and must be handled explicitly
        return ucs4 == 0x20 || (ucs4 <= 0x0d && ucs4 >= 0x09)
                || (ucs4 > 127 && (ucs4 == 0x85 || ucs4 == 0xa0 || QChar::isSpace_helper(ucs4)));
    }
    static bool QT_FASTCALL isMark(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static bool QT_FASTCALL isPunct(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static bool QT_FASTCALL isSymbol(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static constexpr inline bool isLetter(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION
    {
        return (ucs4 >= 'A' && ucs4 <= 'z' && (ucs4 >= 'a' || ucs4 <= 'Z'))
                || (ucs4 > 127 && QChar::isLetter_helper(ucs4));
    }
    static constexpr inline bool isNumber(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION
    { return (ucs4 <= '9' && ucs4 >= '0') || (ucs4 > 127 && QChar::isNumber_helper(ucs4)); }
    static constexpr inline bool isLetterOrNumber(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION
    {
        return (ucs4 >= 'A' && ucs4 <= 'z' && (ucs4 >= 'a' || ucs4 <= 'Z'))
                || (ucs4 >= '0' && ucs4 <= '9')
                || (ucs4 > 127 && QChar::isLetterOrNumber_helper(ucs4));
    }
    static constexpr inline bool isDigit(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION
    { return (ucs4 <= '9' && ucs4 >= '0') || (ucs4 > 127 && QChar::category(ucs4) == Number_DecimalDigit); }
    static constexpr inline bool isLower(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION
    { return (ucs4 <= 'z' && ucs4 >= 'a') || (ucs4 > 127 && QChar::category(ucs4) == Letter_Lowercase); }
    static constexpr inline bool isUpper(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION
    { return (ucs4 <= 'Z' && ucs4 >= 'A') || (ucs4 > 127 && QChar::category(ucs4) == Letter_Uppercase); }
    static constexpr inline bool isTitleCase(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION
    { return ucs4 > 127 && QChar::category(ucs4) == Letter_Titlecase; }

    friend constexpr inline bool operator==(QChar c1, QChar c2) noexcept { return c1.ucs == c2.ucs; }
    friend constexpr inline bool operator< (QChar c1, QChar c2) noexcept { return c1.ucs <  c2.ucs; }

    friend constexpr inline bool operator!=(QChar c1, QChar c2) noexcept { return !operator==(c1, c2); }
    friend constexpr inline bool operator>=(QChar c1, QChar c2) noexcept { return !operator< (c1, c2); }
    friend constexpr inline bool operator> (QChar c1, QChar c2) noexcept { return  operator< (c2, c1); }
    friend constexpr inline bool operator<=(QChar c1, QChar c2) noexcept { return !operator< (c2, c1); }

    friend constexpr inline bool operator==(QChar lhs, std::nullptr_t) noexcept { return lhs.isNull(); }
    friend constexpr inline bool operator< (QChar,     std::nullptr_t) noexcept { return false; }
    friend constexpr inline bool operator==(std::nullptr_t, QChar rhs) noexcept { return rhs.isNull(); }
    friend constexpr inline bool operator< (std::nullptr_t, QChar rhs) noexcept { return !rhs.isNull(); }

    friend constexpr inline bool operator!=(QChar lhs, std::nullptr_t) noexcept { return !operator==(lhs, nullptr); }
    friend constexpr inline bool operator>=(QChar lhs, std::nullptr_t) noexcept { return !operator< (lhs, nullptr); }
    friend constexpr inline bool operator> (QChar lhs, std::nullptr_t) noexcept { return  operator< (nullptr, lhs); }
    friend constexpr inline bool operator<=(QChar lhs, std::nullptr_t) noexcept { return !operator< (nullptr, lhs); }

    friend constexpr inline bool operator!=(std::nullptr_t, QChar rhs) noexcept { return !operator==(nullptr, rhs); }
    friend constexpr inline bool operator>=(std::nullptr_t, QChar rhs) noexcept { return !operator< (nullptr, rhs); }
    friend constexpr inline bool operator> (std::nullptr_t, QChar rhs) noexcept { return  operator< (rhs, nullptr); }
    friend constexpr inline bool operator<=(std::nullptr_t, QChar rhs) noexcept { return !operator< (rhs, nullptr); }

private:
    static bool QT_FASTCALL isSpace_helper(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static bool QT_FASTCALL isLetter_helper(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static bool QT_FASTCALL isNumber_helper(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;
    static bool QT_FASTCALL isLetterOrNumber_helper(char32_t ucs4) noexcept Q_DECL_CONST_FUNCTION;

#ifdef QT_NO_CAST_FROM_ASCII
    QChar(char c) = delete;
    QChar(uchar c) = delete;
#endif

    char16_t ucs;
};

Q_DECLARE_TYPEINFO(QChar, Q_PRIMITIVE_TYPE);

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, QChar);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QChar &);
#endif

namespace Qt {
inline namespace Literals {
inline namespace StringLiterals {

constexpr inline QLatin1Char operator"" _L1(char ch) noexcept
{
    return QLatin1Char(ch);
}

} // StringLiterals
} // Literals
} // Qt

QT_END_NAMESPACE

namespace std {
template <>
struct hash<QT_PREPEND_NAMESPACE(QChar)>
{
    template <typename = void> // for transparent constexpr tracking
    constexpr size_t operator()(QT_PREPEND_NAMESPACE(QChar) c) const
        noexcept(noexcept(std::hash<char16_t>{}(u' ')))
    {
        return std::hash<char16_t>{}(c.unicode());
    }
};
} // namespace std

#endif // QCHAR_H

#include <QtCore/qstringview.h> // for QChar::fromUcs4() definition
