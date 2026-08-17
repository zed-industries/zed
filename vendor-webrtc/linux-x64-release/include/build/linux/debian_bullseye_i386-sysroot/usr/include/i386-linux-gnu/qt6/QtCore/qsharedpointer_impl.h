// Copyright (C) 2021 The Qt Company Ltd.
// Copyright (C) 2022 Intel Corporation.
// Copyright (C) 2019 Klarälvdalens Datakonsult AB.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef Q_QDOC

#ifndef QSHAREDPOINTER_H
#error Do not include qsharedpointer_impl.h directly
#endif

#if 0
#pragma qt_sync_skip_header_check
#pragma qt_sync_stop_processing
#endif

#if 0
// These macros are duplicated here to make syncqt not complain a about
// this header, as we have a "qt_sync_stop_processing" below, which in turn
// is here because this file contains a template mess and duplicates the
// classes found in qsharedpointer.h
QT_BEGIN_NAMESPACE
QT_END_NAMESPACE
#pragma qt_sync_stop_processing
#endif

#include <new>
#include <QtCore/qatomic.h>
#include <QtCore/qhashfunctions.h>
#include <QtCore/qmetatype.h> // for IsPointerToTypeDerivedFromQObject

#include <memory>

QT_BEGIN_NAMESPACE

class QObject;
template <class T>
T qobject_cast(const QObject *object);

//
// forward declarations
//
template <class T> class QWeakPointer;
template <class T> class QSharedPointer;
template <class T> class QEnableSharedFromThis;

class QVariant;

template <class X, class T>
QSharedPointer<X> qSharedPointerCast(const QSharedPointer<T> &ptr);
template <class X, class T>
QSharedPointer<X> qSharedPointerDynamicCast(const QSharedPointer<T> &ptr);
template <class X, class T>
QSharedPointer<X> qSharedPointerConstCast(const QSharedPointer<T> &ptr);

#ifndef QT_NO_QOBJECT
template <class X, class T>
QSharedPointer<X> qSharedPointerObjectCast(const QSharedPointer<T> &ptr);
#endif

namespace QtPrivate {
struct EnableInternalData;
}

namespace QtSharedPointer {
    template <class T> class ExternalRefCount;

    template <class X, class Y> QSharedPointer<X> copyAndSetPointer(X * ptr, const QSharedPointer<Y> &src);

    // used in debug mode to verify the reuse of pointers
    Q_CORE_EXPORT void internalSafetyCheckAdd(const void *, const volatile void *);
    Q_CORE_EXPORT void internalSafetyCheckRemove(const void *);

    template <class T, typename Klass, typename RetVal>
    inline void executeDeleter(T *t, RetVal (Klass:: *memberDeleter)())
    { if (t) (t->*memberDeleter)(); }
    template <class T, typename Deleter>
    inline void executeDeleter(T *t, Deleter d)
    { d(t); }
    struct NormalDeleter {};

    // this uses partial template specialization
    template <class T> struct RemovePointer;
    template <class T> struct RemovePointer<T *> { typedef T Type; };
    template <class T> struct RemovePointer<QSharedPointer<T> > { typedef T Type; };
    template <class T> struct RemovePointer<QWeakPointer<T> > { typedef T Type; };

    // This class is the d-pointer of QSharedPointer and QWeakPointer.
    //
    // It is a reference-counted reference counter. "strongref" is the inner
    // reference counter, and it tracks the lifetime of the pointer itself.
    // "weakref" is the outer reference counter and it tracks the lifetime of
    // the ExternalRefCountData object.
    //
    // The deleter is stored in the destroyer member and is always a pointer to
    // a static function in ExternalRefCountWithCustomDeleter or in
    // ExternalRefCountWithContiguousData
    struct ExternalRefCountData
    {
        typedef void (*DestroyerFn)(ExternalRefCountData *);
        QBasicAtomicInt weakref;
        QBasicAtomicInt strongref;
        DestroyerFn destroyer;

        inline ExternalRefCountData(DestroyerFn d)
            : destroyer(d)
        {
            strongref.storeRelaxed(1);
            weakref.storeRelaxed(1);
        }
        inline ExternalRefCountData(Qt::Initialization) { }
        ~ExternalRefCountData() { Q_ASSERT(!weakref.loadRelaxed()); Q_ASSERT(strongref.loadRelaxed() <= 0); }

