// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSHAREDDATA_H
#define QSHAREDDATA_H

#include <QtCore/qglobal.h>
#include <QtCore/qatomic.h>
#include <QtCore/qhashfunctions.h>

#include <functional>

QT_BEGIN_NAMESPACE


template <class T> class QSharedDataPointer;

class QSharedData
{
public:
    mutable QAtomicInt ref;

    QSharedData() noexcept : ref(0) { }
    QSharedData(const QSharedData &) noexcept : ref(0) { }

    // using the assignment operator would lead to corruption in the ref-counting
    QSharedData &operator=(const QSharedData &) = delete;
    ~QSharedData() = default;
};

struct QAdoptSharedDataTag { explicit constexpr QAdoptSharedDataTag() = default; };

template <typename T>
class QSharedDataPointer
{
public:
    typedef T Type;
    typedef T *pointer;

    void detach() { if (d && d->ref.loadRelaxed() != 1) detach_helper(); }
    T &operator*() { detach(); return *d; }
    const T &operator*() const { return *d; }
    T *operator->() { detach(); return d; }
    const T *operator->() const noexcept { return d; }
    operator T *() { detach(); return d; }
    operator const T *() const noexcept { return d; }
    T *data() { detach(); return d; }
    T *get() { detach(); return d; }
    const T *data() const noexcept { return d; }
    const T *get() const noexcept { return d; }
    const T *constData() const noexcept { return d; }
    T *take() noexcept { return qExchange(d, nullptr); }

    QSharedDataPointer() noexcept : d(nullptr) { }
    ~QSharedDataPointer() { if (d && !d->ref.deref()) delete d; }

    explicit QSharedDataPointer(T *data) noexcept : d(data)
    { if (d) d->ref.ref(); }
    QSharedDataPointer(T *data, QAdoptSharedDataTag) noexcept : d(data)
    {}
    QSharedDataPointer(const QSharedDataPointer &o) noexcept : d(o.d)
    { if (d) d->ref.ref(); }

    void reset(T *ptr = nullptr) noexcept
    {
        if (ptr != d) {
            if (ptr)
                ptr->ref.ref();
            T *old = qExchange(d, ptr);
            if (old && !old->ref.deref())
                delete old;
        }
    }

    QSharedDataPointer &operator=(const QSharedDataPointer &o) noexcept
    {
        reset(o.d);
        return *this;
    }
    inline QSharedDataPointer &operator=(T *o) noexcept
    {
        reset(o);
        return *this;
    }
    QSharedDataPointer(QSharedDataPointer &&o) noexcept : d(qExchange(o.d, nullptr)) {}
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QSharedDataPointer)

    operator bool () const noexcept { return d != nullptr; }
    bool operator!() const noexcept { return d == nullptr; }

    void swap(QSharedDataPointer &other) noexcept
    { qt_ptr_swap(d, other.d); }

#define DECLARE_COMPARE_SET(T1, A1, T2, A2) \
    friend bool operator<(T1, T2) noexcept \
    { return std::less<T*>{}(A1, A2); } \
    friend bool operator<=(T1, T2) noexcept \
    { return !std::less<T*>{}(A2, A1); } \
    friend bool operator>(T1, T2) noexcept \
    { return std::less<T*>{}(A2, A1); } \
    friend bool operator>=(T1, T2) noexcept \
    { return !std::less<T*>{}(A1, A2); } \
    friend bool operator==(T1, T2) noexcept \
    { return A1 == A2; } \
    friend bool operator!=(T1, T2) noexcept \
    { return A1 != A2; } \

    DECLARE_COMPARE_SET(const QSharedDataPointer &p1, p1.d, const QSharedDataPointer &p2, p2.d)
    DECLARE_COMPARE_SET(const QSharedDataPointer &p1, p1.d, const T *ptr, ptr)
    DECLARE_COMPARE_SET(const T *ptr, ptr, const QSharedDataPointer &p2, p2.d)
    DECLARE_COMPARE_SET(const QSharedDataPointer &p1, p1.d, std::nullptr_t, nullptr)
    DECLARE_COMPARE_SET(std::nullptr_t, nullptr, const QSharedDataPointer &p2, p2.d)

protected:
    T *clone();

private:
    void detach_helper();

    T *d;
};

template <typename T>
class QExplicitlySharedDataPointer
{
public:
    typedef T Type;
    typedef T *pointer;

    T &operator*() const { return *d; }
    T *operator->() noexcept { return d; }
    T *operator->() const noexcept { return d; }
    explicit operator T *() { return d; }
    explicit operator const T *() const noexcept { return d; }
    T *data() const noexcept { return d; }
    T *get() const noexcept { return d; }
    const T *constData() const noexcept { return d; }
    T *take() noexcept { return qExchange(d, nullptr); }

    void detach() { if (d && d->ref.loadRelaxed() != 1) detach_helper(); }

    QExplicitlySharedDataPointer() noexcept : d(nullptr) { }
    ~QExplicitlySharedDataPointer() { if (d && !d->ref.deref()) delete d; }

    explicit QExplicitlySharedDataPointer(T *data) noexcept : d(data)
    { if (d) d->ref.ref(); }
    QExplicitlySharedDataPointer(T *data, QAdoptSharedDataTag) noexcept : d(data)
    {}
    QExplicitlySharedDataPointer(const QExplicitlySharedDataPointer &o) noexcept : d(o.d)
    { if (d) d->ref.ref(); }

