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

// Generated code, do not edit! Use generator at tools/qtconcurrent/generaterun/
#ifndef QTCONCURRENT_STOREDFUNCTIONCALL_H
#define QTCONCURRENT_STOREDFUNCTIONCALL_H

#include <QtConcurrent/qtconcurrent_global.h>

#ifndef QT_NO_CONCURRENT
#include <QtConcurrent/qtconcurrentrunbase.h>

QT_BEGIN_NAMESPACE


#ifndef Q_QDOC

namespace QtConcurrent {
template <typename T, typename FunctionPointer>
struct StoredFunctorCall0: public RunFunctionTask<T>
{
    inline StoredFunctorCall0(FunctionPointer _function)
      : function(_function) {}
    void runFunctor() override { this->result = function(); }
    FunctionPointer function;

};

template <typename FunctionPointer>
struct StoredFunctorCall0<void, FunctionPointer>: public RunFunctionTask<void>
{
    inline StoredFunctorCall0(FunctionPointer _function)
      : function(_function) {}
    void runFunctor() override { function(); }
    FunctionPointer function;

};

template <typename T, typename FunctionPointer>
struct StoredFunctorPointerCall0: public RunFunctionTask<T>
{
    inline StoredFunctorPointerCall0(FunctionPointer * _function)
      : function(_function) {}
    void runFunctor() override { this->result =(*function)(); }
    FunctionPointer * function;

};

template <typename T, typename FunctionPointer>
struct VoidStoredFunctorPointerCall0: public RunFunctionTask<T>
{
    inline VoidStoredFunctorPointerCall0(FunctionPointer * _function)
      : function(_function) {}
    void runFunctor() override { (*function)(); }
    FunctionPointer * function;

};

template <typename T, typename FunctionPointer>
struct SelectStoredFunctorPointerCall0
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredFunctorPointerCall0    <T, FunctionPointer>,
             VoidStoredFunctorPointerCall0<T, FunctionPointer> >::type type;
};
template <typename T, typename Class>
class StoredMemberFunctionCall0 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionCall0(T (Class::*_fn)(), const Class &_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        this->result = (object.*fn)();
    }
private:
    T (Class::*fn)();
    Class object;

};
template <typename T, typename Class>
class VoidStoredMemberFunctionCall0 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionCall0(T (Class::*_fn)(), const Class &_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        (object.*fn)();
    }
private:
    T (Class::*fn)();
    Class object;

};
template <typename T, typename Class>
struct SelectStoredMemberFunctionCall0
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionCall0    <T, Class>,
             VoidStoredMemberFunctionCall0<T, Class> >::type type;
};
template <typename T, typename Class>
class StoredConstMemberFunctionCall0 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionCall0(T (Class::*_fn)() const, const Class &_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        this->result = (object.*fn)();
    }
private:
    T (Class::*fn)() const;
    const Class object;

};
template <typename T, typename Class>
class VoidStoredConstMemberFunctionCall0 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionCall0(T (Class::*_fn)() const, const Class &_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        (object.*fn)();
    }
private:
    T (Class::*fn)() const;
    const Class object;

};
template <typename T, typename Class>
struct SelectStoredConstMemberFunctionCall0
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionCall0    <T, Class>,
             VoidStoredConstMemberFunctionCall0<T, Class> >::type type;
};
template <typename T, typename Class>
class StoredMemberFunctionPointerCall0 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionPointerCall0(T (Class::*_fn)(), Class *_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        this->result = (object->*fn)();
    }
private:
    T (Class::*fn)();
    Class *object;

};
template <typename T, typename Class>
class VoidStoredMemberFunctionPointerCall0 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionPointerCall0(T (Class::*_fn)(), Class *_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        (object->*fn)();
    }
private:
    T (Class::*fn)();
    Class *object;

};
template <typename T, typename Class>
struct SelectStoredMemberFunctionPointerCall0
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionPointerCall0    <T, Class>,
             VoidStoredMemberFunctionPointerCall0<T, Class> >::type type;
};
template <typename T, typename Class>
class StoredConstMemberFunctionPointerCall0 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionPointerCall0(T (Class::*_fn)() const, Class const *_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        this->result = (object->*fn)();
    }
private:
    T (Class::*fn)() const;
    Class const *object;

};
template <typename T, typename Class>
class VoidStoredConstMemberFunctionPointerCall0 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionPointerCall0(T (Class::*_fn)() const, Class const *_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        (object->*fn)();
    }
private:
    T (Class::*fn)() const;
    Class const *object;

};
template <typename T, typename Class>
struct SelectStoredConstMemberFunctionPointerCall0
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionPointerCall0    <T, Class>,
             VoidStoredConstMemberFunctionPointerCall0<T, Class> >::type type;
};
#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510
template <typename T, typename Class>
class StoredNoExceptMemberFunctionCall0 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionCall0(T (Class::*_fn)() noexcept, const Class &_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        this->result = (object.*fn)();
    }
