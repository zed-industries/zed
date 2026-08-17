/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtConcurrent module of the Qt Toolkit.
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

#ifndef QTCONCURRENT_FUNCTIONWRAPPERS_H
#define QTCONCURRENT_FUNCTIONWRAPPERS_H

#include <QtConcurrent/qtconcurrentcompilertest.h>
#include <QtCore/QStringList>

#if !defined(QT_NO_CONCURRENT) || defined(Q_CLANG_QDOC)

QT_BEGIN_NAMESPACE

namespace QtConcurrent {

template <typename T>
class FunctionWrapper0
{
public:
    typedef T (*FunctionPointerType)();
    typedef T result_type;
    inline FunctionWrapper0(FunctionPointerType _functionPointer)
    :functionPointer(_functionPointer) { }

    inline T operator()()
    {
        return functionPointer();
    }
private:
    FunctionPointerType functionPointer;
};

template <typename T, typename U>
class FunctionWrapper1
{
public:
    typedef T (*FunctionPointerType)(U u);
    typedef T result_type;
    inline FunctionWrapper1(FunctionPointerType _functionPointer)
    :functionPointer(_functionPointer) { }

    inline T operator()(U u)
    {
        return functionPointer(u);
    }

private:
    FunctionPointerType functionPointer;
};

template <typename T, typename U, typename V>
class FunctionWrapper2
{
public:
    typedef T (*FunctionPointerType)(U u, V v);
    typedef T result_type;
    inline FunctionWrapper2(FunctionPointerType _functionPointer)
    :functionPointer(_functionPointer) { }

    inline T operator()(U u, V v)
    {
        return functionPointer(u, v);
    }
private:
    FunctionPointerType functionPointer;
};

template <typename T, typename C>
class MemberFunctionWrapper
{
public:
    typedef T (C::*FunctionPointerType)();
    typedef T result_type;
    inline MemberFunctionWrapper(FunctionPointerType _functionPointer)
    :functionPointer(_functionPointer) { }

    inline T operator()(C &c)
    {
        return (c.*functionPointer)();
    }
private:
    FunctionPointerType functionPointer;
};

template <typename T, typename C, typename U>
class MemberFunctionWrapper1
{
public:
    typedef T (C::*FunctionPointerType)(U);
    typedef T result_type;

    inline MemberFunctionWrapper1(FunctionPointerType _functionPointer)
        : functionPointer(_functionPointer)
    { }

    inline T operator()(C &c, U u)
    {
        return (c.*functionPointer)(u);
    }

private:
    FunctionPointerType functionPointer;
};

template <typename T, typename C>
class ConstMemberFunctionWrapper
{
public:
    typedef T (C::*FunctionPointerType)() const;
    typedef T result_type;
    inline ConstMemberFunctionWrapper(FunctionPointerType _functionPointer)
    :functionPointer(_functionPointer) { }

    inline T operator()(const C &c) const
    {
        return (c.*functionPointer)();
    }
private:
    FunctionPointerType functionPointer;
};

} // namespace QtConcurrent.

namespace QtPrivate {

template <typename T>
const T& createFunctionWrapper(const T& t)
{
    return t;
}

template <typename T, typename U>
QtConcurrent::FunctionWrapper1<T, U> createFunctionWrapper(T (*func)(U))
{
    return QtConcurrent::FunctionWrapper1<T, U>(func);
}

template <typename T, typename C>
QtConcurrent::MemberFunctionWrapper<T, C> createFunctionWrapper(T (C::*func)())
{
    return QtConcurrent::MemberFunctionWrapper<T, C>(func);
}

template <typename T, typename C, typename U>
QtConcurrent::MemberFunctionWrapper1<T, C, U> createFunctionWrapper(T (C::*func)(U))
{
    return QtConcurrent::MemberFunctionWrapper1<T, C, U>(func);
}

template <typename T, typename C>
QtConcurrent::ConstMemberFunctionWrapper<T, C> createFunctionWrapper(T (C::*func)() const)
{
    return QtConcurrent::ConstMemberFunctionWrapper<T, C>(func);
}

#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510
template <typename T, typename U>
QtConcurrent::FunctionWrapper1<T, U> createFunctionWrapper(T (*func)(U) noexcept)
{
    return QtConcurrent::FunctionWrapper1<T, U>(func);
}

template <typename T, typename C>
QtConcurrent::MemberFunctionWrapper<T, C> createFunctionWrapper(T (C::*func)() noexcept)
{
    return QtConcurrent::MemberFunctionWrapper<T, C>(func);
}

template <typename T, typename C, typename U>
QtConcurrent::MemberFunctionWrapper1<T, C, U> createFunctionWrapper(T (C::*func)(U) noexcept)
{
    return QtConcurrent::MemberFunctionWrapper1<T, C, U>(func);
}

template <typename T, typename C>
QtConcurrent::ConstMemberFunctionWrapper<T, C> createFunctionWrapper(T (C::*func)() const noexcept)
{
    return QtConcurrent::ConstMemberFunctionWrapper<T, C>(func);
}
#endif

struct PushBackWrapper
{
    typedef void result_type;

