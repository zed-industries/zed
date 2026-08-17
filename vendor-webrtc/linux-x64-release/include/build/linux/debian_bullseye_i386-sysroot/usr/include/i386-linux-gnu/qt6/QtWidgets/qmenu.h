// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QMENU_H
#define QMENU_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qwidget.h>
#include <QtCore/qstring.h>
#include <QtGui/qicon.h>
#include <QtGui/qaction.h>

#if defined(Q_OS_MACOS) || defined(Q_CLANG_QDOC)
Q_FORWARD_DECLARE_OBJC_CLASS(NSMenu);
#endif

QT_REQUIRE_CONFIG(menu);

QT_BEGIN_NAMESPACE

class QMenuPrivate;
class QStyleOptionMenuItem;
class QPlatformMenu;

class Q_WIDGETS_EXPORT QMenu : public QWidget
{
private:
    Q_OBJECT
    Q_DECLARE_PRIVATE(QMenu)

    Q_PROPERTY(bool tearOffEnabled READ isTearOffEnabled WRITE setTearOffEnabled)
    Q_PROPERTY(QString title READ title WRITE setTitle)
    Q_PROPERTY(QIcon icon READ icon WRITE setIcon)
    Q_PROPERTY(bool separatorsCollapsible READ separatorsCollapsible WRITE setSeparatorsCollapsible)
    Q_PROPERTY(bool toolTipsVisible READ toolTipsVisible WRITE setToolTipsVisible)

public:
    explicit QMenu(QWidget *parent = nullptr);
    explicit QMenu(const QString &title, QWidget *parent = nullptr);
    ~QMenu();

    using QWidget::addAction;
#if QT_WIDGETS_REMOVED_SINCE(6, 3)
    QAction *addAction(const QString &text);
    QAction *addAction(const QIcon &icon, const QString &text);
#if !QT_CONFIG(shortcut)
    QAction *addAction(const QString &text, const QObject *receiver, const char* member);
    QAction *addAction(const QIcon &icon, const QString &text,
                       const QObject *receiver, const char* member);
#endif
#endif

#if QT_CONFIG(shortcut)
#if QT_DEPRECATED_SINCE(6, 4)
    QT_DEPRECATED_VERSION_X_6_4("Use addAction(text, shortcut, receiver, member) instead.")
    QAction *addAction(const QString &text, const QObject *receiver, const char* member,
                       const QKeySequence &shortcut);
    QT_DEPRECATED_VERSION_X_6_4("Use addAction(icon, text, shortcut, receiver, member) instead.")
    QAction *addAction(const QIcon &icon, const QString &text,
                       const QObject *receiver, const char* member,
                       const QKeySequence &shortcut);

#ifdef Q_CLANG_QDOC
    template<typename Functor>
    QAction *addAction(const QString &text, Functor functor, const QKeySequence &shortcut);
    template<typename Functor>
    QAction *addAction(const QString &text, const QObject *context, Functor functor, const QKeySequence &shortcut);
    template<typename Functor>
    QAction *addAction(const QIcon &icon, const QString &text, Functor functor, const QKeySequence &shortcut);
    template<typename Functor>
    QAction *addAction(const QIcon &icon, const QString &text, const QObject *context, Functor functor, const QKeySequence &shortcut);
#else
    // addAction(QString): Connect to a QObject slot / functor or function pointer (with context)
    template<class Obj, typename Func1>
    QT_DEPRECATED_VERSION_X_6_4("Use addAction(text, shortcut, object, slot) instead.")
    inline typename std::enable_if<!std::is_same<const char*, Func1>::value
        && QtPrivate::IsPointerToTypeDerivedFromQObject<Obj*>::Value, QAction *>::type
        addAction(const QString &text, const Obj *object, Func1 slot,
                  const QKeySequence &shortcut)
    {
        return addAction(text, shortcut, object, slot);
    }
    // addAction(QString): Connect to a functor or function pointer (without context)
    template <typename Func1>
    QT_DEPRECATED_VERSION_X_6_4("Use addAction(text, shortcut, slot) instead.")
    inline QAction *addAction(const QString &text, Func1 slot, const QKeySequence &shortcut)
    {
        return addAction(text, shortcut, slot);
    }
    // addAction(QIcon, QString): Connect to a QObject slot / functor or function pointer (with context)
    template<class Obj, typename Func1>
    QT_DEPRECATED_VERSION_X_6_4("Use addAction(icon, text, shortcut, object, slot) instead.")
    inline typename std::enable_if<!std::is_same<const char*, Func1>::value
        && QtPrivate::IsPointerToTypeDerivedFromQObject<Obj*>::Value, QAction *>::type
        addAction(const QIcon &actionIcon, const QString &text, const Obj *object, Func1 slot,
                  const QKeySequence &shortcut)