        void destroy() { destroyer(this); }

#ifndef QT_NO_QOBJECT
        Q_CORE_EXPORT static ExternalRefCountData *getAndRef(const QObject *);
        Q_CORE_EXPORT void setQObjectShared(const QObject *, bool enable);
        Q_CORE_EXPORT void checkQObjectShared(const QObject *);
#endif
        inline void checkQObjectShared(...) { }
        inline void setQObjectShared(...) { }

        // Normally, only subclasses of ExternalRefCountData are allocated
        // One exception exists in getAndRef; that uses the global operator new
        // to prevent a mismatch with the custom operator delete
        inline void *operator new(std::size_t) = delete;
        // placement new
        inline void *operator new(std::size_t, void *ptr) noexcept { return ptr; }
        inline void operator delete(void *ptr) { ::operator delete(ptr); }
        inline void operator delete(void *, void *) { }
    };
    // sizeof(ExternalRefCountData) = 12 (32-bit) / 16 (64-bit)

    template <class T, typename Deleter>
    struct CustomDeleter
    {
        Deleter deleter;
        T *ptr;

        CustomDeleter(T *p, Deleter d) : deleter(d), ptr(p) {}
        void execute() { executeDeleter(ptr, deleter); }
    };
    // sizeof(CustomDeleter) = sizeof(Deleter) + sizeof(void*) + padding
    // for Deleter = stateless functor: 8 (32-bit) / 16 (64-bit) due to padding
    // for Deleter = function pointer:  8 (32-bit) / 16 (64-bit)
    // for Deleter = PMF: 12 (32-bit) / 24 (64-bit)  (GCC)

    // This specialization of CustomDeleter for a deleter of type NormalDeleter
    // is an optimization: instead of storing a pointer to a function that does
    // the deleting, we simply delete the pointer ourselves.
    template <class T>
    struct CustomDeleter<T, NormalDeleter>
    {
        T *ptr;

        CustomDeleter(T *p, NormalDeleter) : ptr(p) {}
        void execute() { delete ptr; }
    };
    // sizeof(CustomDeleter specialization) = sizeof(void*)

    // This class extends ExternalRefCountData and implements
    // the static function that deletes the object. The pointer and the
    // custom deleter are kept in the "extra" member so we can construct
    // and destruct it independently of the full structure.
    template <class T, typename Deleter>
    struct ExternalRefCountWithCustomDeleter: public ExternalRefCountData
    {
        typedef ExternalRefCountWithCustomDeleter Self;
        typedef ExternalRefCountData BaseClass;
        CustomDeleter<T, Deleter> extra;

        static inline void deleter(ExternalRefCountData *self)
        {
            Self *realself = static_cast<Self *>(self);
            realself->extra.execute();

            // delete the deleter too
            realself->extra.~CustomDeleter<T, Deleter>();
        }
        static void safetyCheckDeleter(ExternalRefCountData *self)
        {
            internalSafetyCheckRemove(self);
            deleter(self);
        }

        static inline Self *create(T *ptr, Deleter userDeleter, DestroyerFn actualDeleter)
        {
            Self *d = static_cast<Self *>(::operator new(sizeof(Self)));

            // initialize the two sub-objects
            new (&d->extra) CustomDeleter<T, Deleter>(ptr, userDeleter);
            new (d) BaseClass(actualDeleter); // can't throw

            return d;
        }
    private:
        // prevent construction
        ExternalRefCountWithCustomDeleter() = delete;
        ~ExternalRefCountWithCustomDeleter() = delete;
        Q_DISABLE_COPY(ExternalRefCountWithCustomDeleter)
    };

    // This class extends ExternalRefCountData and adds a "T"
    // member. That way, when the create() function is called, we allocate
    // memory for both QSharedPointer's d-pointer and the actual object being
    // tracked.
    template <class T>
    struct ExternalRefCountWithContiguousData: public ExternalRefCountData
    {
        typedef ExternalRefCountData Parent;
        typedef typename std::remove_cv<T>::type NoCVType;
        NoCVType data;

        static void deleter(ExternalRefCountData *self)
        {
            ExternalRefCountWithContiguousData *that =
                    static_cast<ExternalRefCountWithContiguousData *>(self);
            that->data.~T();
            Q_UNUSED(that); // MSVC warns if T has a trivial destructor
        }
        static void safetyCheckDeleter(ExternalRefCountData *self)
        {
            internalSafetyCheckRemove(self);
            deleter(self);
        }
        static void noDeleter(ExternalRefCountData *) { }