    template<typename X>
    QExplicitlySharedDataPointer(const QExplicitlySharedDataPointer<X> &o) noexcept
#ifdef QT_ENABLE_QEXPLICITLYSHAREDDATAPOINTER_STATICCAST
        : d(static_cast<T *>(o.data()))
#else
        : d(o.data())
#endif
    { if (d) d->ref.ref(); }

    void reset(T *ptr = nullptr) noexcept
    {
        if (ptr != d) {
            if (ptr)
                ptr->ref.ref();
            T *old = qExchange(d, ptr);
            if (old && !old->ref.deref())
                delete old;
        }
    }

    QExplicitlySharedDataPointer &operator=(const QExplicitlySharedDataPointer &o) noexcept
    {
        reset(o.d);
        return *this;
    }
    QExplicitlySharedDataPointer &operator=(T *o) noexcept
    {
        reset(o);
        return *this;
    }
    QExplicitlySharedDataPointer(QExplicitlySharedDataPointer &&o) noexcept : d(qExchange(o.d, nullptr)) {}
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QExplicitlySharedDataPointer)

    operator bool () const noexcept { return d != nullptr; }
    bool operator!() const noexcept { return d == nullptr; }

    void swap(QExplicitlySharedDataPointer &other) noexcept
    { qt_ptr_swap(d, other.d); }

    DECLARE_COMPARE_SET(const QExplicitlySharedDataPointer &p1, p1.d, const QExplicitlySharedDataPointer &p2, p2.d)
    DECLARE_COMPARE_SET(const QExplicitlySharedDataPointer &p1, p1.d, const T *ptr, ptr)
    DECLARE_COMPARE_SET(const T *ptr, ptr, const QExplicitlySharedDataPointer &p2, p2.d)
    DECLARE_COMPARE_SET(const QExplicitlySharedDataPointer &p1, p1.d, std::nullptr_t, nullptr)
    DECLARE_COMPARE_SET(std::nullptr_t, nullptr, const QExplicitlySharedDataPointer &p2, p2.d)

#undef DECLARE_COMPARE_SET

protected:
    T *clone();

private:
    void detach_helper();

    T *d;
};

// Declared here and as Q_OUTOFLINE_TEMPLATE to work-around MSVC bug causing missing symbols at link time.
template <typename T>
Q_INLINE_TEMPLATE T *QSharedDataPointer<T>::clone()
{
    return new T(*d);
}

template <typename T>
Q_OUTOFLINE_TEMPLATE void QSharedDataPointer<T>::detach_helper()
{
    T *x = clone();
    x->ref.ref();
    if (!d->ref.deref())
        delete d;
    d = x;
}

template <typename T>
Q_INLINE_TEMPLATE T *QExplicitlySharedDataPointer<T>::clone()
{
    return new T(*d);
}

template <typename T>
Q_OUTOFLINE_TEMPLATE void QExplicitlySharedDataPointer<T>::detach_helper()
{
    T *x = clone();
    x->ref.ref();
    if (!d->ref.deref())
        delete d;
    d = x;
}

template <typename T>
void swap(QSharedDataPointer<T> &p1, QSharedDataPointer<T> &p2) noexcept
{ p1.swap(p2); }

template <typename T>
void swap(QExplicitlySharedDataPointer<T> &p1, QExplicitlySharedDataPointer<T> &p2) noexcept
{ p1.swap(p2); }

template <typename T>
size_t qHash(const QSharedDataPointer<T> &ptr, size_t seed = 0) noexcept
{
    return qHash(ptr.data(), seed);
}
template <typename T>
size_t qHash(const QExplicitlySharedDataPointer<T> &ptr, size_t seed = 0) noexcept
{
    return qHash(ptr.data(), seed);
}

template<typename T> Q_DECLARE_TYPEINFO_BODY(QSharedDataPointer<T>, Q_RELOCATABLE_TYPE);
template<typename T> Q_DECLARE_TYPEINFO_BODY(QExplicitlySharedDataPointer<T>, Q_RELOCATABLE_TYPE);

#define QT_DECLARE_QSDP_SPECIALIZATION_DTOR(Class) \
    template<> QSharedDataPointer<Class>::~QSharedDataPointer();

#define QT_DECLARE_QSDP_SPECIALIZATION_DTOR_WITH_EXPORT(Class, ExportMacro) \
    template<> ExportMacro QSharedDataPointer<Class>::~QSharedDataPointer();

#define QT_DEFINE_QSDP_SPECIALIZATION_DTOR(Class) \
    template<> QSharedDataPointer<Class>::~QSharedDataPointer() \
    { \
        if (d && !d->ref.deref()) \
            delete d; \
    }

#define QT_DECLARE_QESDP_SPECIALIZATION_DTOR(Class) \
    template<> QExplicitlySharedDataPointer<Class>::~QExplicitlySharedDataPointer();

#define QT_DECLARE_QESDP_SPECIALIZATION_DTOR_WITH_EXPORT(Class, ExportMacro) \
    template<> ExportMacro QExplicitlySharedDataPointer<Class>::~QExplicitlySharedDataPointer();

#define QT_DEFINE_QESDP_SPECIALIZATION_DTOR(Class) \
    template<> QExplicitlySharedDataPointer<Class>::~QExplicitlySharedDataPointer() \
    { \
        if (d && !d->ref.deref()) \
            delete d; \
    }

QT_END_NAMESPACE

#endif // QSHAREDDATA_H