private:
    T (Class::*fn)() noexcept;
    Class object;

};
template <typename T, typename Class>
class VoidStoredNoExceptMemberFunctionCall0 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionCall0(T (Class::*_fn)() noexcept, const Class &_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        (object.*fn)();
    }
private:
    T (Class::*fn)() noexcept;
    Class object;

};
template <typename T, typename Class>
struct SelectStoredNoExceptMemberFunctionCall0
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionCall0    <T, Class>,
             VoidStoredNoExceptMemberFunctionCall0<T, Class> >::type type;
};
template <typename T, typename Class>
class StoredConstNoExceptMemberFunctionCall0 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionCall0(T (Class::*_fn)() const noexcept, const Class &_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        this->result = (object.*fn)();
    }
private:
    T (Class::*fn)() const noexcept;
    const Class object;

};
template <typename T, typename Class>
class VoidStoredConstNoExceptMemberFunctionCall0 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionCall0(T (Class::*_fn)() const noexcept, const Class &_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        (object.*fn)();
    }
private:
    T (Class::*fn)() const noexcept;
    const Class object;

};
template <typename T, typename Class>
struct SelectStoredConstNoExceptMemberFunctionCall0
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionCall0    <T, Class>,
             VoidStoredConstNoExceptMemberFunctionCall0<T, Class> >::type type;
};
template <typename T, typename Class>
class StoredNoExceptMemberFunctionPointerCall0 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionPointerCall0(T (Class::*_fn)() noexcept, Class *_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        this->result = (object->*fn)();
    }
private:
    T (Class::*fn)() noexcept;
    Class *object;

};
template <typename T, typename Class>
class VoidStoredNoExceptMemberFunctionPointerCall0 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionPointerCall0(T (Class::*_fn)() noexcept, Class *_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        (object->*fn)();
    }
private:
    T (Class::*fn)() noexcept;
    Class *object;

};
template <typename T, typename Class>
struct SelectStoredNoExceptMemberFunctionPointerCall0
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionPointerCall0    <T, Class>,
             VoidStoredNoExceptMemberFunctionPointerCall0<T, Class> >::type type;
};
template <typename T, typename Class>
class StoredConstNoExceptMemberFunctionPointerCall0 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionPointerCall0(T (Class::*_fn)() const noexcept, Class const *_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        this->result = (object->*fn)();
    }
private:
    T (Class::*fn)() const noexcept;
    Class const *object;

};
template <typename T, typename Class>
class VoidStoredConstNoExceptMemberFunctionPointerCall0 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionPointerCall0(T (Class::*_fn)() const noexcept, Class const *_object)
    : fn(_fn), object(_object){ }

    void runFunctor() override
    {
        (object->*fn)();
    }
private:
    T (Class::*fn)() const noexcept;
    Class const *object;

};
template <typename T, typename Class>
struct SelectStoredConstNoExceptMemberFunctionPointerCall0
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionPointerCall0    <T, Class>,
             VoidStoredConstNoExceptMemberFunctionPointerCall0<T, Class> >::type type;
};
#endif

template <typename T, typename FunctionPointer, typename Arg1>
struct StoredFunctorCall1: public RunFunctionTask<T>
{
    inline StoredFunctorCall1(FunctionPointer _function, const Arg1 &_arg1)
      : function(_function), arg1(_arg1) {}
    void runFunctor() override { this->result = function(arg1); }
    FunctionPointer function;
    Arg1 arg1;
};

template <typename FunctionPointer, typename Arg1>
struct StoredFunctorCall1<void, FunctionPointer, Arg1>: public RunFunctionTask<void>
{
    inline StoredFunctorCall1(FunctionPointer _function, const Arg1 &_arg1)
      : function(_function), arg1(_arg1) {}
    void runFunctor() override { function(arg1); }
    FunctionPointer function;
    Arg1 arg1;
};

template <typename T, typename FunctionPointer, typename Arg1>
struct StoredFunctorPointerCall1: public RunFunctionTask<T>
{
    inline StoredFunctorPointerCall1(FunctionPointer * _function, const Arg1 &_arg1)
      : function(_function), arg1(_arg1) {}
    void runFunctor() override { this->result =(*function)(arg1); }
    FunctionPointer * function;
    Arg1 arg1;
};

template <typename T, typename FunctionPointer, typename Arg1>
struct VoidStoredFunctorPointerCall1: public RunFunctionTask<T>
{
    inline VoidStoredFunctorPointerCall1(FunctionPointer * _function, const Arg1 &_arg1)
      : function(_function), arg1(_arg1) {}
    void runFunctor() override { (*function)(arg1); }
    FunctionPointer * function;
    Arg1 arg1;
};

