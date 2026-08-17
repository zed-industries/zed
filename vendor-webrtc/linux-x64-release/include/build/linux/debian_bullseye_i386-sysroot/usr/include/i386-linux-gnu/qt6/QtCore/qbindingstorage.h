// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBINDINGSTORAGE_H
#define QBINDINGSTORAGE_H

#include <QtCore/qglobal.h>
#include <QtCore/qnamespace.h>

QT_BEGIN_NAMESPACE

template <typename Class, typename T, auto Offset, auto Setter, auto Signal, auto Getter>
class QObjectCompatProperty;
struct QPropertyDelayedNotifications;
class QUntypedPropertyData;

namespace QtPrivate {

class QPropertyBindingData;
struct BindingEvaluationState;
struct CompatPropertySafePoint;
}

struct QBindingStatus
{
    QtPrivate::BindingEvaluationState *currentlyEvaluatingBinding = nullptr;
    QtPrivate::CompatPropertySafePoint *currentCompatProperty = nullptr;
    Qt::HANDLE threadId = nullptr;
    QPropertyDelayedNotifications *groupUpdateData = nullptr;
};

namespace QtPrivate {
struct QBindingStatusAccessToken;
Q_AUTOTEST_EXPORT QBindingStatus *getBindingStatus(QBindingStatusAccessToken);
}


struct QBindingStorageData;
class Q_CORE_EXPORT QBindingStorage
{
    mutable QBindingStorageData *d = nullptr;
    QBindingStatus *bindingStatus = nullptr;

    template<typename Class, typename T, auto Offset, auto Setter, auto Signal, auto Getter>
    friend class QObjectCompatProperty;
    friend class QObjectPrivate;
    friend class QtPrivate::QPropertyBindingData;
public:
    QBindingStorage();
    ~QBindingStorage();

    bool isEmpty() { return !d; }
    bool isValid() const noexcept { return bindingStatus; }

    const QBindingStatus *status(QtPrivate::QBindingStatusAccessToken) const;

    void registerDependency(const QUntypedPropertyData *data) const
    {
        if (!bindingStatus || !bindingStatus->currentlyEvaluatingBinding)
            return;
        registerDependency_helper(data);
    }
    QtPrivate::QPropertyBindingData *bindingData(const QUntypedPropertyData *data) const
    {
        if (!d)
            return nullptr;
        return bindingData_helper(data);
    }

#if QT_CORE_REMOVED_SINCE(6, 2)
    void maybeUpdateBindingAndRegister(const QUntypedPropertyData *data) const { registerDependency(data); }
#endif

    QtPrivate::QPropertyBindingData *bindingData(QUntypedPropertyData *data, bool create)
    {
        if (!d && !create)
            return nullptr;
        return bindingData_helper(data, create);
    }
private:
    void reinitAfterThreadMove();
    void clear();
    void registerDependency_helper(const QUntypedPropertyData *data) const;
#if QT_CORE_REMOVED_SINCE(6, 2)
    // ### Unused, but keep for BC
    void maybeUpdateBindingAndRegister_helper(const QUntypedPropertyData *data) const;
#endif
    QtPrivate::QPropertyBindingData *bindingData_helper(const QUntypedPropertyData *data) const;
    QtPrivate::QPropertyBindingData *bindingData_helper(QUntypedPropertyData *data, bool create);
};

QT_END_NAMESPACE

#endif // QBINDINGSTORAGE_H
