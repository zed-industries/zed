// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPAINTERPATH_H
#define QPAINTERPATH_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qtransform.h>
#include <QtCore/qglobal.h>
#include <QtCore/qline.h>
#include <QtCore/qlist.h>
#include <QtCore/qrect.h>
#include <QtCore/qshareddata.h>

QT_BEGIN_NAMESPACE


class QFont;
class QPainterPathPrivate;
struct QPainterPathPrivateDeleter;
class QPainterPathStrokerPrivate;
class QPen;
class QPolygonF;
class QRegion;
class QTransform;
class QVectorPath;

class Q_GUI_EXPORT QPainterPath
{
public:
    enum ElementType {
        MoveToElement,
        LineToElement,
        CurveToElement,
        CurveToDataElement
    };

    class Element {
    public:
        qreal x;
        qreal y;
        ElementType type;

        bool isMoveTo() const { return type == MoveToElement; }
        bool isLineTo() const { return type == LineToElement; }
        bool isCurveTo() const { return type == CurveToElement; }

        operator QPointF () const { return QPointF(x, y); }

        bool operator==(const Element &e) const { return qFuzzyCompare(x, e.x)
            && qFuzzyCompare(y, e.y) && type == e.type; }
        inline bool operator!=(const Element &e) const { return !operator==(e); }
    };

    QPainterPath() noexcept;
    explicit QPainterPath(const QPointF &startPoint);
    QPainterPath(const QPainterPath &other);
    QPainterPath &operator=(const QPainterPath &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QPainterPath)
    ~QPainterPath();

    inline void swap(QPainterPath &other) noexcept { d_ptr.swap(other.d_ptr); }

    void clear();
    void reserve(int size);
    int capacity() const;

    void closeSubpath();

    void moveTo(const QPointF &p);
    inline void moveTo(qreal x, qreal y);

    void lineTo(const QPointF &p);
    inline void lineTo(qreal x, qreal y);

    void arcMoveTo(const QRectF &rect, qreal angle);
    inline void arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle);

    void arcTo(const QRectF &rect, qreal startAngle, qreal arcLength);
    inline void arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength);

    void cubicTo(const QPointF &ctrlPt1, const QPointF &ctrlPt2, const QPointF &endPt);
    inline void cubicTo(qreal ctrlPt1x, qreal ctrlPt1y, qreal ctrlPt2x, qreal ctrlPt2y,
                        qreal endPtx, qreal endPty);
    void quadTo(const QPointF &ctrlPt, const QPointF &endPt);
    inline void quadTo(qreal ctrlPtx, qreal ctrlPty, qreal endPtx, qreal endPty);

    QPointF currentPosition() const;

    void addRect(const QRectF &rect);
    inline void addRect(qreal x, qreal y, qreal w, qreal h);
    void addEllipse(const QRectF &rect);
    inline void addEllipse(qreal x, qreal y, qreal w, qreal h);
    inline void addEllipse(const QPointF &center, qreal rx, qreal ry);
    void addPolygon(const QPolygonF &polygon);
    void addText(const QPointF &point, const QFont &f, const QString &text);
    inline void addText(qreal x, qreal y, const QFont &f, const QString &text);
    void addPath(const QPainterPath &path);
    void addRegion(const QRegion &region);

    void addRoundedRect(const QRectF &rect, qreal xRadius, qreal yRadius,
                        Qt::SizeMode mode = Qt::AbsoluteSize);
    inline void addRoundedRect(qreal x, qreal y, qreal w, qreal h,
                               qreal xRadius, qreal yRadius,
                               Qt::SizeMode mode = Qt::AbsoluteSize);

    void connectPath(const QPainterPath &path);

    bool contains(const QPointF &pt) const;
    bool contains(const QRectF &rect) const;
    bool intersects(const QRectF &rect) const;

    void translate(qreal dx, qreal dy);
    inline void translate(const QPointF &offset);

    [[nodiscard]] QPainterPath translated(qreal dx, qreal dy) const;
    [[nodiscard]] inline QPainterPath translated(const QPointF &offset) const;

    QRectF boundingRect() const;
    QRectF controlPointRect() const;

    Qt::FillRule fillRule() const;
    void setFillRule(Qt::FillRule fillRule);

    bool isEmpty() const;

    [[nodiscard]] QPainterPath toReversed() const;

    QList<QPolygonF> toSubpathPolygons(const QTransform &matrix = QTransform()) const;
    QList<QPolygonF> toFillPolygons(const QTransform &matrix = QTransform()) const;
    QPolygonF toFillPolygon(const QTransform &matrix = QTransform()) const;

    int elementCount() const;
    QPainterPath::Element elementAt(int i) const;
    void setElementPositionAt(int i, qreal x, qreal y);

    qreal   length() const;
    qreal   percentAtLength(qreal t) const;
    QPointF pointAtPercent(qreal t) const;
    qreal   angleAtPercent(qreal t) const;
    qreal   slopeAtPercent(qreal t) const;

    bool intersects(const QPainterPath &p) const;
    bool contains(const QPainterPath &p) const;
    [[nodiscard]] QPainterPath united(const QPainterPath &r) const;
    [[nodiscard]] QPainterPath intersected(const QPainterPath &r) const;
    [[nodiscard]] QPainterPath subtracted(const QPainterPath &r) const;

    [[nodiscard]] QPainterPath simplified() const;

    bool operator==(const QPainterPath &other) const;
    bool operator!=(const QPainterPath &other) const;

    QPainterPath operator&(const QPainterPath &other) const;
    QPainterPath operator|(const QPainterPath &other) const;
    QPainterPath operator+(const QPainterPath &other) const;
    QPainterPath operator-(const QPainterPath &other) const;
    QPainterPath &operator&=(const QPainterPath &other);
    QPainterPath &operator|=(const QPainterPath &other);
    QPainterPath &operator+=(const QPainterPath &other);
    QPainterPath &operator-=(const QPainterPath &other);