        static inline ExternalRefCountData *create(NoCVType **ptr, DestroyerFn destroy)
        {
            ExternalRefCountWithContiguousData *d =
                static_cast<ExternalRefCountWithContiguousData *>(::operator new(sizeof(ExternalRefCountWithContiguousData)));

            // initialize the d-pointer sub-object
            // leave d->data uninitialized
            new (d) Parent(destroy); // can't throw

            *ptr = &d->data;
            return d;
        }

    private:
        // prevent construction
        ExternalRefCountWithContiguousData() = delete;
        ~ExternalRefCountWithContiguousData() = delete;
        Q_DISABLE_COPY(ExternalRefCountWithContiguousData)
    };

#ifndef QT_NO_QOBJECT
    Q_CORE_EXPORT QWeakPointer<QObject> weakPointerFromVariant_internal(const QVariant &variant);
    Q_CORE_EXPORT QSharedPointer<QObject> sharedPointerFromVariant_internal(const QVariant &variant);
#endif
} // namespace QtSharedPointer

template <class T> class QSharedPointer
{
    typedef QtSharedPointer::ExternalRefCountData Data;
    template <typename X>
    using IfCompatible = typename std::enable_if<std::is_convertible<X*, T*>::value, bool>::type;

public:
    typedef T Type;
    typedef T element_type;
    typedef T value_type;
    typedef value_type *pointer;
    typedef const value_type *const_pointer;
    typedef value_type &reference;
    typedef const value_type &const_reference;
    typedef qptrdiff difference_type;

    T *data() const noexcept { return value; }
    T *get() const noexcept { return value; }
    bool isNull() const noexcept { return !data(); }
    explicit operator bool() const noexcept { return !isNull(); }
    bool operator !() const noexcept { return isNull(); }
    T &operator*() const { return *data(); }
    T *operator->() const noexcept { return data(); }

    constexpr QSharedPointer() noexcept : value(nullptr), d(nullptr) { }
    ~QSharedPointer() { deref(); }

    constexpr QSharedPointer(std::nullptr_t) noexcept : value(nullptr), d(nullptr) { }

    template <class X, IfCompatible<X> = true>
    inline explicit QSharedPointer(X *ptr) : value(ptr) // noexcept
    { internalConstruct(ptr, QtSharedPointer::NormalDeleter()); }

    template <class X, typename Deleter, IfCompatible<X> = true>
    inline QSharedPointer(X *ptr, Deleter deleter) : value(ptr) // throws
    { internalConstruct(ptr, deleter); }

    template <typename Deleter>
    QSharedPointer(std::nullptr_t, Deleter deleter) : value(nullptr)
    { internalConstruct(static_cast<T *>(nullptr), deleter); }

    QSharedPointer(const QSharedPointer &other) noexcept : value(other.value), d(other.d)
    { if (d) ref(); }
    QSharedPointer &operator=(const QSharedPointer &other) noexcept
    {
        QSharedPointer copy(other);
        swap(copy);
        return *this;
    }
    QSharedPointer(QSharedPointer &&other) noexcept
        : value(other.value), d(other.d)
    {
        other.d = nullptr;
        other.value = nullptr;
    }
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QSharedPointer)

    template <class X, IfCompatible<X> = true>
    QSharedPointer(QSharedPointer<X> &&other) noexcept
        : value(other.value), d(other.d)
    {
        other.d = nullptr;
        other.value = nullptr;
    }

    template <class X, IfCompatible<X> = true>
    QSharedPointer &operator=(QSharedPointer<X> &&other) noexcept
    {
        QSharedPointer moved(std::move(other));
        swap(moved);
        return *this;
    }

    template <class X, IfCompatible<X> = true>
    QSharedPointer(const QSharedPointer<X> &other) noexcept : value(other.value), d(other.d)
    { if (d) ref(); }

    template <class X, IfCompatible<X> = true>
    inline QSharedPointer &operator=(const QSharedPointer<X> &other)
    {
        QSharedPointer copy(other);
        swap(copy);
        return *this;
    }

    template <class X, IfCompatible<X> = true>
    inline QSharedPointer(const QWeakPointer<X> &other) : value(nullptr), d(nullptr)
    { *this = other; }

    template <class X, IfCompatible<X> = true>
    inline QSharedPointer<T> &operator=(const QWeakPointer<X> &other)
    { internalSet(other.d, other.value); return *this; }

    inline void swap(QSharedPointer &other) noexcept
    { this->internalSwap(other); }

    inline void reset() { clear(); }
    inline void reset(T *t)
    { QSharedPointer copy(t); swap(copy); }
    template <typename Deleter>
    inline void reset(T *t, Deleter deleter)
    { QSharedPointer copy(t, deleter); swap(copy); }

    template <class X>
    QSharedPointer<X> staticCast() const
    {
        return qSharedPointerCast<X, T>(*this);
    }

    template <class X>
    QSharedPointer<X> dynamicCast() const
    {
        return qSharedPointerDynamicCast<X, T>(*this);
    }

    template <class X>
    QSharedPointer<X> constCast() const
    {
        return qSharedPointerConstCast<X, T>(*this);
    }

