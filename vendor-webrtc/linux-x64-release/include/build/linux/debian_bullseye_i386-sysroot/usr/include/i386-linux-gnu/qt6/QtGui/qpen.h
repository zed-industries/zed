// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPEN_H
#define QPEN_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qcolor.h>
#include <QtGui/qbrush.h>

QT_BEGIN_NAMESPACE


class QVariant;
class QPenPrivate;
class QBrush;
class QPen;

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QPen &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QPen &);
#endif

class Q_GUI_EXPORT QPen
{
public:
    QPen();
    QPen(Qt::PenStyle);
    QPen(const QColor &color);
    QPen(const QBrush &brush, qreal width, Qt::PenStyle s = Qt::SolidLine,
         Qt::PenCapStyle c = Qt::SquareCap, Qt::PenJoinStyle j = Qt::BevelJoin);
    QPen(const QPen &pen) noexcept;

    ~QPen();

    QPen &operator=(const QPen &pen) noexcept;
    QPen(QPen &&other) noexcept
        : d(qExchange(other.d, nullptr)) {}
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QPen)
    void swap(QPen &other) noexcept { qt_ptr_swap(d, other.d); }

    Qt::PenStyle style() const;
    void setStyle(Qt::PenStyle);

    QList<qreal> dashPattern() const;
    void setDashPattern(const QList<qreal> &pattern);

    qreal dashOffset() const;
    void setDashOffset(qreal doffset);

    qreal miterLimit() const;
    void setMiterLimit(qreal limit);

    qreal widthF() const;
    void setWidthF(qreal width);

    int width() const;
    void setWidth(int width);

    QColor color() const;
    void setColor(const QColor &color);

    QBrush brush() const;
    void setBrush(const QBrush &brush);

    bool isSolid() const;

    Qt::PenCapStyle capStyle() const;
    void setCapStyle(Qt::PenCapStyle pcs);

    Qt::PenJoinStyle joinStyle() const;
    void setJoinStyle(Qt::PenJoinStyle pcs);

    bool isCosmetic() const;
    void setCosmetic(bool cosmetic);

    bool operator==(const QPen &p) const;
    inline bool operator!=(const QPen &p) const { return !(operator==(p)); }
    operator QVariant() const;

    bool isDetached();
private:
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QPen &);
    friend Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QPen &);

    void detach();
    class QPenPrivate *d;

public:
    typedef QPenPrivate * DataPtr;
    inline DataPtr &data_ptr() { return d; }
};

Q_DECLARE_SHARED(QPen)

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QPen &);
#endif

QT_END_NAMESPACE

#endif // QPEN_H