template <typename T, typename FunctionPointer, typename Arg1>
struct SelectStoredFunctorPointerCall1
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredFunctorPointerCall1    <T, FunctionPointer, Arg1>,
             VoidStoredFunctorPointerCall1<T, FunctionPointer, Arg1> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class StoredMemberFunctionCall1 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionCall1(T (Class::*_fn)(Param1), const Class &_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1);
    Class object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class VoidStoredMemberFunctionCall1 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionCall1(T (Class::*_fn)(Param1), const Class &_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        (object.*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1);
    Class object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
struct SelectStoredMemberFunctionCall1
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionCall1    <T, Class, Param1, Arg1>,
             VoidStoredMemberFunctionCall1<T, Class, Param1, Arg1> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class StoredConstMemberFunctionCall1 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionCall1(T (Class::*_fn)(Param1) const, const Class &_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) const;
    const Class object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class VoidStoredConstMemberFunctionCall1 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionCall1(T (Class::*_fn)(Param1) const, const Class &_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        (object.*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) const;
    const Class object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
struct SelectStoredConstMemberFunctionCall1
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionCall1    <T, Class, Param1, Arg1>,
             VoidStoredConstMemberFunctionCall1<T, Class, Param1, Arg1> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class StoredMemberFunctionPointerCall1 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionPointerCall1(T (Class::*_fn)(Param1), Class *_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1);
    Class *object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class VoidStoredMemberFunctionPointerCall1 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionPointerCall1(T (Class::*_fn)(Param1), Class *_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        (object->*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1);
    Class *object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
struct SelectStoredMemberFunctionPointerCall1
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionPointerCall1    <T, Class, Param1, Arg1>,
             VoidStoredMemberFunctionPointerCall1<T, Class, Param1, Arg1> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class StoredConstMemberFunctionPointerCall1 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionPointerCall1(T (Class::*_fn)(Param1) const, Class const *_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) const;
    Class const *object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class VoidStoredConstMemberFunctionPointerCall1 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionPointerCall1(T (Class::*_fn)(Param1) const, Class const *_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        (object->*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) const;
    Class const *object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
struct SelectStoredConstMemberFunctionPointerCall1
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionPointerCall1    <T, Class, Param1, Arg1>,
             VoidStoredConstMemberFunctionPointerCall1<T, Class, Param1, Arg1> >::type type;
};
#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510
template <typename T, typename Class, typename Param1, typename Arg1>
class StoredNoExceptMemberFunctionCall1 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionCall1(T (Class::*_fn)(Param1) noexcept, const Class &_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) noexcept;
    Class object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class VoidStoredNoExceptMemberFunctionCall1 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionCall1(T (Class::*_fn)(Param1) noexcept, const Class &_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        (object.*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) noexcept;
    Class object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
struct SelectStoredNoExceptMemberFunctionCall1
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionCall1    <T, Class, Param1, Arg1>,
             VoidStoredNoExceptMemberFunctionCall1<T, Class, Param1, Arg1> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class StoredConstNoExceptMemberFunctionCall1 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionCall1(T (Class::*_fn)(Param1) const noexcept, const Class &_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) const noexcept;
    const Class object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class VoidStoredConstNoExceptMemberFunctionCall1 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionCall1(T (Class::*_fn)(Param1) const noexcept, const Class &_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        (object.*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) const noexcept;
    const Class object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
struct SelectStoredConstNoExceptMemberFunctionCall1
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionCall1    <T, Class, Param1, Arg1>,
             VoidStoredConstNoExceptMemberFunctionCall1<T, Class, Param1, Arg1> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class StoredNoExceptMemberFunctionPointerCall1 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionPointerCall1(T (Class::*_fn)(Param1) noexcept, Class *_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) noexcept;
    Class *object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class VoidStoredNoExceptMemberFunctionPointerCall1 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionPointerCall1(T (Class::*_fn)(Param1) noexcept, Class *_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        (object->*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) noexcept;
    Class *object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
struct SelectStoredNoExceptMemberFunctionPointerCall1
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionPointerCall1    <T, Class, Param1, Arg1>,
             VoidStoredNoExceptMemberFunctionPointerCall1<T, Class, Param1, Arg1> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class StoredConstNoExceptMemberFunctionPointerCall1 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionPointerCall1(T (Class::*_fn)(Param1) const noexcept, Class const *_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) const noexcept;
    Class const *object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
class VoidStoredConstNoExceptMemberFunctionPointerCall1 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionPointerCall1(T (Class::*_fn)(Param1) const noexcept, Class const *_object, const Arg1 &_arg1)
    : fn(_fn), object(_object), arg1(_arg1){ }

    void runFunctor() override
    {
        (object->*fn)(arg1);
    }
private:
    T (Class::*fn)(Param1) const noexcept;
    Class const *object;
    Arg1 arg1;
};
template <typename T, typename Class, typename Param1, typename Arg1>
struct SelectStoredConstNoExceptMemberFunctionPointerCall1
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionPointerCall1    <T, Class, Param1, Arg1>,
             VoidStoredConstNoExceptMemberFunctionPointerCall1<T, Class, Param1, Arg1> >::type type;
};
#endif

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2>
struct StoredFunctorCall2: public RunFunctionTask<T>
{
    inline StoredFunctorCall2(FunctionPointer _function, const Arg1 &_arg1, const Arg2 &_arg2)
      : function(_function), arg1(_arg1), arg2(_arg2) {}
    void runFunctor() override { this->result = function(arg1, arg2); }
    FunctionPointer function;
    Arg1 arg1; Arg2 arg2;
};

template <typename FunctionPointer, typename Arg1, typename Arg2>
struct StoredFunctorCall2<void, FunctionPointer, Arg1, Arg2>: public RunFunctionTask<void>
{
    inline StoredFunctorCall2(FunctionPointer _function, const Arg1 &_arg1, const Arg2 &_arg2)
      : function(_function), arg1(_arg1), arg2(_arg2) {}
    void runFunctor() override { function(arg1, arg2); }
    FunctionPointer function;
    Arg1 arg1; Arg2 arg2;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2>
struct StoredFunctorPointerCall2: public RunFunctionTask<T>
{
    inline StoredFunctorPointerCall2(FunctionPointer * _function, const Arg1 &_arg1, const Arg2 &_arg2)
      : function(_function), arg1(_arg1), arg2(_arg2) {}
    void runFunctor() override { this->result =(*function)(arg1, arg2); }
    FunctionPointer * function;
    Arg1 arg1; Arg2 arg2;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2>
struct VoidStoredFunctorPointerCall2: public RunFunctionTask<T>
{
    inline VoidStoredFunctorPointerCall2(FunctionPointer * _function, const Arg1 &_arg1, const Arg2 &_arg2)
      : function(_function), arg1(_arg1), arg2(_arg2) {}
    void runFunctor() override { (*function)(arg1, arg2); }
    FunctionPointer * function;
    Arg1 arg1; Arg2 arg2;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2>
struct SelectStoredFunctorPointerCall2
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredFunctorPointerCall2    <T, FunctionPointer, Arg1, Arg2>,
             VoidStoredFunctorPointerCall2<T, FunctionPointer, Arg1, Arg2> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class StoredMemberFunctionCall2 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionCall2(T (Class::*_fn)(Param1, Param2), const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2);
    Class object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class VoidStoredMemberFunctionCall2 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionCall2(T (Class::*_fn)(Param1, Param2), const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2);
    Class object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
struct SelectStoredMemberFunctionCall2
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionCall2    <T, Class, Param1, Arg1, Param2, Arg2>,
             VoidStoredMemberFunctionCall2<T, Class, Param1, Arg1, Param2, Arg2> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class StoredConstMemberFunctionCall2 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionCall2(T (Class::*_fn)(Param1, Param2) const, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) const;
    const Class object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class VoidStoredConstMemberFunctionCall2 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionCall2(T (Class::*_fn)(Param1, Param2) const, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) const;
    const Class object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
struct SelectStoredConstMemberFunctionCall2
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionCall2    <T, Class, Param1, Arg1, Param2, Arg2>,
             VoidStoredConstMemberFunctionCall2<T, Class, Param1, Arg1, Param2, Arg2> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class StoredMemberFunctionPointerCall2 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionPointerCall2(T (Class::*_fn)(Param1, Param2), Class *_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2);
    Class *object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class VoidStoredMemberFunctionPointerCall2 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionPointerCall2(T (Class::*_fn)(Param1, Param2), Class *_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2);
    Class *object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
struct SelectStoredMemberFunctionPointerCall2
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionPointerCall2    <T, Class, Param1, Arg1, Param2, Arg2>,
             VoidStoredMemberFunctionPointerCall2<T, Class, Param1, Arg1, Param2, Arg2> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class StoredConstMemberFunctionPointerCall2 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionPointerCall2(T (Class::*_fn)(Param1, Param2) const, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) const;
    Class const *object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class VoidStoredConstMemberFunctionPointerCall2 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionPointerCall2(T (Class::*_fn)(Param1, Param2) const, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) const;
    Class const *object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
struct SelectStoredConstMemberFunctionPointerCall2
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionPointerCall2    <T, Class, Param1, Arg1, Param2, Arg2>,
             VoidStoredConstMemberFunctionPointerCall2<T, Class, Param1, Arg1, Param2, Arg2> >::type type;
};
#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class StoredNoExceptMemberFunctionCall2 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionCall2(T (Class::*_fn)(Param1, Param2) noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) noexcept;
    Class object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class VoidStoredNoExceptMemberFunctionCall2 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionCall2(T (Class::*_fn)(Param1, Param2) noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) noexcept;
    Class object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
struct SelectStoredNoExceptMemberFunctionCall2
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionCall2    <T, Class, Param1, Arg1, Param2, Arg2>,
             VoidStoredNoExceptMemberFunctionCall2<T, Class, Param1, Arg1, Param2, Arg2> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class StoredConstNoExceptMemberFunctionCall2 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionCall2(T (Class::*_fn)(Param1, Param2) const noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) const noexcept;
    const Class object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class VoidStoredConstNoExceptMemberFunctionCall2 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionCall2(T (Class::*_fn)(Param1, Param2) const noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) const noexcept;
    const Class object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
struct SelectStoredConstNoExceptMemberFunctionCall2
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionCall2    <T, Class, Param1, Arg1, Param2, Arg2>,
             VoidStoredConstNoExceptMemberFunctionCall2<T, Class, Param1, Arg1, Param2, Arg2> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class StoredNoExceptMemberFunctionPointerCall2 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionPointerCall2(T (Class::*_fn)(Param1, Param2) noexcept, Class *_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) noexcept;
    Class *object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class VoidStoredNoExceptMemberFunctionPointerCall2 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionPointerCall2(T (Class::*_fn)(Param1, Param2) noexcept, Class *_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) noexcept;
    Class *object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
struct SelectStoredNoExceptMemberFunctionPointerCall2
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionPointerCall2    <T, Class, Param1, Arg1, Param2, Arg2>,
             VoidStoredNoExceptMemberFunctionPointerCall2<T, Class, Param1, Arg1, Param2, Arg2> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class StoredConstNoExceptMemberFunctionPointerCall2 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionPointerCall2(T (Class::*_fn)(Param1, Param2) const noexcept, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) const noexcept;
    Class const *object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
class VoidStoredConstNoExceptMemberFunctionPointerCall2 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionPointerCall2(T (Class::*_fn)(Param1, Param2) const noexcept, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2);
    }
private:
    T (Class::*fn)(Param1, Param2) const noexcept;
    Class const *object;
    Arg1 arg1; Arg2 arg2;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2>
struct SelectStoredConstNoExceptMemberFunctionPointerCall2
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionPointerCall2    <T, Class, Param1, Arg1, Param2, Arg2>,
             VoidStoredConstNoExceptMemberFunctionPointerCall2<T, Class, Param1, Arg1, Param2, Arg2> >::type type;
};
#endif

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3>
struct StoredFunctorCall3: public RunFunctionTask<T>
{
    inline StoredFunctorCall3(FunctionPointer _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3) {}
    void runFunctor() override { this->result = function(arg1, arg2, arg3); }
    FunctionPointer function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};

template <typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3>
struct StoredFunctorCall3<void, FunctionPointer, Arg1, Arg2, Arg3>: public RunFunctionTask<void>
{
    inline StoredFunctorCall3(FunctionPointer _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3) {}
    void runFunctor() override { function(arg1, arg2, arg3); }
    FunctionPointer function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3>
struct StoredFunctorPointerCall3: public RunFunctionTask<T>
{
    inline StoredFunctorPointerCall3(FunctionPointer * _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3) {}
    void runFunctor() override { this->result =(*function)(arg1, arg2, arg3); }
    FunctionPointer * function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3>
struct VoidStoredFunctorPointerCall3: public RunFunctionTask<T>
{
    inline VoidStoredFunctorPointerCall3(FunctionPointer * _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3) {}
    void runFunctor() override { (*function)(arg1, arg2, arg3); }
    FunctionPointer * function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3>
struct SelectStoredFunctorPointerCall3
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredFunctorPointerCall3    <T, FunctionPointer, Arg1, Arg2, Arg3>,
             VoidStoredFunctorPointerCall3<T, FunctionPointer, Arg1, Arg2, Arg3> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class StoredMemberFunctionCall3 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionCall3(T (Class::*_fn)(Param1, Param2, Param3), const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3);
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class VoidStoredMemberFunctionCall3 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionCall3(T (Class::*_fn)(Param1, Param2, Param3), const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3);
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
struct SelectStoredMemberFunctionCall3
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionCall3    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3>,
             VoidStoredMemberFunctionCall3<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class StoredConstMemberFunctionCall3 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionCall3(T (Class::*_fn)(Param1, Param2, Param3) const, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) const;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class VoidStoredConstMemberFunctionCall3 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionCall3(T (Class::*_fn)(Param1, Param2, Param3) const, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) const;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
struct SelectStoredConstMemberFunctionCall3
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionCall3    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3>,
             VoidStoredConstMemberFunctionCall3<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class StoredMemberFunctionPointerCall3 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionPointerCall3(T (Class::*_fn)(Param1, Param2, Param3), Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3);
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class VoidStoredMemberFunctionPointerCall3 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionPointerCall3(T (Class::*_fn)(Param1, Param2, Param3), Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3);
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
struct SelectStoredMemberFunctionPointerCall3
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionPointerCall3    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3>,
             VoidStoredMemberFunctionPointerCall3<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class StoredConstMemberFunctionPointerCall3 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionPointerCall3(T (Class::*_fn)(Param1, Param2, Param3) const, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) const;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class VoidStoredConstMemberFunctionPointerCall3 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionPointerCall3(T (Class::*_fn)(Param1, Param2, Param3) const, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) const;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
struct SelectStoredConstMemberFunctionPointerCall3
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionPointerCall3    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3>,
             VoidStoredConstMemberFunctionPointerCall3<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3> >::type type;
};
#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class StoredNoExceptMemberFunctionCall3 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionCall3(T (Class::*_fn)(Param1, Param2, Param3) noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) noexcept;
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class VoidStoredNoExceptMemberFunctionCall3 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionCall3(T (Class::*_fn)(Param1, Param2, Param3) noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) noexcept;
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
struct SelectStoredNoExceptMemberFunctionCall3
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionCall3    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3>,
             VoidStoredNoExceptMemberFunctionCall3<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class StoredConstNoExceptMemberFunctionCall3 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionCall3(T (Class::*_fn)(Param1, Param2, Param3) const noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) const noexcept;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class VoidStoredConstNoExceptMemberFunctionCall3 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionCall3(T (Class::*_fn)(Param1, Param2, Param3) const noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) const noexcept;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
struct SelectStoredConstNoExceptMemberFunctionCall3
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionCall3    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3>,
             VoidStoredConstNoExceptMemberFunctionCall3<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class StoredNoExceptMemberFunctionPointerCall3 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionPointerCall3(T (Class::*_fn)(Param1, Param2, Param3) noexcept, Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) noexcept;
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class VoidStoredNoExceptMemberFunctionPointerCall3 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionPointerCall3(T (Class::*_fn)(Param1, Param2, Param3) noexcept, Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) noexcept;
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
struct SelectStoredNoExceptMemberFunctionPointerCall3
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionPointerCall3    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3>,
             VoidStoredNoExceptMemberFunctionPointerCall3<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class StoredConstNoExceptMemberFunctionPointerCall3 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionPointerCall3(T (Class::*_fn)(Param1, Param2, Param3) const noexcept, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) const noexcept;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
class VoidStoredConstNoExceptMemberFunctionPointerCall3 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionPointerCall3(T (Class::*_fn)(Param1, Param2, Param3) const noexcept, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3) const noexcept;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3>
struct SelectStoredConstNoExceptMemberFunctionPointerCall3
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionPointerCall3    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3>,
             VoidStoredConstNoExceptMemberFunctionPointerCall3<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3> >::type type;
};
#endif

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4>
struct StoredFunctorCall4: public RunFunctionTask<T>
{
    inline StoredFunctorCall4(FunctionPointer _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4) {}
    void runFunctor() override { this->result = function(arg1, arg2, arg3, arg4); }
    FunctionPointer function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};

template <typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4>
struct StoredFunctorCall4<void, FunctionPointer, Arg1, Arg2, Arg3, Arg4>: public RunFunctionTask<void>
{
    inline StoredFunctorCall4(FunctionPointer _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4) {}
    void runFunctor() override { function(arg1, arg2, arg3, arg4); }
    FunctionPointer function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4>
struct StoredFunctorPointerCall4: public RunFunctionTask<T>
{
    inline StoredFunctorPointerCall4(FunctionPointer * _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4) {}
    void runFunctor() override { this->result =(*function)(arg1, arg2, arg3, arg4); }
    FunctionPointer * function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4>
struct VoidStoredFunctorPointerCall4: public RunFunctionTask<T>
{
    inline VoidStoredFunctorPointerCall4(FunctionPointer * _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4) {}
    void runFunctor() override { (*function)(arg1, arg2, arg3, arg4); }
    FunctionPointer * function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4>
struct SelectStoredFunctorPointerCall4
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredFunctorPointerCall4    <T, FunctionPointer, Arg1, Arg2, Arg3, Arg4>,
             VoidStoredFunctorPointerCall4<T, FunctionPointer, Arg1, Arg2, Arg3, Arg4> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class StoredMemberFunctionCall4 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4), const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4);
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class VoidStoredMemberFunctionCall4 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4), const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4);
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
struct SelectStoredMemberFunctionCall4
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionCall4    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4>,
             VoidStoredMemberFunctionCall4<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class StoredConstMemberFunctionCall4 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) const, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) const;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class VoidStoredConstMemberFunctionCall4 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) const, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) const;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
