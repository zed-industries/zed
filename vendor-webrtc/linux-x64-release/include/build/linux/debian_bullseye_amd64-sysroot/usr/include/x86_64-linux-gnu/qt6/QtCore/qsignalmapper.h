// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSIGNALMAPPER_H
#define QSIGNALMAPPER_H

#include <QtCore/qobject.h>

QT_BEGIN_NAMESPACE

class QSignalMapperPrivate;

class Q_CORE_EXPORT QSignalMapper : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QSignalMapper)
public:
    explicit QSignalMapper(QObject *parent = nullptr);
    ~QSignalMapper();

    void setMapping(QObject *sender, int id);
    void setMapping(QObject *sender, const QString &text);
    void setMapping(QObject *sender, QObject *object);
    void removeMappings(QObject *sender);

    QObject *mapping(int id) const;
    QObject *mapping(const QString &text) const;
    QObject *mapping(QObject *object) const;

Q_SIGNALS:
    void mappedInt(int);
    void mappedString(const QString &);
    void mappedObject(QObject *);

public Q_SLOTS:
    void map();
    void map(QObject *sender);

private:
    Q_DISABLE_COPY(QSignalMapper)
    Q_PRIVATE_SLOT(d_func(), void _q_senderDestroyed())
};

QT_END_NAMESPACE

#endif // QSIGNALMAPPER_H
