// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSCOPEDPOINTER_H
#define QSCOPEDPOINTER_H

#include <QtCore/qglobal.h>

#include <stdlib.h>

QT_BEGIN_NAMESPACE

template <typename T>
struct QScopedPointerDeleter
{
    static inline void cleanup(T *pointer) noexcept
    {
        // Enforce a complete type.
        // If you get a compile error here, read the section on forward declared
        // classes in the QScopedPointer documentation.
        typedef char IsIncompleteType[ sizeof(T) ? 1 : -1 ];
        (void) sizeof(IsIncompleteType);

        delete pointer;
    }
    void operator()(T *pointer) const noexcept
    {
        cleanup(pointer);
    }
};

template <typename T>
struct QScopedPointerArrayDeleter
{
    static inline void cleanup(T *pointer) noexcept
    {
        // Enforce a complete type.
        // If you get a compile error here, read the section on forward declared
        // classes in the QScopedPointer documentation.
        typedef char IsIncompleteType[ sizeof(T) ? 1 : -1 ];
        (void) sizeof(IsIncompleteType);

        delete[] pointer;
    }
    void operator()(T *pointer) const noexcept
    {
        cleanup(pointer);
    }
};

struct QScopedPointerPodDeleter
{
    static inline void cleanup(void *pointer) noexcept { free(pointer); }
    void operator()(void *pointer) const noexcept { cleanup(pointer); }
};

#ifndef QT_NO_QOBJECT
template <typename T>
struct QScopedPointerObjectDeleteLater
{
    static inline void cleanup(T *pointer) { if (pointer) pointer->deleteLater(); }
    void operator()(T *pointer) const { cleanup(pointer); }
};

class QObject;
typedef QScopedPointerObjectDeleteLater<QObject> QScopedPointerDeleteLater;
#endif

template <typename T, typename Cleanup = QScopedPointerDeleter<T> >
class [[nodiscard]] QScopedPointer
{
public:
    explicit QScopedPointer(T *p = nullptr) noexcept : d(p)
    {
    }

    inline ~QScopedPointer()
    {
        T *oldD = this->d;
        Cleanup::cleanup(oldD);
    }

    inline T &operator*() const
    {
        Q_ASSERT(d);
        return *d;
    }

    T *operator->() const noexcept
    {
        return d;
    }

    bool operator!() const noexcept
    {
        return !d;
    }

    explicit operator bool() const
    {
        return !isNull();
    }

    T *data() const noexcept
    {
        return d;
    }

    T *get() const noexcept
    {
        return d;
    }

    bool isNull() const noexcept
    {
        return !d;
    }

    void reset(T *other = nullptr) noexcept(noexcept(Cleanup::cleanup(std::declval<T *>())))
    {
        if (d == other)
            return;
        T *oldD = qExchange(d, other);
        Cleanup::cleanup(oldD);
    }

#if QT_DEPRECATED_SINCE(6, 1)
    QT_DEPRECATED_VERSION_X_6_1("Use std::unique_ptr instead, and call release().")
    T *take() noexcept
    {
        T *oldD = qExchange(d, nullptr);
        return oldD;
    }
#endif

#if QT_DEPRECATED_SINCE(6, 2)
    QT_DEPRECATED_VERSION_X_6_2("Use std::unique_ptr instead of QScopedPointer.")
    void swap(QScopedPointer<T, Cleanup> &other) noexcept
    {
        qt_ptr_swap(d, other.d);
    }
#endif

    typedef T *pointer;

    friend bool operator==(const QScopedPointer<T, Cleanup> &lhs, const QScopedPointer<T, Cleanup> &rhs) noexcept
    {
        return lhs.data() == rhs.data();
    }

    friend bool operator!=(const QScopedPointer<T, Cleanup> &lhs, const QScopedPointer<T, Cleanup> &rhs) noexcept
    {
        return lhs.data() != rhs.data();
    }

    friend bool operator==(const QScopedPointer<T, Cleanup> &lhs, std::nullptr_t) noexcept
    {
        return lhs.isNull();
    }

    friend bool operator==(std::nullptr_t, const QScopedPointer<T, Cleanup> &rhs) noexcept
    {
        return rhs.isNull();
    }

    friend bool operator!=(const QScopedPointer<T, Cleanup> &lhs, std::nullptr_t) noexcept
    {
        return !lhs.isNull();
    }

    friend bool operator!=(std::nullptr_t, const QScopedPointer<T, Cleanup> &rhs) noexcept
    {
        return !rhs.isNull();
    }

#if QT_DEPRECATED_SINCE(6, 2)
    QT_DEPRECATED_VERSION_X_6_2("Use std::unique_ptr instead of QScopedPointer.")
    friend void swap(QScopedPointer<T, Cleanup> &p1, QScopedPointer<T, Cleanup> &p2) noexcept
    { p1.swap(p2); }
#endif

protected:
    T *d;

private:
    Q_DISABLE_COPY_MOVE(QScopedPointer)
};

template <typename T, typename Cleanup = QScopedPointerArrayDeleter<T> >
class [[nodiscard]] QScopedArrayPointer : public QScopedPointer<T, Cleanup>
{
    template <typename Ptr>
    using if_same_type = typename std::enable_if<std::is_same<typename std::remove_cv<T>::type, Ptr>::value, bool>::type;
public:
    inline QScopedArrayPointer() : QScopedPointer<T, Cleanup>(nullptr) {}
    inline ~QScopedArrayPointer() = default;

    template <typename D, if_same_type<D> = true>
    explicit QScopedArrayPointer(D *p)
        : QScopedPointer<T, Cleanup>(p)
    {
    }

    inline T &operator[](int i)
    {
        return this->d[i];
    }

    inline const T &operator[](int i) const
    {
        return this->d[i];
    }

#if QT_DEPRECATED_SINCE(6, 2)
    QT_DEPRECATED_VERSION_X_6_2("Use std::unique_ptr instead of QScopedArrayPointer.")
    void swap(QScopedArrayPointer &other) noexcept // prevent QScopedPointer <->QScopedArrayPointer swaps
    { QScopedPointer<T, Cleanup>::swap(other); }
#endif

private:
    explicit inline QScopedArrayPointer(void *)
    {
        // Enforce the same type.

        // If you get a compile error here, make sure you declare
        // QScopedArrayPointer with the same template type as you pass to the
        // constructor. See also the QScopedPointer documentation.

        // Storing a scalar array as a pointer to a different type is not
        // allowed and results in undefined behavior.
    }

    Q_DISABLE_COPY_MOVE(QScopedArrayPointer)
};

#if QT_DEPRECATED_SINCE(6, 2)
template <typename T, typename Cleanup>
QT_DEPRECATED_VERSION_X_6_2("Use std::unique_ptr instead of QScopedArrayPointer.")
inline void swap(QScopedArrayPointer<T, Cleanup> &lhs, QScopedArrayPointer<T, Cleanup> &rhs) noexcept
{ lhs.swap(rhs); }
#endif

QT_END_NAMESPACE

#endif // QSCOPEDPOINTER_H
