// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QACCESSIBLEOBJECT_H
#define QACCESSIBLEOBJECT_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qaccessible.h>

QT_BEGIN_NAMESPACE


#if QT_CONFIG(accessibility)

class QAccessibleObjectPrivate;
class QObject;

class Q_GUI_EXPORT QAccessibleObject : public QAccessibleInterface
{
public:
    explicit QAccessibleObject(QObject *object);

    bool isValid() const override;
    QObject *object() const override;

    // properties
    QRect rect() const override;
    void setText(QAccessible::Text t, const QString &text) override;
    QAccessibleInterface *childAt(int x, int y) const override;

protected:
    ~QAccessibleObject();

private:
    QAccessibleObjectPrivate *d;
    Q_DISABLE_COPY(QAccessibleObject)
};

class Q_GUI_EXPORT QAccessibleApplication : public QAccessibleObject
{
public:
    QAccessibleApplication();

    QWindow *window() const override;
    // relations
    int childCount() const override;
    int indexOfChild(const QAccessibleInterface*) const override;
    QAccessibleInterface *focusChild() const override;

    // navigation
    QAccessibleInterface *parent() const override;
    QAccessibleInterface *child(int index) const override;

    // properties and state
    QString text(QAccessible::Text t) const override;
    QAccessible::Role role() const override;
    QAccessible::State state() const override;
};

#endif // QT_CONFIG(accessibility)

QT_END_NAMESPACE

#endif // QACCESSIBLEOBJECT_H