#ifndef QT_NO_QOBJECT
    template <class X>
    QSharedPointer<X> objectCast() const
    {
        return qSharedPointerObjectCast<X, T>(*this);
    }
#endif

    inline void clear() { QSharedPointer copy; swap(copy); }

    QWeakPointer<T> toWeakRef() const;

    template <typename... Args>
    static QSharedPointer create(Args && ...arguments)
    {
        typedef QtSharedPointer::ExternalRefCountWithContiguousData<T> Private;
# ifdef QT_SHAREDPOINTER_TRACK_POINTERS
        typename Private::DestroyerFn destroy = &Private::safetyCheckDeleter;
# else
        typename Private::DestroyerFn destroy = &Private::deleter;
# endif
        typename Private::DestroyerFn noDestroy = &Private::noDeleter;
        QSharedPointer result(Qt::Uninitialized);
        typename std::remove_cv<T>::type *ptr;
        result.d = Private::create(&ptr, noDestroy);

        // now initialize the data
        new (ptr) T(std::forward<Args>(arguments)...);
        result.value = ptr;
        result.d->destroyer = destroy;
        result.d->setQObjectShared(result.value, true);
# ifdef QT_SHAREDPOINTER_TRACK_POINTERS
        internalSafetyCheckAdd(result.d, result.value);
# endif
        result.enableSharedFromThis(result.data());
        return result;
    }

#define DECLARE_COMPARE_SET(T1, A1, T2, A2) \
    friend bool operator==(T1, T2) noexcept \
    { return A1 == A2; } \
    friend bool operator!=(T1, T2) noexcept \
    { return A1 != A2; }

#define DECLARE_TEMPLATE_COMPARE_SET(T1, A1, T2, A2) \
    template <typename X> \
    friend bool operator==(T1, T2) noexcept \
    { return A1 == A2; } \
    template <typename X> \
    friend bool operator!=(T1, T2) noexcept \
    { return A1 != A2; }

    DECLARE_TEMPLATE_COMPARE_SET(const QSharedPointer &p1, p1.data(), const QSharedPointer<X> &p2, p2.data())
    DECLARE_TEMPLATE_COMPARE_SET(const QSharedPointer &p1, p1.data(), X *ptr, ptr)
    DECLARE_TEMPLATE_COMPARE_SET(X *ptr, ptr, const QSharedPointer &p2, p2.data())
    DECLARE_COMPARE_SET(const QSharedPointer &p1, p1.data(), std::nullptr_t, nullptr)
    DECLARE_COMPARE_SET(std::nullptr_t, nullptr, const QSharedPointer &p2, p2.data())
#undef DECLARE_TEMPLATE_COMPARE_SET
#undef DECLARE_COMPARE_SET

