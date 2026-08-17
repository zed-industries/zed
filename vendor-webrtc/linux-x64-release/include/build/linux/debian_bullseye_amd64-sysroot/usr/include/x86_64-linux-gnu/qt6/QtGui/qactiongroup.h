// Copyright (C) 2019 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QACTIONGROUP_H
#define QACTIONGROUP_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qaction.h>

QT_REQUIRE_CONFIG(action);

QT_BEGIN_NAMESPACE

class QActionGroupPrivate;

class Q_GUI_EXPORT QActionGroup : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QActionGroup)

    Q_PROPERTY(QActionGroup::ExclusionPolicy exclusionPolicy READ exclusionPolicy WRITE setExclusionPolicy)
    Q_PROPERTY(bool enabled READ isEnabled WRITE setEnabled)
    Q_PROPERTY(bool visible READ isVisible WRITE setVisible)

public:
    enum class ExclusionPolicy {
        None,
        Exclusive,
        ExclusiveOptional
    };
    Q_ENUM(ExclusionPolicy)

    explicit QActionGroup(QObject *parent);
    ~QActionGroup();

    QAction *addAction(QAction *a);
    QAction *addAction(const QString &text);
    QAction *addAction(const QIcon &icon, const QString &text);
    void removeAction(QAction *a);
    QList<QAction*> actions() const;
    QAction *checkedAction() const;

    bool isExclusive() const;
    bool isEnabled() const;
    bool isVisible() const;
    ExclusionPolicy exclusionPolicy() const;


public Q_SLOTS:
    void setEnabled(bool);
    inline void setDisabled(bool b) { setEnabled(!b); }
    void setVisible(bool);
    void setExclusive(bool);
    void setExclusionPolicy(ExclusionPolicy policy);

Q_SIGNALS:
    void triggered(QAction *);
    void hovered(QAction *);

private Q_SLOTS:
    void _q_actionTriggered();
    void _q_actionHovered();
    void _q_actionChanged();

protected:
     QActionGroup(QActionGroupPrivate &dd, QObject *parent);

private:
    Q_DISABLE_COPY(QActionGroup)
};

QT_END_NAMESPACE

#endif // QACTIONGROUP_H
