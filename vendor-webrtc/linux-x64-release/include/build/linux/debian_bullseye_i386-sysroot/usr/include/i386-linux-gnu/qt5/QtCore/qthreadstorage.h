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

#ifndef QTHREADSTORAGE_H
#define QTHREADSTORAGE_H

#include <QtCore/qglobal.h>

#if QT_CONFIG(thread)

QT_BEGIN_NAMESPACE


class Q_CORE_EXPORT QThreadStorageData
{
public:
    explicit QThreadStorageData(void (*func)(void *));
    ~QThreadStorageData();

    void** get() const;
    void** set(void* p);

    static void finish(void**);
    int id;
};

#if !defined(QT_MOC_CPP)
// MOC_SKIP_BEGIN

// pointer specialization
template <typename T>
inline
T *&qThreadStorage_localData(QThreadStorageData &d, T **)
{
    void **v = d.get();
    if (!v) v = d.set(nullptr);
    return *(reinterpret_cast<T**>(v));
}

template <typename T>
inline
T *qThreadStorage_localData_const(const QThreadStorageData &d, T **)
{
    void **v = d.get();
    return v ? *(reinterpret_cast<T**>(v)) : 0;
}

template <typename T>
inline
void qThreadStorage_setLocalData(QThreadStorageData &d, T **t)
{ (void) d.set(*t); }

template <typename T>
inline
void qThreadStorage_deleteData(void *d, T **)
{ delete static_cast<T *>(d); }

// value-based specialization
template <typename T>
inline
T &qThreadStorage_localData(QThreadStorageData &d, T *)
{
    void **v = d.get();
    if (!v) v = d.set(new T());
    return *(reinterpret_cast<T*>(*v));
}

template <typename T>
inline
T qThreadStorage_localData_const(const QThreadStorageData &d, T *)
{
    void **v = d.get();
    return v ? *(reinterpret_cast<T*>(*v)) : T();
}

template <typename T>
inline
void qThreadStorage_setLocalData(QThreadStorageData &d, T *t)
{ (void) d.set(new T(*t)); }

template <typename T>
inline
void qThreadStorage_deleteData(void *d, T *)
{ delete static_cast<T *>(d); }


// MOC_SKIP_END
#endif

template <class T>
class QThreadStorage
{
private:
    QThreadStorageData d;

    Q_DISABLE_COPY(QThreadStorage)

    static inline void deleteData(void *x)
    { qThreadStorage_deleteData(x, reinterpret_cast<T*>(0)); }

public:
    inline QThreadStorage() : d(deleteData) { }
    inline ~QThreadStorage() { }

    inline bool hasLocalData() const
    { return d.get() != nullptr; }

    inline T& localData()
    { return qThreadStorage_localData(d, reinterpret_cast<T*>(0)); }
    inline T localData() const
    { return qThreadStorage_localData_const(d, reinterpret_cast<T*>(0)); }

    inline void setLocalData(T t)
    { qThreadStorage_setLocalData(d, &t); }
};

QT_END_NAMESPACE

#else // !QT_CONFIG(thread)

#include <QtCore/qscopedpointer.h>

#include <type_traits>

template <typename T, typename U>
inline bool qThreadStorage_hasLocalData(const QScopedPointer<T, U> &data)
{
    return !!data;
}

template <typename T, typename U>
inline bool qThreadStorage_hasLocalData(const QScopedPointer<T*, U> &data)
{
    return !!data ? *data != nullptr : false;
}

template <typename T>
inline void qThreadStorage_deleteLocalData(T *t)
{
    delete t;
}

template <typename T>
inline void qThreadStorage_deleteLocalData(T **t)
{
    delete *t;
    delete t;
}

template <class T>
class QThreadStorage
{
private:
    struct ScopedPointerThreadStorageDeleter
    {
        static inline void cleanup(T *t)
        {
            if (t == nullptr)
                return;
            qThreadStorage_deleteLocalData(t);
        }
    };
    QScopedPointer<T, ScopedPointerThreadStorageDeleter> data;

public:
    QThreadStorage() = default;
    ~QThreadStorage() = default;
    QThreadStorage(const QThreadStorage &rhs) = delete;
    QThreadStorage &operator=(const QThreadStorage &rhs) = delete;

    inline bool hasLocalData() const
    {
        return qThreadStorage_hasLocalData(data);
    }

    inline T& localData()
    {
        if (!data)
            data.reset(new T());
        return *data;
    }

    inline T localData() const
    {
        return !!data ? *data : T();
    }

    inline void setLocalData(T t)
    {
        data.reset(new T(t));
    }
};

#endif // QT_CONFIG(thread)

#endif // QTHREADSTORAGE_H
