//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef INVOKE_HELPERS_H
#define INVOKE_HELPERS_H

#include <type_traits>
#include <cassert>
#include <functional>

#include "test_macros.h"

template <int I>
struct Int : public std::integral_constant<int, I> {};

template <bool P>
struct Bool : public std::integral_constant<bool, P> {};

struct Q_None {
    template <class T>
    struct apply { typedef T type; };
};

struct Q_Const {
    template <class T>
    struct apply { typedef T const type; };
};

struct Q_Volatile {
    template <class T>
    struct apply { typedef T volatile type; };
};

struct Q_CV {
    template <class T>
    struct apply { typedef T const volatile type; };
};

// Caster - A functor object that performs cv-qualifier and value category
//   conversions.
//   QualTag - A metafunction type that applies cv-qualifiers to its argument.
//   RValue - True if the resulting object should be an RValue reference.
//            False otherwise.
template <class QualTag, bool RValue = false>
struct Caster {
    template <class T>
    struct apply {
        typedef typename std::remove_reference<T>::type RawType;
        typedef typename QualTag::template apply<RawType>::type CVType;
#if TEST_STD_VER >= 11
        typedef typename std::conditional<RValue,
            CVType&&, CVType&
        >::type type;
#else
        typedef CVType& type;
#endif
    };

    template <class T>
    typename apply<T>::type
    operator()(T& obj) const {
        typedef typename apply<T>::type OutType;
        return static_cast<OutType>(obj);
    }
};

typedef Caster<Q_None>           LValueCaster;
typedef Caster<Q_Const>          ConstCaster;
typedef Caster<Q_Volatile>       VolatileCaster;
typedef Caster<Q_CV>             CVCaster;
typedef Caster<Q_None,     true> MoveCaster;
typedef Caster<Q_Const,    true> MoveConstCaster;
typedef Caster<Q_Volatile, true> MoveVolatileCaster;
typedef Caster<Q_CV,       true> MoveCVCaster;


template <class Tp>
Tp const& makeConst(Tp& ref) { return ref; }

template <class Tp>
Tp const* makeConst(Tp* ptr) { return ptr; }

template <class Tp>
std::reference_wrapper<const Tp> makeConst(std::reference_wrapper<Tp>& ref) {
    return std::reference_wrapper<const Tp>(ref.get());
}

template <class Tp>
Tp volatile& makeVolatile(Tp& ref) { return ref; }

template <class Tp>
Tp volatile* makeVolatile(Tp* ptr) { return ptr; }

template <class Tp>
std::reference_wrapper<volatile Tp> makeVolatile(std::reference_wrapper<Tp>& ref) {
    return std::reference_wrapper<volatile Tp>(ref.get());
}

template <class Tp>
Tp const volatile& makeCV(Tp& ref) { return ref; }

template <class Tp>
Tp const volatile* makeCV(Tp* ptr) { return ptr; }

template <class Tp>
std::reference_wrapper<const volatile Tp> makeCV(std::reference_wrapper<Tp>& ref) {
    return std::reference_wrapper<const volatile Tp>(ref.get());
}

// A shorter name for 'static_cast'
template <class QualType, class Tp>
QualType C_(Tp& v) { return static_cast<QualType>(v); };

//==============================================================================
// ArgType - A non-copyable type intended to be used as a dummy argument type
//   to test functions.
struct ArgType {
    int value;
    explicit ArgType(int val = 0) : value(val) {}
private:
    ArgType(ArgType const&);
    ArgType& operator=(ArgType const&);
};

//==============================================================================
// DerivedFromBase - A type that derives from its template argument 'Base'
template <class Base>
struct DerivedFromType : public Base {
    DerivedFromType() : Base() {}
    template <class Tp>
    explicit DerivedFromType(Tp const& t) : Base(t) {}
};

//==============================================================================
// DerefToType - A type that dereferences to its template argument 'To'.
//   The cv-ref qualifiers of the 'DerefToType' object do not propagate
//   to the resulting 'To' object.
template <class To>
struct DerefToType {
    To object;

    DerefToType() {}

    template <class Up>
    explicit DerefToType(Up const& val) : object(val) {}

