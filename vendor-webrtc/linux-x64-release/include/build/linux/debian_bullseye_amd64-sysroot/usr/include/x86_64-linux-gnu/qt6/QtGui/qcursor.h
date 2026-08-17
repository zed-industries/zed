// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCURSOR_H
#define QCURSOR_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qpoint.h>
#include <QtGui/qwindowdefs.h>
#include <QtGui/qbitmap.h>

QT_BEGIN_NAMESPACE


class QVariant;
class QScreen;

/*
  ### The fake cursor has to go first with old qdoc.
*/
#ifdef QT_NO_CURSOR

class Q_GUI_EXPORT QCursor
{
public:
    static QPoint pos();
    static QPoint pos(const QScreen *screen);
    static void setPos(int x, int y);
    static void setPos(QScreen *screen, int x, int y);
    inline static void setPos(const QPoint &p) { setPos(p.x(), p.y()); }
private:
    QCursor();
};

#endif // QT_NO_CURSOR

#ifndef QT_NO_CURSOR

class QCursorData;
class QBitmap;
class QPixmap;


class Q_GUI_EXPORT QCursor
{
public:
    QCursor();
    QCursor(Qt::CursorShape shape);
    QCursor(const QBitmap &bitmap, const QBitmap &mask, int hotX=-1, int hotY=-1);
    explicit QCursor(const QPixmap &pixmap, int hotX=-1, int hotY=-1);
    QCursor(const QCursor &cursor);
    ~QCursor();
    QCursor &operator=(const QCursor &cursor);
    QCursor(QCursor &&other) noexcept : d(qExchange(other.d, nullptr)) {}
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QCursor)

    void swap(QCursor &other) noexcept { qt_ptr_swap(d, other.d); }

    operator QVariant() const;

    Qt::CursorShape shape() const;
    void setShape(Qt::CursorShape newShape);

#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use the overload without argument instead.")
    QBitmap bitmap(Qt::ReturnByValueConstant) const { return bitmap(); }
    QT_DEPRECATED_VERSION_X_6_0("Use the overload without argument instead.")
    QBitmap mask(Qt::ReturnByValueConstant) const { return mask(); }
#endif // QT_DEPRECATED_SINCE(6, 0)
    QBitmap bitmap() const;
    QBitmap mask() const;

    QPixmap pixmap() const;
    QPoint hotSpot() const;

    static QPoint pos();
    static QPoint pos(const QScreen *screen);
    static void setPos(int x, int y);
    static void setPos(QScreen *screen, int x, int y);
    inline static void setPos(const QPoint &p) { setPos(p.x(), p.y()); }
    inline static void setPos(QScreen *screen, const QPoint &p) { setPos(screen, p.x(), p.y()); }

private:
    friend Q_GUI_EXPORT bool operator==(const QCursor &lhs, const QCursor &rhs) noexcept;
    friend inline bool operator!=(const QCursor &lhs, const QCursor &rhs) noexcept { return !(lhs == rhs); }
    QCursorData *d;
};
Q_DECLARE_SHARED(QCursor)

/*****************************************************************************
  QCursor stream functions
 *****************************************************************************/
#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &outS, const QCursor &cursor);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &inS, QCursor &cursor);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QCursor &);
#endif

#endif // QT_NO_CURSOR

QT_END_NAMESPACE

#endif // QCURSOR_H