private:
    explicit QSharedPointer(Qt::Initialization) {}

    void deref() noexcept
    { deref(d); }
    static void deref(Data *dd) noexcept
    {
        if (!dd) return;
        if (!dd->strongref.deref()) {
            dd->destroy();
        }
        if (!dd->weakref.deref())
            delete dd;
    }

    template <class X>
    inline void enableSharedFromThis(const QEnableSharedFromThis<X> *ptr)
    {
        ptr->initializeFromSharedPointer(constCast<typename std::remove_cv<T>::type>());
    }

    inline void enableSharedFromThis(...) {}

    template <typename X, typename Deleter>
    inline void internalConstruct(X *ptr, Deleter deleter)
    {
        typedef QtSharedPointer::ExternalRefCountWithCustomDeleter<X, Deleter> Private;
# ifdef QT_SHAREDPOINTER_TRACK_POINTERS
        typename Private::DestroyerFn actualDeleter = &Private::safetyCheckDeleter;
# else
        typename Private::DestroyerFn actualDeleter = &Private::deleter;
# endif
        d = Private::create(ptr, deleter, actualDeleter);

#ifdef QT_SHAREDPOINTER_TRACK_POINTERS
        internalSafetyCheckAdd(d, ptr);
#endif
        d->setQObjectShared(ptr, true);
        enableSharedFromThis(ptr);
    }

    void internalSwap(QSharedPointer &other) noexcept
    {
        qt_ptr_swap(d, other.d);
        qt_ptr_swap(this->value, other.value);
    }

    template <class X> friend class QSharedPointer;
    template <class X> friend class QWeakPointer;
    template <class X, class Y> friend QSharedPointer<X> QtSharedPointer::copyAndSetPointer(X * ptr, const QSharedPointer<Y> &src);
    void ref() const noexcept { d->weakref.ref(); d->strongref.ref(); }

    inline void internalSet(Data *o, T *actual)
    {
        if (o) {
            // increase the strongref, but never up from zero
            // or less (-1 is used by QWeakPointer on untracked QObject)
            int tmp = o->strongref.loadRelaxed();
            while (tmp > 0) {
                // try to increment from "tmp" to "tmp + 1"
                if (o->strongref.testAndSetRelaxed(tmp, tmp + 1))
                    break;   // succeeded
                tmp = o->strongref.loadRelaxed();  // failed, try again
            }

            if (tmp > 0) {
                o->weakref.ref();
            } else {
                o->checkQObjectShared(actual);
                o = nullptr;
            }
        }

        qt_ptr_swap(d, o);
        qt_ptr_swap(this->value, actual);
        if (!d || d->strongref.loadRelaxed() == 0)
            this->value = nullptr;

        // dereference saved data
        deref(o);
    }

    Type *value;
    Data *d;
};

template <class T>
class QWeakPointer
{
    typedef QtSharedPointer::ExternalRefCountData Data;
    template <typename X>
    using IfCompatible = typename std::enable_if<std::is_convertible<X*, T*>::value, bool>::type;

public:
    typedef T element_type;
    typedef T value_type;
    typedef value_type *pointer;
    typedef const value_type *const_pointer;
    typedef value_type &reference;
    typedef const value_type &const_reference;
    typedef qptrdiff difference_type;

    bool isNull() const noexcept { return d == nullptr || d->strongref.loadRelaxed() == 0 || value == nullptr; }
    explicit operator bool() const noexcept { return !isNull(); }
    bool operator !() const noexcept { return isNull(); }

    constexpr QWeakPointer() noexcept : d(nullptr), value(nullptr) { }
    inline ~QWeakPointer() { if (d && !d->weakref.deref()) delete d; }

