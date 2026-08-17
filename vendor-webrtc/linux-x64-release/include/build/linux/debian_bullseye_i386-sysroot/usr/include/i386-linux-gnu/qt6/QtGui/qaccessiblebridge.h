// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QACCESSIBLEBRIDGE_H
#define QACCESSIBLEBRIDGE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qplugin.h>
#include <QtCore/qfactoryinterface.h>

QT_BEGIN_NAMESPACE


#if QT_CONFIG(accessibility)

class QAccessibleInterface;
class QAccessibleEvent;

class QAccessibleBridge
{
public:
    virtual ~QAccessibleBridge() {}
    virtual void setRootObject(QAccessibleInterface *) = 0;
    virtual void notifyAccessibilityUpdate(QAccessibleEvent *event) = 0;
};

#define QAccessibleBridgeFactoryInterface_iid "org.qt-project.Qt.QAccessibleBridgeFactoryInterface"

class Q_GUI_EXPORT QAccessibleBridgePlugin : public QObject
{
    Q_OBJECT
public:
    explicit QAccessibleBridgePlugin(QObject *parent = nullptr);
    ~QAccessibleBridgePlugin();

    virtual QAccessibleBridge *create(const QString &key) = 0;
};

#endif // QT_CONFIG(accessibility)

QT_END_NAMESPACE

#endif // QACCESSIBLEBRIDGE_H
