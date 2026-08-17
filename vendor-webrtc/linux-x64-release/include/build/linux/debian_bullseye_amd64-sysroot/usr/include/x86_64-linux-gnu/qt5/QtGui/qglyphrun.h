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

#ifndef QGLYPHRUN_H
#define QGLYPHRUN_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qsharedpointer.h>
#include <QtCore/qvector.h>
#include <QtCore/qpoint.h>
#include <QtGui/qrawfont.h>

#if !defined(QT_NO_RAWFONT)

QT_BEGIN_NAMESPACE


class QGlyphRunPrivate;
class Q_GUI_EXPORT QGlyphRun
{
public:
    enum GlyphRunFlag {
        Overline        = 0x01,
        Underline       = 0x02,
        StrikeOut       = 0x04,
        RightToLeft     = 0x08,
        SplitLigature   = 0x10
    };
    Q_DECLARE_FLAGS(GlyphRunFlags, GlyphRunFlag)

    QGlyphRun();
    QGlyphRun(const QGlyphRun &other);
    QGlyphRun &operator=(QGlyphRun &&other) noexcept { swap(other); return *this; }
    QGlyphRun &operator=(const QGlyphRun &other);
    ~QGlyphRun();

    void swap(QGlyphRun &other) noexcept { qSwap(d, other.d); }

    QRawFont rawFont() const;
    void setRawFont(const QRawFont &rawFont);

    void setRawData(const quint32 *glyphIndexArray,
                    const QPointF *glyphPositionArray,
                    int size);

    QVector<quint32> glyphIndexes() const;
    void setGlyphIndexes(const QVector<quint32> &glyphIndexes);

    QVector<QPointF> positions() const;
    void setPositions(const QVector<QPointF> &positions);

    void clear();

    bool operator==(const QGlyphRun &other) const;
    inline bool operator!=(const QGlyphRun &other) const
    { return !operator==(other); }

    void setOverline(bool overline);
    bool overline() const;

    void setUnderline(bool underline);
    bool underline() const;

    void setStrikeOut(bool strikeOut);
    bool strikeOut() const;

    void setRightToLeft(bool on);
    bool isRightToLeft() const;

    void setFlag(GlyphRunFlag flag, bool enabled = true);
    void setFlags(GlyphRunFlags flags);
    GlyphRunFlags flags() const;

    void setBoundingRect(const QRectF &boundingRect);
    QRectF boundingRect() const;

    bool isEmpty() const;

private:
    friend class QGlyphRunPrivate;
    friend class QTextLine;

    QGlyphRun operator+(const QGlyphRun &other) const;
    QGlyphRun &operator+=(const QGlyphRun &other);

    void detach();
    QExplicitlySharedDataPointer<QGlyphRunPrivate> d;
};

Q_DECLARE_SHARED(QGlyphRun)

QT_END_NAMESPACE

#endif // QT_NO_RAWFONT

#endif // QGLYPHRUN_H