struct SelectStoredConstMemberFunctionCall4
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionCall4    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4>,
             VoidStoredConstMemberFunctionCall4<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class StoredMemberFunctionPointerCall4 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionPointerCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4), Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4);
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class VoidStoredMemberFunctionPointerCall4 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionPointerCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4), Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4);
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
struct SelectStoredMemberFunctionPointerCall4
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionPointerCall4    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4>,
             VoidStoredMemberFunctionPointerCall4<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class StoredConstMemberFunctionPointerCall4 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionPointerCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) const, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) const;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class VoidStoredConstMemberFunctionPointerCall4 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionPointerCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) const, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) const;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
struct SelectStoredConstMemberFunctionPointerCall4
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionPointerCall4    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4>,
             VoidStoredConstMemberFunctionPointerCall4<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4> >::type type;
};
#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class StoredNoExceptMemberFunctionCall4 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) noexcept;
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class VoidStoredNoExceptMemberFunctionCall4 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) noexcept;
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
struct SelectStoredNoExceptMemberFunctionCall4
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionCall4    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4>,
             VoidStoredNoExceptMemberFunctionCall4<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class StoredConstNoExceptMemberFunctionCall4 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) const noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) const noexcept;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class VoidStoredConstNoExceptMemberFunctionCall4 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) const noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) const noexcept;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
