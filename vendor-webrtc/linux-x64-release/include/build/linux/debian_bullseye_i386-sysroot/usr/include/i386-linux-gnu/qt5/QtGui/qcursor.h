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

#ifndef QCURSOR_H
#define QCURSOR_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qpoint.h>
#include <QtGui/qwindowdefs.h>

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
    QCursor(const QPixmap &pixmap, int hotX=-1, int hotY=-1);
    QCursor(const QCursor &cursor);
    ~QCursor();
    QCursor &operator=(const QCursor &cursor);
    QCursor(QCursor &&other) noexcept : d(other.d) { other.d = nullptr; }
    inline QCursor &operator=(QCursor &&other) noexcept
    { swap(other); return *this; }

    void swap(QCursor &other) noexcept { qSwap(d, other.d); }

    operator QVariant() const;

    Qt::CursorShape shape() const;
    void setShape(Qt::CursorShape newShape);

#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_VERSION_X(5, 15, "Use the other overload which returns QBitmap by-value")
    const QBitmap *bitmap() const; // ### Qt 7: Remove function

    QT_DEPRECATED_VERSION_X(5, 15, "Use the other overload which returns QBitmap by-value")
    const QBitmap *mask() const; // ### Qt 7: Remove function

    QBitmap bitmap(Qt::ReturnByValueConstant) const;
    QBitmap mask(Qt::ReturnByValueConstant) const;
#else
    QBitmap bitmap(Qt::ReturnByValueConstant = Qt::ReturnByValue) const; // ### Qt 7: Remove arg
    QBitmap mask(Qt::ReturnByValueConstant = Qt::ReturnByValue) const; // ### Qt 7: Remove arg
#endif // QT_DEPRECATED_SINCE(5, 15)
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
    QCursorData *d;
};
Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QCursor)

Q_GUI_EXPORT bool operator==(const QCursor &lhs, const QCursor &rhs) noexcept;
inline bool operator!=(const QCursor &lhs, const QCursor &rhs) noexcept { return !(lhs == rhs); }

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
