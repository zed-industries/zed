/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#ifndef QLOGGINGCATEGORY_H
#define QLOGGINGCATEGORY_H

#include <QtCore/qglobal.h>
#include <QtCore/qdebug.h>

QT_BEGIN_NAMESPACE

class Q_CORE_EXPORT QLoggingCategory
{
    Q_DISABLE_COPY(QLoggingCategory)
public:
    // ### Qt 6: Merge constructors
    explicit QLoggingCategory(const char *category);
    QLoggingCategory(const char *category, QtMsgType severityLevel);
    ~QLoggingCategory();

    bool isEnabled(QtMsgType type) const;
    void setEnabled(QtMsgType type, bool enable);

#ifdef Q_ATOMIC_INT8_IS_SUPPORTED
    bool isDebugEnabled() const { return bools.enabledDebug.loadRelaxed(); }
    bool isInfoEnabled() const { return bools.enabledInfo.loadRelaxed(); }
    bool isWarningEnabled() const { return bools.enabledWarning.loadRelaxed(); }
    bool isCriticalEnabled() const { return bools.enabledCritical.loadRelaxed(); }
#else
    bool isDebugEnabled() const { return enabled.loadRelaxed() >> DebugShift & 1; }
    bool isInfoEnabled() const { return enabled.loadRelaxed() >> InfoShift & 1; }
    bool isWarningEnabled() const { return enabled.loadRelaxed() >> WarningShift & 1; }
    bool isCriticalEnabled() const { return enabled.loadRelaxed() >> CriticalShift & 1; }
#endif
    const char *categoryName() const { return name; }

    // allows usage of both factory method and variable in qCX macros
    QLoggingCategory &operator()() { return *this; }
    const QLoggingCategory &operator()() const { return *this; }

    static QLoggingCategory *defaultCategory();

    typedef void (*CategoryFilter)(QLoggingCategory*);
    static CategoryFilter installFilter(CategoryFilter);

    static void setFilterRules(const QString &rules);

private:
    void init(const char *category, QtMsgType severityLevel);

    Q_DECL_UNUSED_MEMBER void *d; // reserved for future use
    const char *name;

#ifdef Q_BIG_ENDIAN
    enum { DebugShift = 0, WarningShift = 8, CriticalShift = 16, InfoShift = 24 };
#else
    enum { DebugShift = 24, WarningShift = 16, CriticalShift = 8, InfoShift = 0};
#endif

    struct AtomicBools {
#ifdef Q_ATOMIC_INT8_IS_SUPPORTED
        QBasicAtomicInteger<bool> enabledDebug;
        QBasicAtomicInteger<bool> enabledWarning;
        QBasicAtomicInteger<bool> enabledCritical;
        QBasicAtomicInteger<bool> enabledInfo;
#endif
    };
    union {
        AtomicBools bools;
        QBasicAtomicInt enabled;
    };
    Q_DECL_UNUSED_MEMBER bool placeholder[4]; // reserved for future use
};

#define Q_DECLARE_LOGGING_CATEGORY(name) \
    extern const QLoggingCategory &name();

#define Q_LOGGING_CATEGORY(name, ...) \
    const QLoggingCategory &name() \
    { \
        static const QLoggingCategory category(__VA_ARGS__); \
        return category; \
    }

#if !defined(QT_NO_DEBUG_OUTPUT)
#  define qCDebug(category, ...) \
    for (bool qt_category_enabled = category().isDebugEnabled(); qt_category_enabled; qt_category_enabled = false) \
        QMessageLogger(QT_MESSAGELOG_FILE, QT_MESSAGELOG_LINE, QT_MESSAGELOG_FUNC, category().categoryName()).debug(__VA_ARGS__)
#else
#  define qCDebug(category, ...) QT_NO_QDEBUG_MACRO()
#endif

#if !defined(QT_NO_INFO_OUTPUT)
#  define qCInfo(category, ...) \
    for (bool qt_category_enabled = category().isInfoEnabled(); qt_category_enabled; qt_category_enabled = false) \
        QMessageLogger(QT_MESSAGELOG_FILE, QT_MESSAGELOG_LINE, QT_MESSAGELOG_FUNC, category().categoryName()).info(__VA_ARGS__)
#else
#  define qCInfo(category, ...) QT_NO_QDEBUG_MACRO()
#endif

#if !defined(QT_NO_WARNING_OUTPUT)
#  define qCWarning(category, ...) \
    for (bool qt_category_enabled = category().isWarningEnabled(); qt_category_enabled; qt_category_enabled = false) \
        QMessageLogger(QT_MESSAGELOG_FILE, QT_MESSAGELOG_LINE, QT_MESSAGELOG_FUNC, category().categoryName()).warning(__VA_ARGS__)
#else
#  define qCWarning(category, ...) QT_NO_QDEBUG_MACRO()
#endif

#define qCCritical(category, ...) \
    for (bool qt_category_enabled = category().isCriticalEnabled(); qt_category_enabled; qt_category_enabled = false) \
        QMessageLogger(QT_MESSAGELOG_FILE, QT_MESSAGELOG_LINE, QT_MESSAGELOG_FUNC, category().categoryName()).critical(__VA_ARGS__)

QT_END_NAMESPACE

#endif // QLOGGINGCATEGORY_H
