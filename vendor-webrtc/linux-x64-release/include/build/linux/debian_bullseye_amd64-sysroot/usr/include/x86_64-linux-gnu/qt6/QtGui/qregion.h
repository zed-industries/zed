// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QREGION_H
#define QREGION_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qatomic.h>
#include <QtCore/qrect.h>
#include <QtGui/qwindowdefs.h>
#include <QtCore/qcontainerfwd.h>

#ifndef QT_NO_DATASTREAM
#include <QtCore/qdatastream.h>
#endif

QT_BEGIN_NAMESPACE


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
        : d(qExchange(other.d, const_cast<QRegionData*>(&shared_empty))) {}
    QRegion(const QBitmap &bitmap);
    ~QRegion();
    QRegion &operator=(const QRegion &);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QRegion)
    void swap(QRegion &other) noexcept { qt_ptr_swap(d, other.d); }
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
    [[nodiscard]] QRegion translated(int dx, int dy) const;
    [[nodiscard]] inline QRegion translated(const QPoint &p) const { return translated(p.x(), p.y()); }

    [[nodiscard]] QRegion united(const QRegion &r) const;
    [[nodiscard]] QRegion united(const QRect &r) const;
    [[nodiscard]] QRegion intersected(const QRegion &r) const;
    [[nodiscard]] QRegion intersected(const QRect &r) const;
    [[nodiscard]] QRegion subtracted(const QRegion &r) const;
    [[nodiscard]] QRegion xored(const QRegion &r) const;

    bool intersects(const QRegion &r) const;
    bool intersects(const QRect &r) const;

    QRect boundingRect() const noexcept;
    void setRects(const QRect *rect, int num);
    int rectCount() const noexcept;

    QRegion operator|(const QRegion &r) const;
    QRegion operator+(const QRegion &r) const;
    QRegion operator+(const QRect &r) const;
    QRegion operator&(const QRegion &r) const;
    QRegion operator&(const QRect &r) const;
    QRegion operator-(const QRegion &r) const;
    QRegion operator^(const QRegion &r) const;

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

    // Platform specific conversion functions
#if defined(Q_OS_WIN) || defined(Q_QDOC)
    HRGN toHRGN() const;
    static QRegion fromHRGN(HRGN hrgn);
#endif

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
Q_DECLARE_SHARED(QRegion)

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
