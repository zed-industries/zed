// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QACCESSIBLEPLUGIN_H
#define QACCESSIBLEPLUGIN_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qaccessible.h>
#include <QtCore/qfactoryinterface.h>

QT_BEGIN_NAMESPACE


#if QT_CONFIG(accessibility)

class QAccessibleInterface;

#define QAccessibleFactoryInterface_iid "org.qt-project.Qt.QAccessibleFactoryInterface"

class QAccessiblePluginPrivate;

class Q_GUI_EXPORT QAccessiblePlugin : public QObject
{
    Q_OBJECT
public:
    explicit QAccessiblePlugin(QObject *parent = nullptr);
    ~QAccessiblePlugin();

    virtual QAccessibleInterface *create(const QString &key, QObject *object) = 0;
};

#endif // QT_CONFIG(accessibility)

QT_END_NAMESPACE

#endif // QACCESSIBLEPLUGIN_H