struct SelectStoredConstNoExceptMemberFunctionCall4
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionCall4    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4>,
             VoidStoredConstNoExceptMemberFunctionCall4<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class StoredNoExceptMemberFunctionPointerCall4 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionPointerCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) noexcept, Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) noexcept;
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class VoidStoredNoExceptMemberFunctionPointerCall4 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionPointerCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) noexcept, Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) noexcept;
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
struct SelectStoredNoExceptMemberFunctionPointerCall4
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionPointerCall4    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4>,
             VoidStoredNoExceptMemberFunctionPointerCall4<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class StoredConstNoExceptMemberFunctionPointerCall4 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionPointerCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) const noexcept, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) const noexcept;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
class VoidStoredConstNoExceptMemberFunctionPointerCall4 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionPointerCall4(T (Class::*_fn)(Param1, Param2, Param3, Param4) const noexcept, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3, arg4);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4) const noexcept;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4>
struct SelectStoredConstNoExceptMemberFunctionPointerCall4
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionPointerCall4    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4>,
             VoidStoredConstNoExceptMemberFunctionPointerCall4<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4> >::type type;
};
#endif

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4, typename Arg5>
struct StoredFunctorCall5: public RunFunctionTask<T>
{
    inline StoredFunctorCall5(FunctionPointer _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5) {}
    void runFunctor() override { this->result = function(arg1, arg2, arg3, arg4, arg5); }
    FunctionPointer function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};