    {
        return addAction(actionIcon, text, shortcut, object, slot);
    }
    // addAction(QIcon, QString): Connect to a functor or function pointer (without context)
    template <typename Func1>
    QT_DEPRECATED_VERSION_X_6_4("Use addAction(icon, text, shortcut, slot) instead.")
    inline QAction *addAction(const QIcon &actionIcon, const QString &text, Func1 slot,
                              const QKeySequence &shortcut)
    {
        return addAction(actionIcon, text, shortcut, slot);
    }
#endif // !Q_CLANG_QDOC
#endif // QT_DEPRECATED_SINCE(6, 4)
#endif // QT_CONFIG(shortcut)

    QAction *addMenu(QMenu *menu);
    QMenu *addMenu(const QString &title);
    QMenu *addMenu(const QIcon &icon, const QString &title);

    QAction *addSeparator();

    QAction *addSection(const QString &text);
    QAction *addSection(const QIcon &icon, const QString &text);

    QAction *insertMenu(QAction *before, QMenu *menu);
    QAction *insertSeparator(QAction *before);
    QAction *insertSection(QAction *before, const QString &text);
    QAction *insertSection(QAction *before, const QIcon &icon, const QString &text);

    bool isEmpty() const;
    void clear();

    void setTearOffEnabled(bool);
    bool isTearOffEnabled() const;

    bool isTearOffMenuVisible() const;
    void showTearOffMenu();
    void showTearOffMenu(const QPoint &pos);
    void hideTearOffMenu();

    void setDefaultAction(QAction *);
    QAction *defaultAction() const;

    void setActiveAction(QAction *act);
    QAction *activeAction() const;

    void popup(const QPoint &pos, QAction *at = nullptr);
    QAction *exec();
    QAction *exec(const QPoint &pos, QAction *at = nullptr);

    static QAction *exec(const QList<QAction *> &actions, const QPoint &pos, QAction *at = nullptr, QWidget *parent = nullptr);

    QSize sizeHint() const override;

    QRect actionGeometry(QAction *) const;
    QAction *actionAt(const QPoint &) const;

    QAction *menuAction() const;
    static QMenu *menuInAction(const QAction *action)
    { return qobject_cast<QMenu *>(action->menuObject()); }

    QString title() const;
    void setTitle(const QString &title);

    QIcon icon() const;
    void setIcon(const QIcon &icon);

    void setNoReplayFor(QWidget *widget);
    QPlatformMenu *platformMenu();
    void setPlatformMenu(QPlatformMenu *platformMenu);

#if defined(Q_OS_MACOS) || defined(Q_CLANG_QDOC)
    NSMenu* toNSMenu();
    void setAsDockMenu();
#endif

    bool separatorsCollapsible() const;
    void setSeparatorsCollapsible(bool collapse);

    bool toolTipsVisible() const;
    void setToolTipsVisible(bool visible);

Q_SIGNALS:
    void aboutToShow();
    void aboutToHide();
    void triggered(QAction *action);
    void hovered(QAction *action);

protected:
    int columnCount() const;

    void changeEvent(QEvent *) override;
    void keyPressEvent(QKeyEvent *) override;
    void mouseReleaseEvent(QMouseEvent *) override;
    void mousePressEvent(QMouseEvent *) override;
    void mouseMoveEvent(QMouseEvent *) override;
#if QT_CONFIG(wheelevent)
    void wheelEvent(QWheelEvent *) override;
#endif
    void enterEvent(QEnterEvent *) override;
    void leaveEvent(QEvent *) override;
    void hideEvent(QHideEvent *) override;
    void paintEvent(QPaintEvent *) override;
    void actionEvent(QActionEvent *) override;
    void timerEvent(QTimerEvent *) override;
    bool event(QEvent *) override;
    bool focusNextPrevChild(bool next) override;
    virtual void initStyleOption(QStyleOptionMenuItem *option, const QAction *action) const;

private Q_SLOTS:
    void internalDelayedPopup();

private:
    Q_PRIVATE_SLOT(d_func(), void _q_actionTriggered())
    Q_PRIVATE_SLOT(d_func(), void _q_actionHovered())
    Q_PRIVATE_SLOT(d_func(), void _q_overrideMenuActionDestroyed())
    Q_PRIVATE_SLOT(d_func(), void _q_platformMenuAboutToShow())

protected:
    QMenu(QMenuPrivate &dd, QWidget* parent = nullptr);

private:
    Q_DISABLE_COPY(QMenu)

    friend class QMenuBar;
    friend class QMenuBarPrivate;
    friend class QTornOffMenu;
    friend class QComboBox;
    friend class QtWidgetsActionPrivate;
    friend class QToolButtonPrivate;
    friend void qt_mac_emit_menuSignals(QMenu *menu, bool show);
    friend void qt_mac_menu_emit_hovered(QMenu *menu, QAction *action);
};

QT_END_NAMESPACE

#endif // QMENU_H
