/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtWidgets module of the Qt Toolkit.
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

#ifndef QUNDOVIEW_H
#define QUNDOVIEW_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qlistview.h>
#include <QtCore/qstring.h>

QT_REQUIRE_CONFIG(undoview);

QT_BEGIN_NAMESPACE

class QUndoViewPrivate;
class QUndoStack;
class QUndoGroup;
class QIcon;


class Q_WIDGETS_EXPORT QUndoView : public QListView
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QUndoView)
    Q_PROPERTY(QString emptyLabel READ emptyLabel WRITE setEmptyLabel)
    Q_PROPERTY(QIcon cleanIcon READ cleanIcon WRITE setCleanIcon)

public:
    explicit QUndoView(QWidget *parent = nullptr);
    explicit QUndoView(QUndoStack *stack, QWidget *parent = nullptr);
#if QT_CONFIG(undogroup)
    explicit QUndoView(QUndoGroup *group, QWidget *parent = nullptr);
#endif
    ~QUndoView();

    QUndoStack *stack() const;
#if QT_CONFIG(undogroup)
    QUndoGroup *group() const;
#endif

    void setEmptyLabel(const QString &label);
    QString emptyLabel() const;

    void setCleanIcon(const QIcon &icon);
    QIcon cleanIcon() const;

public Q_SLOTS:
    void setStack(QUndoStack *stack);
#if QT_CONFIG(undogroup)
    void setGroup(QUndoGroup *group);
#endif

private:
    Q_DISABLE_COPY(QUndoView)
};

QT_END_NAMESPACE

#endif // QUNDOVIEW_H
