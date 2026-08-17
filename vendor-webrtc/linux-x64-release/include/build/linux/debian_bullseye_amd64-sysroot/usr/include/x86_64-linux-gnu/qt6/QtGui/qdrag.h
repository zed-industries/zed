// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDRAG_H
#define QDRAG_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qobject.h>

QT_REQUIRE_CONFIG(draganddrop);

QT_BEGIN_NAMESPACE

class QMimeData;
class QDragPrivate;
class QPixmap;
class QPoint;
class QDragManager;


class Q_GUI_EXPORT QDrag : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QDrag)
public:
    explicit QDrag(QObject *dragSource);
    ~QDrag();

    void setMimeData(QMimeData *data);
    QMimeData *mimeData() const;

    void setPixmap(const QPixmap &);
    QPixmap pixmap() const;

    void setHotSpot(const QPoint &hotspot);
    QPoint hotSpot() const;

    QObject *source() const;
    QObject *target() const;

    Qt::DropAction exec(Qt::DropActions supportedActions = Qt::MoveAction);
    Qt::DropAction exec(Qt::DropActions supportedActions, Qt::DropAction defaultAction);

    void setDragCursor(const QPixmap &cursor, Qt::DropAction action);
    QPixmap dragCursor(Qt::DropAction action) const;

    Qt::DropActions supportedActions() const;
    Qt::DropAction defaultAction() const;

    static void cancel();

Q_SIGNALS:
    void actionChanged(Qt::DropAction action);
    void targetChanged(QObject *newTarget);

private:
    friend class QDragManager;
    Q_DISABLE_COPY(QDrag)
};

QT_END_NAMESPACE

#endif // QDRAG_H
