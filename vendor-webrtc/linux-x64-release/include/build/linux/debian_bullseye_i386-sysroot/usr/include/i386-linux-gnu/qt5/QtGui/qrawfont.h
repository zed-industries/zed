/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QRAWFONT_H
#define QRAWFONT_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qstring.h>
#include <QtCore/qiodevice.h>
#include <QtCore/qglobal.h>
#include <QtCore/qobject.h>
#include <QtCore/qpoint.h>
#include <QtGui/qfont.h>
#include <QtGui/qtransform.h>
#include <QtGui/qfontdatabase.h>

#if !defined(QT_NO_RAWFONT)

QT_BEGIN_NAMESPACE


class QRawFontPrivate;
class Q_GUI_EXPORT QRawFont
{
public:
    enum AntialiasingType {
        PixelAntialiasing,
        SubPixelAntialiasing
    };

    enum LayoutFlag {
        SeparateAdvances = 0,
        KernedAdvances = 1,
        UseDesignMetrics = 2
    };
    Q_DECLARE_FLAGS(LayoutFlags, LayoutFlag)

    QRawFont();
    QRawFont(const QString &fileName,
             qreal pixelSize,
             QFont::HintingPreference hintingPreference = QFont::PreferDefaultHinting);
    QRawFont(const QByteArray &fontData,
             qreal pixelSize,
             QFont::HintingPreference hintingPreference = QFont::PreferDefaultHinting);
    QRawFont(const QRawFont &other);
    QRawFont &operator=(QRawFont &&other) noexcept { swap(other); return *this; }
    QRawFont &operator=(const QRawFont &other);
    ~QRawFont();

    void swap(QRawFont &other) noexcept { qSwap(d, other.d); }

    bool isValid() const;

    bool operator==(const QRawFont &other) const;
    inline bool operator!=(const QRawFont &other) const
    { return !operator==(other); }

    QString familyName() const;
    QString styleName() const;

    QFont::Style style() const;
    int weight() const;

    QVector<quint32> glyphIndexesForString(const QString &text) const;
    inline QVector<QPointF> advancesForGlyphIndexes(const QVector<quint32> &glyphIndexes) const;
    inline QVector<QPointF> advancesForGlyphIndexes(const QVector<quint32> &glyphIndexes, LayoutFlags layoutFlags) const;
    bool glyphIndexesForChars(const QChar *chars, int numChars, quint32 *glyphIndexes, int *numGlyphs) const;
    bool advancesForGlyphIndexes(const quint32 *glyphIndexes, QPointF *advances, int numGlyphs) const;
    bool advancesForGlyphIndexes(const quint32 *glyphIndexes, QPointF *advances, int numGlyphs, LayoutFlags layoutFlags) const;

    QImage alphaMapForGlyph(quint32 glyphIndex,
                            AntialiasingType antialiasingType = SubPixelAntialiasing,
                            const QTransform &transform = QTransform()) const;
    QPainterPath pathForGlyph(quint32 glyphIndex) const;
    QRectF boundingRect(quint32 glyphIndex) const;

    void setPixelSize(qreal pixelSize);
    qreal pixelSize() const;

    QFont::HintingPreference hintingPreference() const;

    qreal ascent() const;
    qreal capHeight() const;
    qreal descent() const;
    qreal leading() const;
    qreal xHeight() const;
    qreal averageCharWidth() const;
    qreal maxCharWidth() const;
    qreal lineThickness() const;
    qreal underlinePosition() const;

    qreal unitsPerEm() const;

    void loadFromFile(const QString &fileName,
                      qreal pixelSize,
                      QFont::HintingPreference hintingPreference);

    void loadFromData(const QByteArray &fontData,
                      qreal pixelSize,
                      QFont::HintingPreference hintingPreference);

    bool supportsCharacter(uint ucs4) const;
    bool supportsCharacter(QChar character) const;
    QList<QFontDatabase::WritingSystem> supportedWritingSystems() const;

    QByteArray fontTable(const char *tagName) const;

    static QRawFont fromFont(const QFont &font,
                             QFontDatabase::WritingSystem writingSystem = QFontDatabase::Any);

private:
    friend class QRawFontPrivate;
    friend class QTextLayout;
    friend class QTextEngine;

    QExplicitlySharedDataPointer<QRawFontPrivate> d;
};

Q_DECLARE_SHARED(QRawFont)

Q_DECLARE_OPERATORS_FOR_FLAGS(QRawFont::LayoutFlags)

Q_GUI_EXPORT uint qHash(const QRawFont &font, uint seed = 0) noexcept;

inline QVector<QPointF> QRawFont::advancesForGlyphIndexes(const QVector<quint32> &glyphIndexes, QRawFont::LayoutFlags layoutFlags) const
{
    QVector<QPointF> advances(glyphIndexes.size());
    if (advancesForGlyphIndexes(glyphIndexes.constData(), advances.data(), glyphIndexes.size(), layoutFlags))
        return advances;
    return QVector<QPointF>();
}

inline QVector<QPointF> QRawFont::advancesForGlyphIndexes(const QVector<quint32> &glyphIndexes) const
{
    return advancesForGlyphIndexes(glyphIndexes, QRawFont::SeparateAdvances);
}

QT_END_NAMESPACE

#endif // QT_NO_RAWFONT

#endif // QRAWFONT_H
