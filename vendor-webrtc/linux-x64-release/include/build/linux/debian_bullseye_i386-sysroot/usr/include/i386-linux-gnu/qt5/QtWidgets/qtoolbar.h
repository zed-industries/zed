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

#ifndef QDYNAMICTOOLBAR_H
#define QDYNAMICTOOLBAR_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qaction.h>
#include <QtWidgets/qwidget.h>

QT_REQUIRE_CONFIG(toolbar);

QT_BEGIN_NAMESPACE

class QToolBarPrivate;

class QAction;
class QIcon;
class QMainWindow;
class QStyleOptionToolBar;

class Q_WIDGETS_EXPORT QToolBar : public QWidget
{
    Q_OBJECT

    Q_PROPERTY(bool movable READ isMovable WRITE setMovable NOTIFY movableChanged)
    Q_PROPERTY(Qt::ToolBarAreas allowedAreas READ allowedAreas WRITE setAllowedAreas NOTIFY allowedAreasChanged)
    Q_PROPERTY(Qt::Orientation orientation READ orientation WRITE setOrientation NOTIFY orientationChanged)
    Q_PROPERTY(QSize iconSize READ iconSize WRITE setIconSize NOTIFY iconSizeChanged)
    Q_PROPERTY(Qt::ToolButtonStyle toolButtonStyle READ toolButtonStyle WRITE setToolButtonStyle
               NOTIFY toolButtonStyleChanged)
    Q_PROPERTY(bool floating READ isFloating)
    Q_PROPERTY(bool floatable READ isFloatable WRITE setFloatable)

public:
    explicit QToolBar(const QString &title, QWidget *parent = nullptr);
    explicit QToolBar(QWidget *parent = nullptr);
    ~QToolBar();

    void setMovable(bool movable);
    bool isMovable() const;

    void setAllowedAreas(Qt::ToolBarAreas areas);
    Qt::ToolBarAreas allowedAreas() const;

    inline bool isAreaAllowed(Qt::ToolBarArea area) const
    { return (allowedAreas() & area) == area; }

    void setOrientation(Qt::Orientation orientation);
    Qt::Orientation orientation() const;

    void clear();

    using QWidget::addAction;
    QAction *addAction(const QString &text);
    QAction *addAction(const QIcon &icon, const QString &text);
    QAction *addAction(const QString &text, const QObject *receiver, const char* member);
    QAction *addAction(const QIcon &icon, const QString &text,
                       const QObject *receiver, const char* member);
#ifdef Q_CLANG_QDOC
    template<typename Functor>
    QAction *addAction(const QString &text, Functor functor);
    template<typename Functor>
    QAction *addAction(const QString &text, const QObject *context, Functor functor);
    template<typename Functor>
    QAction *addAction(const QIcon &icon, const QString &text, Functor functor);
    template<typename Functor>
    QAction *addAction(const QIcon &icon, const QString &text, const QObject *context, Functor functor);
#else
    // addAction(QString): Connect to a QObject slot / functor or function pointer (with context)
    template<class Obj, typename Func1>
    inline typename std::enable_if<!std::is_same<const char*, Func1>::value
        && QtPrivate::IsPointerToTypeDerivedFromQObject<Obj*>::Value, QAction *>::type
        addAction(const QString &text, const Obj *object, Func1 slot)
    {
        QAction *result = addAction(text);
        connect(result, &QAction::triggered, object, std::move(slot));
        return result;
    }
    // addAction(QString): Connect to a functor or function pointer (without context)
    template <typename Func1>
    inline QAction *addAction(const QString &text, Func1 slot)
    {
        QAction *result = addAction(text);
        connect(result, &QAction::triggered, slot);
        return result;
    }
    // addAction(QString): Connect to a QObject slot / functor or function pointer (with context)
    template<class Obj, typename Func1>
    inline typename std::enable_if<!std::is_same<const char*, Func1>::value
        && QtPrivate::IsPointerToTypeDerivedFromQObject<Obj*>::Value, QAction *>::type
        addAction(const QIcon &actionIcon, const QString &text, const Obj *object, Func1 slot)
    {
        QAction *result = addAction(actionIcon, text);
        connect(result, &QAction::triggered, object, std::move(slot));
        return result;
    }
    // addAction(QIcon, QString): Connect to a functor or function pointer (without context)
    template <typename Func1>
    inline QAction *addAction(const QIcon &actionIcon, const QString &text, Func1 slot)
    {
        QAction *result = addAction(actionIcon, text);
        connect(result, &QAction::triggered, slot);
        return result;
    }
#endif // !Q_CLANG_QDOC

    QAction *addSeparator();
    QAction *insertSeparator(QAction *before);

    QAction *addWidget(QWidget *widget);
    QAction *insertWidget(QAction *before, QWidget *widget);

    QRect actionGeometry(QAction *action) const;
    QAction *actionAt(const QPoint &p) const;
    inline QAction *actionAt(int x, int y) const;

    QAction *toggleViewAction() const;

    QSize iconSize() const;
    Qt::ToolButtonStyle toolButtonStyle() const;

    QWidget *widgetForAction(QAction *action) const;

    bool isFloatable() const;
    void setFloatable(bool floatable);
    bool isFloating() const;

public Q_SLOTS:
    void setIconSize(const QSize &iconSize);
    void setToolButtonStyle(Qt::ToolButtonStyle toolButtonStyle);

Q_SIGNALS:
    void actionTriggered(QAction *action);
    void movableChanged(bool movable);
    void allowedAreasChanged(Qt::ToolBarAreas allowedAreas);
    void orientationChanged(Qt::Orientation orientation);
    void iconSizeChanged(const QSize &iconSize);
    void toolButtonStyleChanged(Qt::ToolButtonStyle toolButtonStyle);
    void topLevelChanged(bool topLevel);
    void visibilityChanged(bool visible);

protected:
    void actionEvent(QActionEvent *event) override;
    void changeEvent(QEvent *event) override;
    void paintEvent(QPaintEvent *event) override;
    bool event(QEvent *event) override;
    void initStyleOption(QStyleOptionToolBar *option) const;


private:
    Q_DECLARE_PRIVATE(QToolBar)
    Q_DISABLE_COPY(QToolBar)
    Q_PRIVATE_SLOT(d_func(), void _q_toggleView(bool))
    Q_PRIVATE_SLOT(d_func(), void _q_updateIconSize(const QSize &))
    Q_PRIVATE_SLOT(d_func(), void _q_updateToolButtonStyle(Qt::ToolButtonStyle))

    friend class QMainWindow;
    friend class QMainWindowLayout;
    friend class QToolBarLayout;
    friend class QToolBarAreaLayout;
};

inline QAction *QToolBar::actionAt(int ax, int ay) const
{ return actionAt(QPoint(ax, ay)); }

QT_END_NAMESPACE

#endif // QDYNAMICTOOLBAR_H