    To& operator*() const volatile { return const_cast<To&>(object); }
};

//==============================================================================
// DerefPropToType - A type that dereferences to its template argument 'To'.
//   The cv-ref qualifiers of the 'DerefPropToType' object propagate
//   to the resulting 'To' object.
template <class To>
struct DerefPropType {
    To object;

    DerefPropType() {}

    template <class Up>
    explicit DerefPropType(Up const& val) : object(val) {}

#if TEST_STD_VER < 11
    To& operator*() { return object; }
    To const& operator*() const { return object; }
    To volatile& operator*() volatile  { return object; }
    To const volatile& operator*() const volatile { return object; }
#else
    To& operator*() & { return object; }
    To const& operator*() const & { return object; }
    To volatile& operator*() volatile  & { return object; }
    To const volatile& operator*() const volatile & { return object; }
    To&& operator*() && { return static_cast<To &&>(object); }
    To const&& operator*() const && { return static_cast<To const&&>(object); }
    To volatile&& operator*() volatile  && { return static_cast<To volatile&&>(object); }
    To const volatile&& operator*() const volatile && { return static_cast<To const volatile&&>(object); }
#endif
};

//==============================================================================
// MethodID - A type that uniquely identifies a member function for a class.
//   This type is used to communicate between the member functions being tested
//   and the tests invoking them.
// - Test methods should call 'setUncheckedCall()' whenever they are invoked.
// - Tests consume the unchecked call using checkCall(<return-value>)` to assert
//   that the method has been called and that the return value of `__invoke`
//   matches what the method actually returned.
template <class T>
struct MethodID {
    typedef void* IDType;

    static int dummy; // A dummy memory location.
    static void* id; // The "ID" is the value of this pointer.
    static bool unchecked_call; // Has a call happened that has not been checked.

    static void*& setUncheckedCall() {
        assert(unchecked_call == false);
        unchecked_call = true;
        return id;
    }

    static bool checkCalled(void*& return_value) {
        bool old = unchecked_call;
        unchecked_call = false;
        return old && id == return_value && &id == &return_value;
    }
};

template <class T> int   MethodID<T>::dummy = 0;
template <class T> void* MethodID<T>::id = (void*)&MethodID<T>::dummy;
template <class T> bool  MethodID<T>::unchecked_call = false;


//==============================================================================
// FunctionPtrID - Like MethodID but for free function pointers.
template <class T, T*>
struct FunctionPtrID {
    static int dummy; // A dummy memory location.
    static void* id; // The "ID" is the value of this pointer.
    static bool unchecked_call; // Has a call happened that has not been checked.

    static void*& setUncheckedCall() {
        assert(unchecked_call == false);
        unchecked_call = true;
        return id;
    }

    static bool checkCalled(void*& return_value) {
        bool old = unchecked_call;
        unchecked_call = false;
        return old && id == return_value && &id == &return_value;
    }
};

template <class T, T* Ptr> int   FunctionPtrID<T, Ptr>::dummy = 0;
template <class T, T* Ptr> void* FunctionPtrID<T, Ptr>::id = (void*)&FunctionPtrID<T, Ptr>::dummy;
template <class T, T* Ptr> bool  FunctionPtrID<T, Ptr>::unchecked_call = false;

//==============================================================================
// BasicTest - The basic test structure for everything except
// member object pointers.
// ID - The "Function Identifier" type used either MethodID or FunctionPtrID.
// Arity - The Arity of the call signature.
// ObjectCaster - The object transformation functor type.
// ArgCaster - The extra argument transformation functor type.
template <class ID, int Arity, class ObjectCaster = LValueCaster,
                               class ArgCaster    = LValueCaster>
struct BasicTest {
    template <class ObjectT>
    void runTest(ObjectT& object) {
        Int<Arity> A;
        runTestImp(A, object);
    }

    template <class MethodPtr, class ObjectT>
    void runTest(MethodPtr ptr, ObjectT& object) {
        Int<Arity> A;
        runTestImp(A, ptr, object);
    }

private:
    typedef void*& CallRet;
    ObjectCaster object_cast;
    ArgCaster arg_cast;
    ArgType a0, a1, a2;