private:
    QExplicitlySharedDataPointer<QPainterPathPrivate> d_ptr;

    inline void ensureData() { if (!d_ptr) ensureData_helper(); }
    void ensureData_helper();
    void detach();
    void setDirty(bool);
    void computeBoundingRect() const;
    void computeControlPointRect() const;

    QPainterPathPrivate *d_func() const { return d_ptr.data(); }

    friend class QPainterPathStroker;
    friend class QPainterPathStrokerPrivate;
    friend class QTransform;
    friend class QVectorPath;
    friend Q_GUI_EXPORT const QVectorPath &qtVectorPathForPath(const QPainterPath &);

#ifndef QT_NO_DATASTREAM
    friend Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QPainterPath &);
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QPainterPath &);
#endif
};

Q_DECLARE_SHARED(QPainterPath)
Q_DECLARE_TYPEINFO(QPainterPath::Element, Q_PRIMITIVE_TYPE);

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QPainterPath &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QPainterPath &);
#endif

class Q_GUI_EXPORT QPainterPathStroker
{
    Q_DECLARE_PRIVATE(QPainterPathStroker)
public:
    QPainterPathStroker();
    explicit QPainterPathStroker(const QPen &pen);
    ~QPainterPathStroker();

    void setWidth(qreal width);
    qreal width() const;

    void setCapStyle(Qt::PenCapStyle style);
    Qt::PenCapStyle capStyle() const;

    void setJoinStyle(Qt::PenJoinStyle style);
    Qt::PenJoinStyle joinStyle() const;

    void setMiterLimit(qreal length);
    qreal miterLimit() const;

    void setCurveThreshold(qreal threshold);
    qreal curveThreshold() const;

    void setDashPattern(Qt::PenStyle);
    void setDashPattern(const QList<qreal> &dashPattern);
    QList<qreal> dashPattern() const;

    void setDashOffset(qreal offset);
    qreal dashOffset() const;

    QPainterPath createStroke(const QPainterPath &path) const;

private:
    Q_DISABLE_COPY(QPainterPathStroker)

    friend class QX11PaintEngine;

    QScopedPointer<QPainterPathStrokerPrivate> d_ptr;
};

inline void QPainterPath::moveTo(qreal x, qreal y)
{
    moveTo(QPointF(x, y));
}

inline void QPainterPath::lineTo(qreal x, qreal y)
{
    lineTo(QPointF(x, y));
}

inline void QPainterPath::arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength)
{
    arcTo(QRectF(x, y, w, h), startAngle, arcLength);
}

inline void QPainterPath::arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle)
{
    arcMoveTo(QRectF(x, y, w, h), angle);
}

inline void QPainterPath::cubicTo(qreal ctrlPt1x, qreal ctrlPt1y, qreal ctrlPt2x, qreal ctrlPt2y,
                                   qreal endPtx, qreal endPty)
{
    cubicTo(QPointF(ctrlPt1x, ctrlPt1y), QPointF(ctrlPt2x, ctrlPt2y),
            QPointF(endPtx, endPty));
}

inline void QPainterPath::quadTo(qreal ctrlPtx, qreal ctrlPty, qreal endPtx, qreal endPty)
{
    quadTo(QPointF(ctrlPtx, ctrlPty), QPointF(endPtx, endPty));
}

inline void QPainterPath::addEllipse(qreal x, qreal y, qreal w, qreal h)
{
    addEllipse(QRectF(x, y, w, h));
}

inline void QPainterPath::addEllipse(const QPointF &center, qreal rx, qreal ry)
{
    addEllipse(QRectF(center.x() - rx, center.y() - ry, 2 * rx, 2 * ry));
}

inline void QPainterPath::addRect(qreal x, qreal y, qreal w, qreal h)
{
    addRect(QRectF(x, y, w, h));
}

inline void QPainterPath::addRoundedRect(qreal x, qreal y, qreal w, qreal h,
                                         qreal xRadius, qreal yRadius,
                                         Qt::SizeMode mode)
{
    addRoundedRect(QRectF(x, y, w, h), xRadius, yRadius, mode);
}

inline void QPainterPath::addText(qreal x, qreal y, const QFont &f, const QString &text)
{
    addText(QPointF(x, y), f, text);
}

inline void QPainterPath::translate(const QPointF &offset)
{ translate(offset.x(), offset.y()); }

inline QPainterPath QPainterPath::translated(const QPointF &offset) const
{ return translated(offset.x(), offset.y()); }

inline QPainterPath operator *(const QPainterPath &p, const QTransform &m)
{ return m.map(p); }

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QPainterPath &);
#endif

QT_END_NAMESPACE

#endif // QPAINTERPATH_H
