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

#ifndef QREGION_H
#define QREGION_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qatomic.h>
#include <QtCore/qrect.h>
#include <QtGui/qwindowdefs.h>

#ifndef QT_NO_DATASTREAM
#include <QtCore/qdatastream.h>
#endif

QT_BEGIN_NAMESPACE


template <class T> class QVector;
class QVariant;

struct QRegionPrivate;

class QBitmap;

class Q_GUI_EXPORT QRegion
{
public:
    enum RegionType { Rectangle, Ellipse };

    QRegion();
    QRegion(int x, int y, int w, int h, RegionType t = Rectangle);
    QRegion(const QRect &r, RegionType t = Rectangle);
    QRegion(const QPolygon &pa, Qt::FillRule fillRule = Qt::OddEvenFill);
    QRegion(const QRegion &region);
    QRegion(QRegion &&other) noexcept
        : d(other.d) { other.d = const_cast<QRegionData*>(&shared_empty); }
    QRegion(const QBitmap &bitmap);
    ~QRegion();
    QRegion &operator=(const QRegion &);
    inline QRegion &operator=(QRegion &&other) noexcept
    { qSwap(d, other.d); return *this; }
    inline void swap(QRegion &other) noexcept { qSwap(d, other.d); }
    bool isEmpty() const;
    bool isNull() const;

    typedef const QRect *const_iterator;
    typedef std::reverse_iterator<const_iterator> const_reverse_iterator;

    const_iterator begin()  const noexcept;
    const_iterator cbegin() const noexcept { return begin(); }
    const_iterator end()    const noexcept;
    const_iterator cend()   const noexcept { return end(); }
    const_reverse_iterator rbegin()  const noexcept { return const_reverse_iterator(end()); }
    const_reverse_iterator crbegin() const noexcept { return rbegin(); }
    const_reverse_iterator rend()    const noexcept { return const_reverse_iterator(begin()); }
    const_reverse_iterator crend()   const noexcept { return rend(); }

    bool contains(const QPoint &p) const;
    bool contains(const QRect &r) const;

    void translate(int dx, int dy);
    inline void translate(const QPoint &p) { translate(p.x(), p.y()); }
    Q_REQUIRED_RESULT QRegion translated(int dx, int dy) const;
    Q_REQUIRED_RESULT inline QRegion translated(const QPoint &p) const { return translated(p.x(), p.y()); }

    Q_REQUIRED_RESULT QRegion united(const QRegion &r) const;
    Q_REQUIRED_RESULT QRegion united(const QRect &r) const;
    Q_REQUIRED_RESULT QRegion intersected(const QRegion &r) const;
    Q_REQUIRED_RESULT QRegion intersected(const QRect &r) const;
    Q_REQUIRED_RESULT QRegion subtracted(const QRegion &r) const;
    Q_REQUIRED_RESULT QRegion xored(const QRegion &r) const;

#if QT_DEPRECATED_SINCE(5, 0)
    Q_REQUIRED_RESULT inline QT_DEPRECATED QRegion unite(const QRegion &r) const { return united(r); }
    Q_REQUIRED_RESULT inline QT_DEPRECATED QRegion unite(const QRect &r) const { return united(r); }
    Q_REQUIRED_RESULT inline QT_DEPRECATED QRegion intersect(const QRegion &r) const { return intersected(r); }
    Q_REQUIRED_RESULT inline QT_DEPRECATED QRegion intersect(const QRect &r) const { return intersected(r); }
    Q_REQUIRED_RESULT inline QT_DEPRECATED QRegion subtract(const QRegion &r) const { return subtracted(r); }
    Q_REQUIRED_RESULT inline QT_DEPRECATED QRegion eor(const QRegion &r) const { return xored(r); }
#endif

    bool intersects(const QRegion &r) const;
    bool intersects(const QRect &r) const;

    QRect boundingRect() const noexcept;
#if QT_DEPRECATED_SINCE(5, 11)
    QT_DEPRECATED_X("Use begin()/end() instead")
    QVector<QRect> rects() const;
#endif
    void setRects(const QRect *rect, int num);
    int rectCount() const noexcept;
#ifdef Q_COMPILER_MANGLES_RETURN_TYPE
    // ### Qt 6: remove these, they're kept for MSVC compat
    const QRegion operator|(const QRegion &r) const;
    const QRegion operator+(const QRegion &r) const;
    const QRegion operator+(const QRect &r) const;
    const QRegion operator&(const QRegion &r) const;
    const QRegion operator&(const QRect &r) const;
    const QRegion operator-(const QRegion &r) const;
    const QRegion operator^(const QRegion &r) const;
#else
    QRegion operator|(const QRegion &r) const;
    QRegion operator+(const QRegion &r) const;
    QRegion operator+(const QRect &r) const;
    QRegion operator&(const QRegion &r) const;
    QRegion operator&(const QRect &r) const;
    QRegion operator-(const QRegion &r) const;
    QRegion operator^(const QRegion &r) const;
#endif // Q_COMPILER_MANGLES_RETURN_TYPE
    QRegion& operator|=(const QRegion &r);
    QRegion& operator+=(const QRegion &r);
    QRegion& operator+=(const QRect &r);
    QRegion& operator&=(const QRegion &r);
    QRegion& operator&=(const QRect &r);
    QRegion& operator-=(const QRegion &r);
    QRegion& operator^=(const QRegion &r);

    bool operator==(const QRegion &r) const;
    inline bool operator!=(const QRegion &r) const { return !(operator==(r)); }
    operator QVariant() const;

#ifndef QT_NO_DATASTREAM
    friend Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QRegion &);
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QRegion &);
#endif
private:
    QRegion copy() const;   // helper of detach.
    void detach();
Q_GUI_EXPORT
    friend bool qt_region_strictContains(const QRegion &region,
                                         const QRect &rect);
    friend struct QRegionPrivate;

#ifndef QT_NO_DATASTREAM
    void exec(const QByteArray &ba, int ver = 0, QDataStream::ByteOrder byteOrder = QDataStream::BigEndian);
#endif
    struct QRegionData {
        QtPrivate::RefCount ref;
        QRegionPrivate *qt_rgn;
    };
    struct QRegionData *d;
    static const struct QRegionData shared_empty;
    static void cleanUp(QRegionData *x);
};
Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QRegion)

/*****************************************************************************
  QRegion stream functions
 *****************************************************************************/

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QRegion &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QRegion &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QRegion &);
#endif

QT_END_NAMESPACE

#endif // QREGION_H
