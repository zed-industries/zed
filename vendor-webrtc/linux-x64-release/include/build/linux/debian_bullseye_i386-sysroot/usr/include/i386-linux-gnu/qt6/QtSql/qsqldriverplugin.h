// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSQLDRIVERPLUGIN_H
#define QSQLDRIVERPLUGIN_H

#include <QtSql/qtsqlglobal.h>
#include <QtCore/qplugin.h>
#include <QtCore/qfactoryinterface.h>

QT_BEGIN_NAMESPACE


class QSqlDriver;

#define QSqlDriverFactoryInterface_iid "org.qt-project.Qt.QSqlDriverFactoryInterface"

class Q_SQL_EXPORT QSqlDriverPlugin : public QObject
{
    Q_OBJECT
public:
    explicit QSqlDriverPlugin(QObject *parent = nullptr);
    ~QSqlDriverPlugin();

    virtual QSqlDriver *create(const QString &key) = 0;

};

QT_END_NAMESPACE

#endif // QSQLDRIVERPLUGIN_H
