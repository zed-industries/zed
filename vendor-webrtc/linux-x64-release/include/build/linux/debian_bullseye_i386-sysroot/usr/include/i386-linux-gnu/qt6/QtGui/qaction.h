// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QACTION_H
#define QACTION_H

#include <QtGui/qtguiglobal.h>
#if QT_CONFIG(shortcut)
#  include <QtGui/qkeysequence.h>
#endif
#include <QtGui/qicon.h>
#include <QtCore/qstring.h>
#include <QtCore/qvariant.h>
#include <QtCore/qobject.h>

QT_REQUIRE_CONFIG(action);

QT_BEGIN_NAMESPACE

class QActionEvent;
class QActionGroup;
class QActionPrivate;
class QMenu;
#if QT_DEPRECATED_SINCE(6,0)
class QWidget;
class QGraphicsWidget;
#endif

class Q_GUI_EXPORT QAction : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QAction)

    Q_PROPERTY(bool checkable READ isCheckable WRITE setCheckable NOTIFY checkableChanged FINAL)
    Q_PROPERTY(bool checked READ isChecked WRITE setChecked NOTIFY toggled)
    Q_PROPERTY(bool enabled READ isEnabled WRITE setEnabled NOTIFY enabledChanged
               RESET resetEnabled FINAL)
    Q_PROPERTY(QIcon icon READ icon WRITE setIcon NOTIFY changed)
    Q_PROPERTY(QString text READ text WRITE setText NOTIFY changed)
    Q_PROPERTY(QString iconText READ iconText WRITE setIconText NOTIFY changed)
    Q_PROPERTY(QString toolTip READ toolTip WRITE setToolTip NOTIFY changed)
    Q_PROPERTY(QString statusTip READ statusTip WRITE setStatusTip NOTIFY changed)
    Q_PROPERTY(QString whatsThis READ whatsThis WRITE setWhatsThis NOTIFY changed)
    Q_PROPERTY(QFont font READ font WRITE setFont NOTIFY changed)
#if QT_CONFIG(shortcut)
    Q_PROPERTY(QKeySequence shortcut READ shortcut WRITE setShortcut NOTIFY changed)
    Q_PROPERTY(Qt::ShortcutContext shortcutContext READ shortcutContext WRITE setShortcutContext
               NOTIFY changed)
    Q_PROPERTY(bool autoRepeat READ autoRepeat WRITE setAutoRepeat NOTIFY changed)
#endif // QT_CONFIG(shortcut)
    Q_PROPERTY(bool visible READ isVisible WRITE setVisible NOTIFY visibleChanged FINAL)
    Q_PROPERTY(MenuRole menuRole READ menuRole WRITE setMenuRole NOTIFY changed)
    Q_PROPERTY(bool iconVisibleInMenu READ isIconVisibleInMenu WRITE setIconVisibleInMenu
               NOTIFY changed)
    Q_PROPERTY(bool shortcutVisibleInContextMenu READ isShortcutVisibleInContextMenu
               WRITE setShortcutVisibleInContextMenu NOTIFY changed)
    Q_PROPERTY(Priority priority READ priority WRITE setPriority NOTIFY changed)

public:
    // note this is copied into qplatformmenu.h, which must stay in sync
    enum MenuRole { NoRole = 0, TextHeuristicRole, ApplicationSpecificRole, AboutQtRole,
                    AboutRole, PreferencesRole, QuitRole };
    Q_ENUM(MenuRole)
    enum Priority { LowPriority = 0,
                    NormalPriority = 128,
                    HighPriority = 256};
    Q_ENUM(Priority)
    explicit QAction(QObject *parent = nullptr);
    explicit QAction(const QString &text, QObject *parent = nullptr);
    explicit QAction(const QIcon &icon, const QString &text, QObject *parent = nullptr);

    ~QAction();

    QList<QObject *> associatedObjects() const;

#if QT_DEPRECATED_SINCE(6,0)
#ifdef Q_CLANG_QDOC
    QWidget *parentWidget() const;
    QList<QWidget*> associatedWidgets() const;
    QList<QGraphicsWidget*> associatedGraphicsWidgets() const;
