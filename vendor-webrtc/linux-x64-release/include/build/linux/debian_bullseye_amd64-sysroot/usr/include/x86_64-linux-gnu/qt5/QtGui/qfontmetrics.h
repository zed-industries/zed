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

#ifndef QFONTMETRICS_H
#define QFONTMETRICS_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qfont.h>
#include <QtCore/qsharedpointer.h>
#ifndef QT_INCLUDE_COMPAT
#include <QtCore/qrect.h>
#endif

QT_BEGIN_NAMESPACE



class QTextCodec;
class QRect;


class Q_GUI_EXPORT QFontMetrics
{
public:
    explicit QFontMetrics(const QFont &);
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    QFontMetrics(const QFont &font, QPaintDevice *pd);
#ifndef Q_QDOC
    // the template is necessary to make QFontMetrics(font,nullptr) and QFontMetrics(font,NULL)
    // not ambiguous. Implementation detail that should not be documented.
    template<char = 0>
#endif
    QFontMetrics(const QFont &font, const QPaintDevice *pd)
        : QFontMetrics(font, const_cast<QPaintDevice*>(pd))
    {}
#else
    QFontMetrics(const QFont &font, const QPaintDevice *pd);
#endif
    QFontMetrics(const QFontMetrics &);
    ~QFontMetrics();

    QFontMetrics &operator=(const QFontMetrics &);
    inline QFontMetrics &operator=(QFontMetrics &&other) noexcept
    { qSwap(d, other.d); return *this; }

    void swap(QFontMetrics &other) noexcept
    { qSwap(d, other.d); }

    int ascent() const;
    int capHeight() const;
    int descent() const;
    int height() const;
    int leading() const;
    int lineSpacing() const;
    int minLeftBearing() const;
    int minRightBearing() const;
    int maxWidth() const;

    int xHeight() const;
    int averageCharWidth() const;

    bool inFont(QChar) const;
    bool inFontUcs4(uint ucs4) const;

    int leftBearing(QChar) const;
    int rightBearing(QChar) const;

#if QT_DEPRECATED_SINCE(5, 11)
    QT_DEPRECATED_X("Use QFontMetrics::horizontalAdvance")
    int width(const QString &, int len = -1) const;
    QT_DEPRECATED_X("Use QFontMetrics::horizontalAdvance")
    int width(const QString &, int len, int flags) const;
    QT_DEPRECATED_X("Use QFontMetrics::horizontalAdvance")
    int width(QChar) const;
#endif

    int horizontalAdvance(const QString &, int len = -1) const;
    int horizontalAdvance(QChar) const;

#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
    QT_DEPRECATED int charWidth(const QString &str, int pos) const;
#endif

    QRect boundingRect(QChar) const;

    QRect boundingRect(const QString &text) const;
    QRect boundingRect(const QRect &r, int flags, const QString &text, int tabstops = 0, int *tabarray = nullptr) const;
    inline QRect boundingRect(int x, int y, int w, int h, int flags, const QString &text,
                              int tabstops = 0, int *tabarray = nullptr) const
        { return boundingRect(QRect(x, y, w, h), flags, text, tabstops, tabarray); }
    QSize size(int flags, const QString& str, int tabstops = 0, int *tabarray = nullptr) const;

    QRect tightBoundingRect(const QString &text) const;

    QString elidedText(const QString &text, Qt::TextElideMode mode, int width, int flags = 0) const;

    int underlinePos() const;
    int overlinePos() const;
    int strikeOutPos() const;
    int lineWidth() const;

    qreal fontDpi() const;

    bool operator==(const QFontMetrics &other) const;
    inline bool operator !=(const QFontMetrics &other) const { return !operator==(other); }

private:
    friend class QFontMetricsF;
    friend class QStackTextEngine;

    QExplicitlySharedDataPointer<QFontPrivate> d;
};

Q_DECLARE_SHARED(QFontMetrics)

class Q_GUI_EXPORT QFontMetricsF
{
public:
    explicit QFontMetricsF(const QFont &font);
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    QFontMetricsF(const QFont &font, QPaintDevice *pd);
#ifndef Q_QDOC
    // the template is necessary to make QFontMetrics(font,nullptr) and QFontMetrics(font,NULL)
    // not ambiguous. Implementation detail that should not be documented.
    template<char = 0>
#endif
    QFontMetricsF(const QFont &font, const QPaintDevice *pd)
        : QFontMetricsF(font, const_cast<QPaintDevice*>(pd))
    {}
#else
    QFontMetricsF(const QFont &font, const QPaintDevice *pd);
#endif
    QFontMetricsF(const QFontMetrics &);
    QFontMetricsF(const QFontMetricsF &);
    ~QFontMetricsF();

    QFontMetricsF &operator=(const QFontMetricsF &);
    QFontMetricsF &operator=(const QFontMetrics &);
    inline QFontMetricsF &operator=(QFontMetricsF &&other) noexcept
    { qSwap(d, other.d); return *this; }

    void swap(QFontMetricsF &other) noexcept { qSwap(d, other.d); }

    qreal ascent() const;
    qreal capHeight() const;
    qreal descent() const;
    qreal height() const;
    qreal leading() const;
    qreal lineSpacing() const;
    qreal minLeftBearing() const;
    qreal minRightBearing() const;
    qreal maxWidth() const;

    qreal xHeight() const;
    qreal averageCharWidth() const;

    bool inFont(QChar) const;
    bool inFontUcs4(uint ucs4) const;

    qreal leftBearing(QChar) const;
    qreal rightBearing(QChar) const;

#if QT_DEPRECATED_SINCE(5, 11)
    qreal width(const QString &string) const;
    qreal width(QChar) const;
#endif

    qreal horizontalAdvance(const QString &string, int length = -1) const;
    qreal horizontalAdvance(QChar) const;

    QRectF boundingRect(const QString &string) const;
    QRectF boundingRect(QChar) const;
    QRectF boundingRect(const QRectF &r, int flags, const QString& string, int tabstops = 0, int *tabarray = nullptr) const;
    QSizeF size(int flags, const QString& str, int tabstops = 0, int *tabarray = nullptr) const;

    QRectF tightBoundingRect(const QString &text) const;

    QString elidedText(const QString &text, Qt::TextElideMode mode, qreal width, int flags = 0) const;

    qreal underlinePos() const;
    qreal overlinePos() const;
    qreal strikeOutPos() const;
    qreal lineWidth() const;

    qreal fontDpi() const;

    bool operator==(const QFontMetricsF &other) const;
    inline bool operator !=(const QFontMetricsF &other) const { return !operator==(other); }

private:
    QExplicitlySharedDataPointer<QFontPrivate> d;
};

Q_DECLARE_SHARED(QFontMetricsF)

QT_END_NAMESPACE

#endif // QFONTMETRICS_H