    QWeakPointer(const QWeakPointer &other) noexcept : d(other.d), value(other.value)
    { if (d) d->weakref.ref(); }
    QWeakPointer(QWeakPointer &&other) noexcept
        : d(other.d), value(other.value)
    {
        other.d = nullptr;
        other.value = nullptr;
    }
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QWeakPointer)

    template <class X, IfCompatible<X> = true>
    QWeakPointer(QWeakPointer<X> &&other) noexcept
        : d(other.d), value(other.value)
    {
        other.d = nullptr;
        other.value = nullptr;
    }

    template <class X, IfCompatible<X> = true>
    QWeakPointer &operator=(QWeakPointer<X> &&other) noexcept
    {
        QWeakPointer moved(std::move(other));
        swap(moved);
        return *this;
    }

    QWeakPointer &operator=(const QWeakPointer &other) noexcept
    {
        QWeakPointer copy(other);
        swap(copy);
        return *this;
    }

    void swap(QWeakPointer &other) noexcept
    {
        qt_ptr_swap(this->d, other.d);
        qt_ptr_swap(this->value, other.value);
    }

    inline QWeakPointer(const QSharedPointer<T> &o) : d(o.d), value(o.data())
    { if (d) d->weakref.ref();}
    inline QWeakPointer &operator=(const QSharedPointer<T> &o)
    {
        internalSet(o.d, o.value);
        return *this;
    }

    template <class X, IfCompatible<X> = true>
    inline QWeakPointer(const QWeakPointer<X> &o) : d(nullptr), value(nullptr)
    { *this = o; }

    template <class X, IfCompatible<X> = true>
    inline QWeakPointer &operator=(const QWeakPointer<X> &o)
    {
        // conversion between X and T could require access to the virtual table
        // so force the operation to go through QSharedPointer
        *this = o.toStrongRef();
        return *this;
    }

    template <class X, IfCompatible<X> = true>
    inline QWeakPointer(const QSharedPointer<X> &o) : d(nullptr), value(nullptr)
    { *this = o; }

    template <class X, IfCompatible<X> = true>
    inline QWeakPointer &operator=(const QSharedPointer<X> &o)
    {
        internalSet(o.d, o.data());
        return *this;
    }

    inline void clear() { *this = QWeakPointer(); }

    inline QSharedPointer<T> toStrongRef() const { return QSharedPointer<T>(*this); }
    // std::weak_ptr compatibility:
    inline QSharedPointer<T> lock() const { return toStrongRef(); }

    template <class X>
    bool operator==(const QWeakPointer<X> &o) const noexcept
    { return d == o.d && value == static_cast<const T *>(o.value); }

    template <class X>
    bool operator!=(const QWeakPointer<X> &o) const noexcept
    { return !(*this == o); }

    template <class X>
    bool operator==(const QSharedPointer<X> &o) const noexcept
    { return d == o.d; }

    template <class X>
    bool operator!=(const QSharedPointer<X> &o) const noexcept
    { return !(*this == o); }

    template <typename X>
    friend bool operator==(const QSharedPointer<X> &p1, const QWeakPointer &p2) noexcept
    { return p2 == p1; }
    template <typename X>
    friend bool operator!=(const QSharedPointer<X> &p1, const QWeakPointer &p2) noexcept
    { return p2 != p1; }

    friend bool operator==(const QWeakPointer &p, std::nullptr_t)
    { return p.isNull(); }
    friend bool operator==(std::nullptr_t, const QWeakPointer &p)
    { return p.isNull(); }
    friend bool operator!=(const QWeakPointer &p, std::nullptr_t)
    { return !p.isNull(); }
    friend bool operator!=(std::nullptr_t, const QWeakPointer &p)
    { return !p.isNull(); }

private:
    friend struct QtPrivate::EnableInternalData;
    template <class X> friend class QSharedPointer;
    template <class X> friend class QWeakPointer;
    template <class X> friend class QPointer;

    template <class X>
    inline QWeakPointer &assign(X *ptr)
    { return *this = QWeakPointer<X>(ptr, true); }

#ifndef QT_NO_QOBJECT
    template <class X, IfCompatible<X> = true>
    inline QWeakPointer(X *ptr, bool) : d(ptr ? Data::getAndRef(ptr) : nullptr), value(ptr)
    { }
#endif

    inline void internalSet(Data *o, T *actual)
    {
        if (d == o) return;
        if (o)
            o->weakref.ref();
        if (d && !d->weakref.deref())
            delete d;
        d = o;
        value = actual;
    }

    // ### TODO - QTBUG-88102: remove all users of this API; no one should ever
    // access a weak pointer's data but the weak pointer itself
    inline T *internalData() const noexcept
    {
        return d == nullptr || d->strongref.loadRelaxed() == 0 ? nullptr : value;
    }

    Data *d;
    T *value;
};

namespace QtPrivate {
struct EnableInternalData {
    template <typename T>
    static T *internalData(const QWeakPointer<T> &p) noexcept { return p.internalData(); }
};
// hack to delay name lookup to instantiation time by making
// EnableInternalData a dependent name:
template <typename T>
struct EnableInternalDataWrap : EnableInternalData {};
}

template <class T>
class QEnableSharedFromThis
{
protected:
    QEnableSharedFromThis() = default;
    QEnableSharedFromThis(const QEnableSharedFromThis &) {}
    QEnableSharedFromThis &operator=(const QEnableSharedFromThis &) { return *this; }

public:
    inline QSharedPointer<T> sharedFromThis() { return QSharedPointer<T>(weakPointer); }
    inline QSharedPointer<const T> sharedFromThis() const { return QSharedPointer<const T>(weakPointer); }

private:
    template <class X> friend class QSharedPointer;
    template <class X>
    inline void initializeFromSharedPointer(const QSharedPointer<X> &ptr) const
    {
        weakPointer = ptr;
    }