    //==========================================================================
    //                       BULLET 1, 2 AND 3 TEST METHODS
    //==========================================================================
    template <class MethodPtr, class ObjectT>
    void runTestImp(Int<0>, MethodPtr ptr, ObjectT& object) {
        {
            static_assert((std::is_same<
                decltype(std::__invoke(ptr, object_cast(object)))
              , CallRet>::value), "");
            assert(ID::unchecked_call == false);
            CallRet ret = std::__invoke(ptr, object_cast(object));
            assert(ID::checkCalled(ret));
        }
    }

    template <class MethodPtr, class ObjectT>
    void runTestImp(Int<1>, MethodPtr ptr, ObjectT& object) {
        {
            static_assert((std::is_same<
                decltype(std::__invoke(ptr, object_cast(object), arg_cast(a0)))
              , CallRet>::value), "");
            assert(ID::unchecked_call == false);
            CallRet ret = std::__invoke(ptr, object_cast(object), arg_cast(a0));
            assert(ID::checkCalled(ret));
        }
    }

    template <class MethodPtr, class ObjectT>
    void runTestImp(Int<2>, MethodPtr ptr, ObjectT& object) {
        {
            static_assert((std::is_same<
                decltype(std::__invoke(ptr, object_cast(object), arg_cast(a0), arg_cast(a1)))
              , CallRet>::value), "");
            assert(ID::unchecked_call == false);
            CallRet ret = std::__invoke(ptr, object_cast(object), arg_cast(a0), arg_cast(a1));
            assert(ID::checkCalled(ret));
        }
    }

    template <class MethodPtr, class ObjectT>
    void runTestImp(Int<3>, MethodPtr ptr, ObjectT& object) {
        {
            static_assert((std::is_same<
                decltype(std::__invoke(ptr, object_cast(object), arg_cast(a0), arg_cast(a1), arg_cast(a2)))
              , CallRet>::value), "");
            assert(ID::unchecked_call == false);
            CallRet ret = std::__invoke(ptr, object_cast(object), arg_cast(a0), arg_cast(a1), arg_cast(a2));
            assert(ID::checkCalled(ret));
        }
    }

    //==========================================================================
    //                       BULLET 7 TEST METHODS
    //==========================================================================
    template <class ObjectT>
    void runTestImp(Int<0>, ObjectT& object) {
        {
            static_assert((std::is_same<
                decltype(std::__invoke(object_cast(object)))
              , CallRet>::value), "");
            assert(ID::unchecked_call == false);
            CallRet ret = std::__invoke(object_cast(object));
            assert(ID::checkCalled(ret));
        }
    }

    template <class ObjectT>
    void runTestImp(Int<1>, ObjectT& object) {
        {
            static_assert((std::is_same<
                decltype(std::__invoke(object_cast(object), arg_cast(a0)))
              , CallRet>::value), "");
            assert(ID::unchecked_call == false);
            CallRet ret = std::__invoke(object_cast(object), arg_cast(a0));
            assert(ID::checkCalled(ret));
        }
    }

    template <class ObjectT>
    void runTestImp(Int<2>, ObjectT& object) {
        {
            static_assert((std::is_same<
                decltype(std::__invoke(object_cast(object), arg_cast(a0), arg_cast(a1)))
              , CallRet>::value), "");
            assert(ID::unchecked_call == false);
            CallRet ret = std::__invoke(object_cast(object), arg_cast(a0), arg_cast(a1));
            assert(ID::checkCalled(ret));
        }
    }

    template <class ObjectT>
    void runTestImp(Int<3>, ObjectT& object) {
        {
            static_assert((std::is_same<
                decltype(std::__invoke(object_cast(object), arg_cast(a0), arg_cast(a1), arg_cast(a2)))
              , CallRet>::value), "");
            assert(ID::unchecked_call == false);
            CallRet ret = std::__invoke(object_cast(object), arg_cast(a0), arg_cast(a1), arg_cast(a2));
            assert(ID::checkCalled(ret));
        }
    }
};

#endif // INVOKE_HELPERS_H