    template <class C, class U>
    inline void operator()(C &c, const U &u) const
    {
        return c.push_back(u);
    }

    template <class C, class U>
    inline void operator()(C &c, U &&u) const
    {
        return c.push_back(u);
    }
};

template <typename Functor, bool foo = HasResultType<Functor>::Value>
struct LazyResultType { typedef typename Functor::result_type Type; };
template <typename Functor>
struct LazyResultType<Functor, false> { typedef void Type; };

template <class T>
struct ReduceResultType;

template <class U, class V>
struct ReduceResultType<void(*)(U&,V)>
{
    typedef U ResultType;
};

template <class T, class C, class U>
struct ReduceResultType<T(C::*)(U)>
{
    typedef C ResultType;
};

#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510
template <class U, class V>
struct ReduceResultType<void(*)(U&,V) noexcept>
{
    typedef U ResultType;
};

template <class T, class C, class U>
struct ReduceResultType<T(C::*)(U) noexcept>
{
    typedef C ResultType;
};
#endif

template <class InputSequence, class MapFunctor>
struct MapResultType
{
    typedef typename LazyResultType<MapFunctor>::Type ResultType;
};

template <class U, class V>
struct MapResultType<void, U (*)(V)>
{
    typedef U ResultType;
};

template <class T, class C>
struct MapResultType<void, T(C::*)() const>
{
    typedef T ResultType;
};

#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510
template <class U, class V>
struct MapResultType<void, U (*)(V) noexcept>
{
    typedef U ResultType;
};

template <class T, class C>
struct MapResultType<void, T(C::*)() const noexcept>
{
    typedef T ResultType;
};
#endif

#ifndef QT_NO_TEMPLATE_TEMPLATE_PARAMETERS

template <template <typename> class InputSequence, typename MapFunctor, typename T>
struct MapResultType<InputSequence<T>, MapFunctor>
{
    typedef InputSequence<typename LazyResultType<MapFunctor>::Type> ResultType;
};

template <template <typename> class InputSequence, class T, class U, class V>
struct MapResultType<InputSequence<T>, U (*)(V)>
{
    typedef InputSequence<U> ResultType;
};

template <template <typename> class InputSequence, class T, class U, class C>
struct MapResultType<InputSequence<T>, U(C::*)() const>
{
    typedef InputSequence<U> ResultType;
};

#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510

template <template <typename> class InputSequence, class T, class U, class V>
struct MapResultType<InputSequence<T>, U (*)(V) noexcept>
{
    typedef InputSequence<U> ResultType;
};

template <template <typename> class InputSequence, class T, class U, class C>
struct MapResultType<InputSequence<T>, U(C::*)() const noexcept>
{
    typedef InputSequence<U> ResultType;
};
#endif

#endif // QT_NO_TEMPLATE_TEMPLATE_PARAMETER

template <class MapFunctor>
struct MapResultType<QStringList, MapFunctor>
{
    typedef QList<typename LazyResultType<MapFunctor>::Type> ResultType;
};

template <class U, class V>
struct MapResultType<QStringList, U (*)(V)>
{
    typedef QList<U> ResultType;
};

template <class U, class C>
struct MapResultType<QStringList, U(C::*)() const>
{
    typedef QList<U> ResultType;
};

#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510

template <class U, class V>
struct MapResultType<QStringList, U (*)(V) noexcept>
{
    typedef QList<U> ResultType;
};

template <class U, class C>
struct MapResultType<QStringList, U(C::*)() const noexcept>
{
    typedef QList<U> ResultType;
};
#endif

} // namespace QtPrivate.


QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