template <typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4, typename Arg5>
struct StoredFunctorCall5<void, FunctionPointer, Arg1, Arg2, Arg3, Arg4, Arg5>: public RunFunctionTask<void>
{
    inline StoredFunctorCall5(FunctionPointer _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5) {}
    void runFunctor() override { function(arg1, arg2, arg3, arg4, arg5); }
    FunctionPointer function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4, typename Arg5>
struct StoredFunctorPointerCall5: public RunFunctionTask<T>
{
    inline StoredFunctorPointerCall5(FunctionPointer * _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5) {}
    void runFunctor() override { this->result =(*function)(arg1, arg2, arg3, arg4, arg5); }
    FunctionPointer * function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4, typename Arg5>
struct VoidStoredFunctorPointerCall5: public RunFunctionTask<T>
{
    inline VoidStoredFunctorPointerCall5(FunctionPointer * _function, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
      : function(_function), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5) {}
    void runFunctor() override { (*function)(arg1, arg2, arg3, arg4, arg5); }
    FunctionPointer * function;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};

template <typename T, typename FunctionPointer, typename Arg1, typename Arg2, typename Arg3, typename Arg4, typename Arg5>
struct SelectStoredFunctorPointerCall5
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredFunctorPointerCall5    <T, FunctionPointer, Arg1, Arg2, Arg3, Arg4, Arg5>,
             VoidStoredFunctorPointerCall5<T, FunctionPointer, Arg1, Arg2, Arg3, Arg4, Arg5> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class StoredMemberFunctionCall5 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5), const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5);
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class VoidStoredMemberFunctionCall5 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5), const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5);
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
struct SelectStoredMemberFunctionCall5
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionCall5    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5>,
             VoidStoredMemberFunctionCall5<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class StoredConstMemberFunctionCall5 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) const, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) const;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class VoidStoredConstMemberFunctionCall5 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) const, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) const;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