#else
    /*
        These are templates so that instantiation happens only in calling code, when
        QWidget, QMenu, and QGraphicsWidget can be expected to be fully defined.
    */
    template<typename T = QWidget*>
    QT_DEPRECATED_VERSION_X_6_0("Use parent() with qobject_cast() instead")
    T parentWidget() const
    {
        auto result = parent();
        while (result && !qobject_cast<T>(result))
            result = result->parent();
        return static_cast<T>(result);
    }

    template<typename T = QWidget*>
    QT_DEPRECATED_VERSION_X_6_0("Use associatedObjects() with qobject_cast() instead")
    QList<T> associatedWidgets() const
    {
        QList<T> result;
        for (auto object : associatedObjects())
            if (auto widget = qobject_cast<T>(object))
                result.append(widget);
        return result;
    }
    template<typename T = QGraphicsWidget*>
    QT_DEPRECATED_VERSION_X_6_0("Use associatedObjects() with qobject_cast() instead")
    QList<T> associatedGraphicsWidgets() const
    {
        QList<T> result;
        for (auto object : associatedObjects())
            if (auto graphicsWidget = qobject_cast<T>(object))
                result.append(graphicsWidget);
        return result;
    }
#endif
#endif

    void setActionGroup(QActionGroup *group);
    QActionGroup *actionGroup() const;
    void setIcon(const QIcon &icon);
    QIcon icon() const;

    void setText(const QString &text);
    QString text() const;

    void setIconText(const QString &text);
    QString iconText() const;

    void setToolTip(const QString &tip);
    QString toolTip() const;

    void setStatusTip(const QString &statusTip);
    QString statusTip() const;

    void setWhatsThis(const QString &what);
    QString whatsThis() const;

    void setPriority(Priority priority);
    Priority priority() const;

    void setSeparator(bool b);
    bool isSeparator() const;

#if QT_CONFIG(shortcut)
    void setShortcut(const QKeySequence &shortcut);
    QKeySequence shortcut() const;

    void setShortcuts(const QList<QKeySequence> &shortcuts);
    void setShortcuts(QKeySequence::StandardKey);
    QList<QKeySequence> shortcuts() const;

    void setShortcutContext(Qt::ShortcutContext context);
    Qt::ShortcutContext shortcutContext() const;

    void setAutoRepeat(bool);
    bool autoRepeat() const;
#endif // QT_CONFIG(shortcut)

    void setFont(const QFont &font);
    QFont font() const;

    void setCheckable(bool);
    bool isCheckable() const;

    QVariant data() const;
    void setData(const QVariant &var);

    bool isChecked() const;

    bool isEnabled() const;

    bool isVisible() const;

    enum ActionEvent { Trigger, Hover };
    void activate(ActionEvent event);

    void setMenuRole(MenuRole menuRole);
    MenuRole menuRole() const;

#ifdef Q_CLANG_QDOC
    QMenu *menu() const;
    void setMenu(QMenu *menu);
#else
    template<typename T = QMenu*>
    T menu() const
    {
        return qobject_cast<T>(menuObject());
    }
    template<typename T = QMenu*>
    void setMenu(T m)
    {
        setMenuObject(m);
    }
#endif

    void setIconVisibleInMenu(bool visible);
    bool isIconVisibleInMenu() const;

    void setShortcutVisibleInContextMenu(bool show);
    bool isShortcutVisibleInContextMenu() const;

    bool showStatusText(QObject *object = nullptr);

protected:
    bool event(QEvent *) override;
    QAction(QActionPrivate &dd, QObject *parent);

public Q_SLOTS:
    void trigger() { activate(Trigger); }
    void hover() { activate(Hover); }
    void setChecked(bool);
    void toggle();
    void setEnabled(bool);
    void resetEnabled();
    inline void setDisabled(bool b) { setEnabled(!b); }
    void setVisible(bool);

Q_SIGNALS:
    void changed();
    void enabledChanged(bool enabled);
    void checkableChanged(bool checkable);
    void visibleChanged();
    void triggered(bool checked = false);
    void hovered();
    void toggled(bool);

private:
    Q_DISABLE_COPY(QAction)
    friend class QActionGroup;
    friend class QWidget;
    friend class QMenu;
    friend class QMenuPrivate;
    friend class QToolButton;
    friend class QGraphicsWidget;

    QObject *menuObject() const;
    void setMenuObject(QObject *object);
};

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QAction *);
#endif

QT_END_NAMESPACE

#endif // QACTION_H
