// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QGLYPHRUN_H
#define QGLYPHRUN_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qlist.h>
#include <QtCore/qpoint.h>
#include <QtCore/qsharedpointer.h>
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
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QGlyphRun)
    QGlyphRun &operator=(const QGlyphRun &other);
    ~QGlyphRun();

    void swap(QGlyphRun &other) noexcept { d.swap(other.d); }

    QRawFont rawFont() const;
    void setRawFont(const QRawFont &rawFont);

    void setRawData(const quint32 *glyphIndexArray,
                    const QPointF *glyphPositionArray,
                    int size);

    QList<quint32> glyphIndexes() const;
    void setGlyphIndexes(const QList<quint32> &glyphIndexes);

    QList<QPointF> positions() const;
    void setPositions(const QList<QPointF> &positions);

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
