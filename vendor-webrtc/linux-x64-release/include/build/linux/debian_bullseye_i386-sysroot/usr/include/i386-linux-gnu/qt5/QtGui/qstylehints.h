/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QSTYLEHINTS_H
#define QSTYLEHINTS_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qobject.h>

QT_BEGIN_NAMESPACE


class QPlatformIntegration;
class QStyleHintsPrivate;

class Q_GUI_EXPORT QStyleHints : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QStyleHints)
    Q_PROPERTY(int cursorFlashTime READ cursorFlashTime NOTIFY cursorFlashTimeChanged FINAL)
    Q_PROPERTY(qreal fontSmoothingGamma READ fontSmoothingGamma STORED false CONSTANT FINAL)
    Q_PROPERTY(int keyboardAutoRepeatRate READ keyboardAutoRepeatRate STORED false CONSTANT FINAL)
    Q_PROPERTY(int keyboardInputInterval READ keyboardInputInterval NOTIFY keyboardInputIntervalChanged FINAL)
    Q_PROPERTY(int mouseDoubleClickInterval READ mouseDoubleClickInterval NOTIFY mouseDoubleClickIntervalChanged FINAL)
    Q_PROPERTY(int mousePressAndHoldInterval READ mousePressAndHoldInterval NOTIFY mousePressAndHoldIntervalChanged FINAL)
    Q_PROPERTY(QChar passwordMaskCharacter READ passwordMaskCharacter STORED false CONSTANT FINAL)
    Q_PROPERTY(int passwordMaskDelay READ passwordMaskDelay STORED false CONSTANT FINAL)
    Q_PROPERTY(bool setFocusOnTouchRelease READ setFocusOnTouchRelease STORED false CONSTANT FINAL)
    Q_PROPERTY(bool showIsFullScreen READ showIsFullScreen STORED false CONSTANT FINAL)
    Q_PROPERTY(bool showIsMaximized READ showIsMaximized STORED false CONSTANT FINAL)
    Q_PROPERTY(bool showShortcutsInContextMenus READ showShortcutsInContextMenus WRITE setShowShortcutsInContextMenus NOTIFY showShortcutsInContextMenusChanged FINAL)
    Q_PROPERTY(int startDragDistance READ startDragDistance NOTIFY startDragDistanceChanged FINAL)
    Q_PROPERTY(int startDragTime READ startDragTime NOTIFY startDragTimeChanged FINAL)
    Q_PROPERTY(int startDragVelocity READ startDragVelocity STORED false CONSTANT FINAL)
    Q_PROPERTY(bool useRtlExtensions READ useRtlExtensions STORED false CONSTANT FINAL)
    Q_PROPERTY(Qt::TabFocusBehavior tabFocusBehavior READ tabFocusBehavior NOTIFY tabFocusBehaviorChanged FINAL)
    Q_PROPERTY(bool singleClickActivation READ singleClickActivation STORED false CONSTANT FINAL)
    Q_PROPERTY(bool useHoverEffects READ useHoverEffects WRITE setUseHoverEffects NOTIFY useHoverEffectsChanged FINAL)
    Q_PROPERTY(int wheelScrollLines READ wheelScrollLines NOTIFY wheelScrollLinesChanged FINAL)
    Q_PROPERTY(int mouseQuickSelectionThreshold READ mouseQuickSelectionThreshold WRITE setMouseQuickSelectionThreshold NOTIFY mouseQuickSelectionThresholdChanged FINAL)
    Q_PROPERTY(int mouseDoubleClickDistance READ mouseDoubleClickDistance STORED false CONSTANT FINAL)
    Q_PROPERTY(int touchDoubleTapDistance READ touchDoubleTapDistance STORED false CONSTANT FINAL)

public:
    void setMouseDoubleClickInterval(int mouseDoubleClickInterval);
    int mouseDoubleClickInterval() const;
    int mouseDoubleClickDistance() const;
    int touchDoubleTapDistance() const;
    void setMousePressAndHoldInterval(int mousePressAndHoldInterval);
    int mousePressAndHoldInterval() const;
    void setStartDragDistance(int startDragDistance);
    int startDragDistance() const;
    void setStartDragTime(int startDragTime);
    int startDragTime() const;
    int startDragVelocity() const;
    void setKeyboardInputInterval(int keyboardInputInterval);
    int keyboardInputInterval() const;
    int keyboardAutoRepeatRate() const;
    void setCursorFlashTime(int cursorFlashTime);
    int cursorFlashTime() const;
    bool showIsFullScreen() const;
    bool showIsMaximized() const;
    bool showShortcutsInContextMenus() const;
    void setShowShortcutsInContextMenus(bool showShortcutsInContextMenus);
    int passwordMaskDelay() const;
    QChar passwordMaskCharacter() const;
    qreal fontSmoothingGamma() const;
    bool useRtlExtensions() const;
    bool setFocusOnTouchRelease() const;
    Qt::TabFocusBehavior tabFocusBehavior() const;
    void setTabFocusBehavior(Qt::TabFocusBehavior tabFocusBehavior);
    bool singleClickActivation() const;
    bool useHoverEffects() const;
    void setUseHoverEffects(bool useHoverEffects);
    int wheelScrollLines() const;
    void setWheelScrollLines(int scrollLines);
    void setMouseQuickSelectionThreshold(int threshold);
    int mouseQuickSelectionThreshold() const;

Q_SIGNALS:
    void cursorFlashTimeChanged(int cursorFlashTime);
    void keyboardInputIntervalChanged(int keyboardInputInterval);
    void mouseDoubleClickIntervalChanged(int mouseDoubleClickInterval);
    void mousePressAndHoldIntervalChanged(int mousePressAndHoldInterval);
    void startDragDistanceChanged(int startDragDistance);
    void startDragTimeChanged(int startDragTime);
    void tabFocusBehaviorChanged(Qt::TabFocusBehavior tabFocusBehavior);
    void useHoverEffectsChanged(bool useHoverEffects);
    void showShortcutsInContextMenusChanged(bool);
    void wheelScrollLinesChanged(int scrollLines);
    void mouseQuickSelectionThresholdChanged(int threshold);

private:
    friend class QGuiApplication;
    QStyleHints();
};

QT_END_NAMESPACE

#endif