struct SelectStoredConstMemberFunctionCall5
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionCall5    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5>,
             VoidStoredConstMemberFunctionCall5<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class StoredMemberFunctionPointerCall5 : public RunFunctionTask<T>
{
public:
    StoredMemberFunctionPointerCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5), Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5);
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class VoidStoredMemberFunctionPointerCall5 : public RunFunctionTask<T>
{
public:
    VoidStoredMemberFunctionPointerCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5), Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5);
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
struct SelectStoredMemberFunctionPointerCall5
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredMemberFunctionPointerCall5    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5>,
             VoidStoredMemberFunctionPointerCall5<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class StoredConstMemberFunctionPointerCall5 : public RunFunctionTask<T>
{
public:
    StoredConstMemberFunctionPointerCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) const, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) const;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class VoidStoredConstMemberFunctionPointerCall5 : public RunFunctionTask<T>
{
public:
    VoidStoredConstMemberFunctionPointerCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) const, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) const;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
struct SelectStoredConstMemberFunctionPointerCall5
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstMemberFunctionPointerCall5    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5>,
             VoidStoredConstMemberFunctionPointerCall5<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5> >::type type;
};
#if defined(__cpp_noexcept_function_type) && __cpp_noexcept_function_type >= 201510
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class StoredNoExceptMemberFunctionCall5 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) noexcept;
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class VoidStoredNoExceptMemberFunctionCall5 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) noexcept;
    Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