    mutable QWeakPointer<T> weakPointer;
};

//
// operator-
//
template <class T, class X>
Q_INLINE_TEMPLATE typename QSharedPointer<T>::difference_type operator-(const QSharedPointer<T> &ptr1, const QSharedPointer<X> &ptr2)
{
    return ptr1.data() - ptr2.data();
}
template <class T, class X>
Q_INLINE_TEMPLATE typename QSharedPointer<T>::difference_type operator-(const QSharedPointer<T> &ptr1, X *ptr2)
{
    return ptr1.data() - ptr2;
}
template <class T, class X>
Q_INLINE_TEMPLATE typename QSharedPointer<X>::difference_type operator-(T *ptr1, const QSharedPointer<X> &ptr2)
{
    return ptr1 - ptr2.data();
}

//
// operator<
//
template <class T, class X>
Q_INLINE_TEMPLATE bool operator<(const QSharedPointer<T> &ptr1, const QSharedPointer<X> &ptr2)
{
    using CT = typename std::common_type<T *, X *>::type;
    return std::less<CT>()(ptr1.data(), ptr2.data());
}
template <class T, class X>
Q_INLINE_TEMPLATE bool operator<(const QSharedPointer<T> &ptr1, X *ptr2)
{
    using CT = typename std::common_type<T *, X *>::type;
    return std::less<CT>()(ptr1.data(), ptr2);
}
template <class T, class X>
Q_INLINE_TEMPLATE bool operator<(T *ptr1, const QSharedPointer<X> &ptr2)
{
    using CT = typename std::common_type<T *, X *>::type;
    return std::less<CT>()(ptr1, ptr2.data());
}

//
// qHash
//
template <class T>
Q_INLINE_TEMPLATE size_t qHash(const QSharedPointer<T> &ptr, size_t seed = 0)
{
    return qHash(ptr.data(), seed);
}


template <class T>
Q_INLINE_TEMPLATE QWeakPointer<T> QSharedPointer<T>::toWeakRef() const
{
    return QWeakPointer<T>(*this);
}

template <class T>
inline void swap(QSharedPointer<T> &p1, QSharedPointer<T> &p2) noexcept
{ p1.swap(p2); }

template <class T>
inline void swap(QWeakPointer<T> &p1, QWeakPointer<T> &p2) noexcept
{ p1.swap(p2); }

namespace QtSharedPointer {
// helper functions:
    template <class X, class T>
    Q_INLINE_TEMPLATE QSharedPointer<X> copyAndSetPointer(X *ptr, const QSharedPointer<T> &src)
    {
        QSharedPointer<X> result;
        result.internalSet(src.d, ptr);
        return result;
    }
}

// cast operators
template <class X, class T>
Q_INLINE_TEMPLATE QSharedPointer<X> qSharedPointerCast(const QSharedPointer<T> &src)
{
    X *ptr = static_cast<X *>(src.data()); // if you get an error in this line, the cast is invalid
    return QtSharedPointer::copyAndSetPointer(ptr, src);
}
template <class X, class T>
Q_INLINE_TEMPLATE QSharedPointer<X> qSharedPointerCast(const QWeakPointer<T> &src)
{
    return qSharedPointerCast<X, T>(src.toStrongRef());
}

template <class X, class T>
Q_INLINE_TEMPLATE QSharedPointer<X> qSharedPointerDynamicCast(const QSharedPointer<T> &src)
{
    X *ptr = dynamic_cast<X *>(src.data()); // if you get an error in this line, the cast is invalid
    if (!ptr)
        return QSharedPointer<X>();
    return QtSharedPointer::copyAndSetPointer(ptr, src);
}
template <class X, class T>
Q_INLINE_TEMPLATE QSharedPointer<X> qSharedPointerDynamicCast(const QWeakPointer<T> &src)
{
    return qSharedPointerDynamicCast<X, T>(src.toStrongRef());
}

