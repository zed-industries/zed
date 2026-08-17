// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFONTMETRICS_H
#define QFONTMETRICS_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qfont.h>
#include <QtCore/qsharedpointer.h>
#ifndef QT_INCLUDE_COMPAT
#include <QtCore/qrect.h>
#endif

QT_BEGIN_NAMESPACE

class QRect;
class QTextOption;

class Q_GUI_EXPORT QFontMetrics
{
public:
    explicit QFontMetrics(const QFont &);
    QFontMetrics(const QFont &font, const QPaintDevice *pd);
    QFontMetrics(const QFontMetrics &);
    ~QFontMetrics();

    QFontMetrics &operator=(const QFontMetrics &);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QFontMetrics)

    void swap(QFontMetrics &other) noexcept
    { d.swap(other.d); }

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

    int horizontalAdvance(const QString &, int len = -1) const;
    int horizontalAdvance(const QString &, const QTextOption &textOption) const;
    int horizontalAdvance(QChar) const;

    QRect boundingRect(QChar) const;

    QRect boundingRect(const QString &text) const;
    QRect boundingRect(const QString &text, const QTextOption &textOption) const;
    QRect boundingRect(const QRect &r, int flags, const QString &text, int tabstops = 0, int *tabarray = nullptr) const;
    inline QRect boundingRect(int x, int y, int w, int h, int flags, const QString &text,
                              int tabstops = 0, int *tabarray = nullptr) const
        { return boundingRect(QRect(x, y, w, h), flags, text, tabstops, tabarray); }
    QSize size(int flags, const QString& str, int tabstops = 0, int *tabarray = nullptr) const;

    QRect tightBoundingRect(const QString &text) const;
    QRect tightBoundingRect(const QString &text, const QTextOption &textOption) const;

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
    QFontMetricsF(const QFont &font, const QPaintDevice *pd);
    QFontMetricsF(const QFontMetrics &);
    QFontMetricsF(const QFontMetricsF &);
    ~QFontMetricsF();

    QFontMetricsF &operator=(const QFontMetricsF &);
    QFontMetricsF &operator=(const QFontMetrics &);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QFontMetricsF)

    void swap(QFontMetricsF &other) noexcept { d.swap(other.d); }

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

    qreal horizontalAdvance(const QString &string, int length = -1) const;
    qreal horizontalAdvance(QChar) const;
    qreal horizontalAdvance(const QString &string, const QTextOption &textOption) const;

    QRectF boundingRect(const QString &string) const;
    QRectF boundingRect(const QString &text, const QTextOption &textOption) const;
    QRectF boundingRect(QChar) const;
    QRectF boundingRect(const QRectF &r, int flags, const QString& string, int tabstops = 0, int *tabarray = nullptr) const;
    QSizeF size(int flags, const QString& str, int tabstops = 0, int *tabarray = nullptr) const;

    QRectF tightBoundingRect(const QString &text) const;
    QRectF tightBoundingRect(const QString &text, const QTextOption &textOption) const;

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
