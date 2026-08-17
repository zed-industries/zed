// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBUTTONGROUP_H
#define QBUTTONGROUP_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qobject.h>

QT_REQUIRE_CONFIG(buttongroup);

QT_BEGIN_NAMESPACE

class QAbstractButton;
class QAbstractButtonPrivate;
class QButtonGroupPrivate;

class Q_WIDGETS_EXPORT QButtonGroup : public QObject
{
    Q_OBJECT

    Q_PROPERTY(bool exclusive READ exclusive WRITE setExclusive)
public:
    explicit QButtonGroup(QObject *parent = nullptr);
    ~QButtonGroup();

    void setExclusive(bool);
    bool exclusive() const;

    void addButton(QAbstractButton *, int id = -1);
    void removeButton(QAbstractButton *);

    QList<QAbstractButton*> buttons() const;

    QAbstractButton * checkedButton() const;
    // no setter on purpose!

    QAbstractButton *button(int id) const;
    void setId(QAbstractButton *button, int id);
    int id(QAbstractButton *button) const;
    int checkedId() const;

Q_SIGNALS:
    void buttonClicked(QAbstractButton *);
    void buttonPressed(QAbstractButton *);
    void buttonReleased(QAbstractButton *);
    void buttonToggled(QAbstractButton *, bool);
    void idClicked(int);
    void idPressed(int);
    void idReleased(int);
    void idToggled(int, bool);

private:
    Q_DISABLE_COPY(QButtonGroup)
    Q_DECLARE_PRIVATE(QButtonGroup)
    friend class QAbstractButton;
    friend class QAbstractButtonPrivate;
};

QT_END_NAMESPACE

#endif // QBUTTONGROUP_H
