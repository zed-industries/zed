// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFONT_H
#define QFONT_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qwindowdefs.h>
#include <QtCore/qstring.h>
#include <QtCore/qsharedpointer.h>


QT_BEGIN_NAMESPACE


class QFontPrivate;                                     /* don't touch */
class QVariant;

class Q_GUI_EXPORT QFont
{
    Q_GADGET
public:
    enum StyleHint {
        Helvetica,  SansSerif = Helvetica,
        Times,      Serif = Times,
        Courier,    TypeWriter = Courier,
        OldEnglish, Decorative = OldEnglish,
        System,
        AnyStyle,
        Cursive,
        Monospace,
        Fantasy
    };
    Q_ENUM(StyleHint)

    enum StyleStrategy {
        PreferDefault       = 0x0001,
        PreferBitmap        = 0x0002,
        PreferDevice        = 0x0004,
        PreferOutline       = 0x0008,
        ForceOutline        = 0x0010,
        PreferMatch         = 0x0020,
        PreferQuality       = 0x0040,
        PreferAntialias     = 0x0080,
        NoAntialias         = 0x0100,
        NoSubpixelAntialias = 0x0800,
        PreferNoShaping     = 0x1000,
        NoFontMerging       = 0x8000
    };
    Q_ENUM(StyleStrategy)

    enum HintingPreference {
        PreferDefaultHinting        = 0,
        PreferNoHinting             = 1,
        PreferVerticalHinting       = 2,
        PreferFullHinting           = 3
    };
    Q_ENUM(HintingPreference)

    enum Weight {
        Thin = 100,
        ExtraLight = 200,
        Light = 300,
        Normal = 400,
        Medium = 500,
        DemiBold = 600,
        Bold = 700,
        ExtraBold = 800,
        Black = 900
    };
    Q_ENUM(Weight)

    enum Style {
        StyleNormal,
        StyleItalic,
        StyleOblique
    };
    Q_ENUM(Style)

    enum Stretch {
        AnyStretch     =   0,
        UltraCondensed =  50,
        ExtraCondensed =  62,
        Condensed      =  75,
        SemiCondensed  =  87,
        Unstretched    = 100,
        SemiExpanded   = 112,
        Expanded       = 125,
        ExtraExpanded  = 150,
        UltraExpanded  = 200
    };
    Q_ENUM(Stretch)

    enum Capitalization {
        MixedCase,
        AllUppercase,
        AllLowercase,
        SmallCaps,
        Capitalize
    };
    Q_ENUM(Capitalization)

    enum SpacingType {
        PercentageSpacing,
        AbsoluteSpacing
    };
    Q_ENUM(SpacingType)

    enum ResolveProperties {
        NoPropertiesResolved        = 0x0000,
        FamilyResolved              = 0x0001,
        SizeResolved                = 0x0002,
        StyleHintResolved           = 0x0004,
        StyleStrategyResolved       = 0x0008,
        WeightResolved              = 0x0010,
        StyleResolved               = 0x0020,
        UnderlineResolved           = 0x0040,
        OverlineResolved            = 0x0080,
        StrikeOutResolved           = 0x0100,
        FixedPitchResolved          = 0x0200,
        StretchResolved             = 0x0400,
        KerningResolved             = 0x0800,
        CapitalizationResolved      = 0x1000,
        LetterSpacingResolved       = 0x2000,
        WordSpacingResolved         = 0x4000,
        HintingPreferenceResolved   = 0x8000,
        StyleNameResolved           = 0x10000,
        FamiliesResolved            = 0x20000,
        AllPropertiesResolved       = 0x3ffff
    };
    Q_ENUM(ResolveProperties)

    QFont();

    QFont(const QString &family, int pointSize = -1, int weight = -1, bool italic = false);
    explicit QFont(const QStringList &families, int pointSize = -1, int weight = -1, bool italic = false);
    QFont(const QFont &font, const QPaintDevice *pd);
    QFont(const QFont &font);
    ~QFont();

    void swap(QFont &other) noexcept
    { d.swap(other.d); std::swap(resolve_mask, other.resolve_mask); }

    QString family() const;
    void setFamily(const QString &);

    QStringList families() const;
    void setFamilies(const QStringList &);

    QString styleName() const;
    void setStyleName(const QString &);

    int pointSize() const;
    void setPointSize(int);
    qreal pointSizeF() const;
    void setPointSizeF(qreal);

