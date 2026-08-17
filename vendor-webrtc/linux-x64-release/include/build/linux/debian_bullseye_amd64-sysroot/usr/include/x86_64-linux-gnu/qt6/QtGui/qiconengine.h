// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QICONENGINE_H
#define QICONENGINE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qlist.h>
#include <QtGui/qicon.h>

QT_BEGIN_NAMESPACE


class Q_GUI_EXPORT QIconEngine
{
public:
    QIconEngine();
    virtual ~QIconEngine();
    virtual void paint(QPainter *painter, const QRect &rect, QIcon::Mode mode, QIcon::State state) = 0;
    virtual QSize actualSize(const QSize &size, QIcon::Mode mode, QIcon::State state);
    virtual QPixmap pixmap(const QSize &size, QIcon::Mode mode, QIcon::State state);

    virtual void addPixmap(const QPixmap &pixmap, QIcon::Mode mode, QIcon::State state);
    virtual void addFile(const QString &fileName, const QSize &size, QIcon::Mode mode, QIcon::State state);

    virtual QString key() const;
    virtual QIconEngine *clone() const = 0;
    virtual bool read(QDataStream &in);
    virtual bool write(QDataStream &out) const;

    virtual QList<QSize> availableSizes(QIcon::Mode mode = QIcon::Normal,
                                    QIcon::State state = QIcon::Off);

    virtual QString iconName();
    virtual bool isNull();
    virtual QPixmap scaledPixmap(const QSize &size, QIcon::Mode mode, QIcon::State state, qreal scale);

    enum IconEngineHook { IsNullHook = 3, ScaledPixmapHook };

    struct ScaledPixmapArgument
    {
        QSize size;
        QIcon::Mode mode;
        QIcon::State state;
        qreal scale;
        QPixmap pixmap;
    };

    virtual void virtual_hook(int id, void *data);

protected:
    QIconEngine(const QIconEngine &other);

private:
    QIconEngine &operator=(const QIconEngine &other) = delete;
};

QT_END_NAMESPACE

#endif // QICONENGINE_H
