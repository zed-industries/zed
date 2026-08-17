// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QWHATSTHIS_H
#define QWHATSTHIS_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qobject.h>
#include <QtGui/qcursor.h>

QT_REQUIRE_CONFIG(whatsthis);

QT_BEGIN_NAMESPACE

#if QT_CONFIG(action)
class QAction;
#endif // QT_CONFIG(action)

class Q_WIDGETS_EXPORT QWhatsThis
{
    QWhatsThis() = delete;

public:
    static void enterWhatsThisMode();
    static bool inWhatsThisMode();
    static void leaveWhatsThisMode();

    static void showText(const QPoint &pos, const QString &text, QWidget *w = nullptr);
    static void hideText();

#if QT_CONFIG(action)
    static QAction *createAction(QObject *parent = nullptr);
#endif // QT_CONFIG(action)

};

QT_END_NAMESPACE

#endif // QWHATSTHIS_H