    int pixelSize() const;
    void setPixelSize(int);

    Weight weight() const;
    void setWeight(Weight weight);

    inline bool bold() const;
    inline void setBold(bool);

    void setStyle(Style style);
    Style style() const;

    inline bool italic() const;
    inline void setItalic(bool b);

    bool underline() const;
    void setUnderline(bool);

    bool overline() const;
    void setOverline(bool);

    bool strikeOut() const;
    void setStrikeOut(bool);

    bool fixedPitch() const;
    void setFixedPitch(bool);

    bool kerning() const;
    void setKerning(bool);

    StyleHint styleHint() const;
    StyleStrategy styleStrategy() const;
    void setStyleHint(StyleHint, StyleStrategy = PreferDefault);
    void setStyleStrategy(StyleStrategy s);

    int stretch() const;
    void setStretch(int);

    qreal letterSpacing() const;
    SpacingType letterSpacingType() const;
    void setLetterSpacing(SpacingType type, qreal spacing);

    qreal wordSpacing() const;
    void setWordSpacing(qreal spacing);

    void setCapitalization(Capitalization);
    Capitalization capitalization() const;

    void setHintingPreference(HintingPreference hintingPreference);
    HintingPreference hintingPreference() const;

    // dupicated from QFontInfo
    bool exactMatch() const;

    QFont &operator=(const QFont &);
    bool operator==(const QFont &) const;
    bool operator!=(const QFont &) const;
    bool operator<(const QFont &) const;
    operator QVariant() const;
    bool isCopyOf(const QFont &) const;
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QFont)

    QString key() const;

    QString toString() const;
    bool fromString(const QString &);

    static QString substitute(const QString &);
    static QStringList substitutes(const QString &);
    static QStringList substitutions();
    static void insertSubstitution(const QString&, const QString &);
    static void insertSubstitutions(const QString&, const QStringList &);
    static void removeSubstitutions(const QString &);
    static void initialize();
    static void cleanup();
    static void cacheStatistics();

    QString defaultFamily() const;

    QFont resolve(const QFont &) const;
    inline uint resolveMask() const { return resolve_mask; }
    inline void setResolveMask(uint mask) { resolve_mask = mask; }

#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use setWeight() instead") void setLegacyWeight(int legacyWeight);
    QT_DEPRECATED_VERSION_X_6_0("Use weight() instead") int legacyWeight() const;
#endif

private:
    explicit QFont(QFontPrivate *);

    void detach();


    friend class QFontPrivate;
    friend class QFontDialogPrivate;
    friend class QFontMetrics;
    friend class QFontMetricsF;
    friend class QFontInfo;
    friend class QPainter;
    friend class QPainterPrivate;
    friend class QApplication;
    friend class QWidget;
    friend class QWidgetPrivate;
    friend class QTextLayout;
    friend class QTextEngine;
    friend class QStackTextEngine;
    friend class QTextLine;
    friend struct QScriptLine;
    friend class QOpenGLContext;
    friend class QWin32PaintEngine;
    friend class QAlphaPaintEngine;
    friend class QPainterPath;
    friend class QTextItemInt;
    friend class QPicturePaintEngine;
    friend class QPainterReplayer;
    friend class QPaintBufferEngine;
    friend class QCommandLinkButtonPrivate;
    friend class QFontEngine;

#ifndef QT_NO_DATASTREAM
    friend Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QFont &);
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QFont &);
#endif

#ifndef QT_NO_DEBUG_STREAM
    friend Q_GUI_EXPORT QDebug operator<<(QDebug, const QFont &);
#endif

    QExplicitlySharedDataPointer<QFontPrivate> d;
    uint resolve_mask;
};

Q_DECLARE_SHARED(QFont)

Q_GUI_EXPORT size_t qHash(const QFont &font, size_t seed = 0) noexcept;

inline bool QFont::bold() const
{ return weight() > Medium; }


inline void QFont::setBold(bool enable)
{ setWeight(enable ? Bold : Normal); }

inline bool QFont::italic() const
{
    return (style() != StyleNormal);
}

inline void QFont::setItalic(bool b) {
    setStyle(b ? StyleItalic : StyleNormal);
}


/*****************************************************************************
  QFont stream functions
 *****************************************************************************/

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QFont &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QFont &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QFont &);
#endif

QT_END_NAMESPACE

#endif // QFONT_H