template <class X, class T>
Q_INLINE_TEMPLATE QSharedPointer<X> qSharedPointerConstCast(const QSharedPointer<T> &src)
{
    X *ptr = const_cast<X *>(src.data()); // if you get an error in this line, the cast is invalid
    return QtSharedPointer::copyAndSetPointer(ptr, src);
}
template <class X, class T>
Q_INLINE_TEMPLATE QSharedPointer<X> qSharedPointerConstCast(const QWeakPointer<T> &src)
{
    return qSharedPointerConstCast<X, T>(src.toStrongRef());
}

template <class X, class T>
Q_INLINE_TEMPLATE
QWeakPointer<X> qWeakPointerCast(const QSharedPointer<T> &src)
{
    return qSharedPointerCast<X, T>(src).toWeakRef();
}

#ifndef QT_NO_QOBJECT
template <class X, class T>
Q_INLINE_TEMPLATE QSharedPointer<X> qSharedPointerObjectCast(const QSharedPointer<T> &src)
{
    X *ptr = qobject_cast<X *>(src.data());
    return QtSharedPointer::copyAndSetPointer(ptr, src);
}
template <class X, class T>
Q_INLINE_TEMPLATE QSharedPointer<X> qSharedPointerObjectCast(const QWeakPointer<T> &src)
{
    return qSharedPointerObjectCast<X>(src.toStrongRef());
}

template <class X, class T>
inline QSharedPointer<typename QtSharedPointer::RemovePointer<X>::Type>
qobject_cast(const QSharedPointer<T> &src)
{
    return qSharedPointerObjectCast<typename QtSharedPointer::RemovePointer<X>::Type, T>(src);
}
template <class X, class T>
inline QSharedPointer<typename QtSharedPointer::RemovePointer<X>::Type>
qobject_cast(const QWeakPointer<T> &src)
{
    return qSharedPointerObjectCast<typename QtSharedPointer::RemovePointer<X>::Type, T>(src);
}

/// ### TODO - QTBUG-88102: make this use toStrongRef() (once support for
/// storing non-managed QObjects in QWeakPointer is removed)
template<typename T>
QWeakPointer<typename std::enable_if<QtPrivate::IsPointerToTypeDerivedFromQObject<T*>::Value, T>::type>
qWeakPointerFromVariant(const QVariant &variant)
{
    return QWeakPointer<T>(qobject_cast<T*>(QtPrivate::EnableInternalData::internalData(QtSharedPointer::weakPointerFromVariant_internal(variant))));
}
template<typename T>
QSharedPointer<typename std::enable_if<QtPrivate::IsPointerToTypeDerivedFromQObject<T*>::Value, T>::type>
qSharedPointerFromVariant(const QVariant &variant)
{
    return qSharedPointerObjectCast<T>(QtSharedPointer::sharedPointerFromVariant_internal(variant));
}

// std::shared_ptr helpers

template <typename X, class T>
std::shared_ptr<X> qobject_pointer_cast(const std::shared_ptr<T> &src)
{
    using element_type = typename std::shared_ptr<X>::element_type;
    return std::shared_ptr<X>(src, qobject_cast<element_type *>(src.get()));
}

template <typename X, class T>
std::shared_ptr<X> qobject_pointer_cast(std::shared_ptr<T> &&src)
{
    using element_type = typename std::shared_ptr<X>::element_type;
    auto castResult = qobject_cast<element_type *>(src.get());
    if (castResult) {
        // C++2a's move aliasing constructor will leave src empty.
        // Before C++2a we don't really know if the compiler has support for it.
        // The move aliasing constructor is the resolution for LWG2996,
        // which does not impose a feature-testing macro. So: clear src.
        return std::shared_ptr<X>(std::exchange(src, nullptr), castResult);
    }
    return std::shared_ptr<X>();
}

template <typename X, class T>
std::shared_ptr<X> qSharedPointerObjectCast(const std::shared_ptr<T> &src)
{
    return qobject_pointer_cast<X>(src);
}

template <typename X, class T>
std::shared_ptr<X> qSharedPointerObjectCast(std::shared_ptr<T> &&src)
{
    return qobject_pointer_cast<X>(std::move(src));
}

#endif

template<typename T> Q_DECLARE_TYPEINFO_BODY(QWeakPointer<T>, Q_RELOCATABLE_TYPE);
template<typename T> Q_DECLARE_TYPEINFO_BODY(QSharedPointer<T>, Q_RELOCATABLE_TYPE);


QT_END_NAMESPACE

#endif