struct SelectStoredNoExceptMemberFunctionCall5
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionCall5    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5>,
             VoidStoredNoExceptMemberFunctionCall5<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class StoredConstNoExceptMemberFunctionCall5 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) const noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        this->result = (object.*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) const noexcept;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class VoidStoredConstNoExceptMemberFunctionCall5 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) const noexcept, const Class &_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        (object.*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) const noexcept;
    const Class object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
struct SelectStoredConstNoExceptMemberFunctionCall5
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionCall5    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5>,
             VoidStoredConstNoExceptMemberFunctionCall5<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class StoredNoExceptMemberFunctionPointerCall5 : public RunFunctionTask<T>
{
public:
    StoredNoExceptMemberFunctionPointerCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) noexcept, Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) noexcept;
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class VoidStoredNoExceptMemberFunctionPointerCall5 : public RunFunctionTask<T>
{
public:
    VoidStoredNoExceptMemberFunctionPointerCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) noexcept, Class *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) noexcept;
    Class *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
struct SelectStoredNoExceptMemberFunctionPointerCall5
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredNoExceptMemberFunctionPointerCall5    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5>,
             VoidStoredNoExceptMemberFunctionPointerCall5<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5> >::type type;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class StoredConstNoExceptMemberFunctionPointerCall5 : public RunFunctionTask<T>
{
public:
    StoredConstNoExceptMemberFunctionPointerCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) const noexcept, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        this->result = (object->*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) const noexcept;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
class VoidStoredConstNoExceptMemberFunctionPointerCall5 : public RunFunctionTask<T>
{
public:
    VoidStoredConstNoExceptMemberFunctionPointerCall5(T (Class::*_fn)(Param1, Param2, Param3, Param4, Param5) const noexcept, Class const *_object, const Arg1 &_arg1, const Arg2 &_arg2, const Arg3 &_arg3, const Arg4 &_arg4, const Arg5 &_arg5)
    : fn(_fn), object(_object), arg1(_arg1), arg2(_arg2), arg3(_arg3), arg4(_arg4), arg5(_arg5){ }

    void runFunctor() override
    {
        (object->*fn)(arg1, arg2, arg3, arg4, arg5);
    }
private:
    T (Class::*fn)(Param1, Param2, Param3, Param4, Param5) const noexcept;
    Class const *object;
    Arg1 arg1; Arg2 arg2; Arg3 arg3; Arg4 arg4; Arg5 arg5;
};
template <typename T, typename Class, typename Param1, typename Arg1, typename Param2, typename Arg2, typename Param3, typename Arg3, typename Param4, typename Arg4, typename Param5, typename Arg5>
struct SelectStoredConstNoExceptMemberFunctionPointerCall5
{
    typedef typename SelectSpecialization<T>::template
        Type<StoredConstNoExceptMemberFunctionPointerCall5    <T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5>,
             VoidStoredConstNoExceptMemberFunctionPointerCall5<T, Class, Param1, Arg1, Param2, Arg2, Param3, Arg3, Param4, Arg4, Param5, Arg5> >::type type;
};
#endif

template <typename T, typename Functor>
class StoredFunctorCall : public RunFunctionTask<T>
{
public:
    StoredFunctorCall(const Functor &f) : functor(f) { }
    void runFunctor() override
    {
        this->result = functor();
    }
private:
    Functor functor;
};
template <typename Functor>
class StoredFunctorCall<void, Functor> : public RunFunctionTask<void>
{
public:
    StoredFunctorCall(const Functor &f) : functor(f) { }
    void runFunctor() override
    {
        functor();
    }
private:
    Functor functor;
};


} //namespace QtConcurrent

#endif // Q_QDOC

QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
