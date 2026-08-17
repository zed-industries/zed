// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2013 Olivier Goffart <ogoffart@woboq.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QOBJECTDEFS_H
#error Do not include qobjectdefs_impl.h directly
#include <QtCore/qnamespace.h>
#endif

#if 0
#pragma qt_sync_skip_header_check
#pragma qt_sync_stop_processing
#endif

QT_BEGIN_NAMESPACE
class QObject;
class QObjectPrivate;

namespace QtPrivate {
    template <typename T> struct RemoveRef { typedef T Type; };
    template <typename T> struct RemoveRef<T&> { typedef T Type; };
    template <typename T> struct RemoveConstRef { typedef T Type; };
    template <typename T> struct RemoveConstRef<const T&> { typedef T Type; };

    /*
       The following List classes are used to help to handle the list of arguments.
       It follow the same principles as the lisp lists.
       List_Left<L,N> take a list and a number as a parameter and returns (via the Value typedef,
       the list composed of the first N element of the list
     */
    // With variadic template, lists are represented using a variadic template argument instead of the lisp way
    template <typename...> struct List {};
    template <typename Head, typename... Tail> struct List<Head, Tail...> { typedef Head Car; typedef List<Tail...> Cdr; };
    template <typename, typename> struct List_Append;
    template <typename... L1, typename...L2> struct List_Append<List<L1...>, List<L2...>> { typedef List<L1..., L2...> Value; };
    template <typename L, int N> struct List_Left {
        typedef typename List_Append<List<typename L::Car>,typename List_Left<typename L::Cdr, N - 1>::Value>::Value Value;
    };
    template <typename L> struct List_Left<L, 0> { typedef List<> Value; };
    // List_Select<L,N> returns (via typedef Value) the Nth element of the list L
    template <typename L, int N> struct List_Select { typedef typename List_Select<typename L::Cdr, N - 1>::Value Value; };
    template <typename L> struct List_Select<L,0> { typedef typename L::Car Value; };

    /*
       trick to set the return value of a slot that works even if the signal or the slot returns void
       to be used like     function(), ApplyReturnValue<ReturnType>(&return_value)
       if function() returns a value, the operator,(T, ApplyReturnValue<ReturnType>) is called, but if it
       returns void, the builtin one is used without an error.
    */
    template <typename T>
    struct ApplyReturnValue {
        void *data;
        explicit ApplyReturnValue(void *data_) : data(data_) {}
    };
    template<typename T, typename U>
    void operator,(T &&value, const ApplyReturnValue<U> &container) {
        if (container.data)
            *reinterpret_cast<U *>(container.data) = std::forward<T>(value);
    }
    template<typename T>
    void operator,(T, const ApplyReturnValue<void> &) {}


    /*
      The FunctionPointer<Func> struct is a type trait for function pointer.
        - ArgumentCount  is the number of argument, or -1 if it is unknown
        - the Object typedef is the Object of a pointer to member function
        - the Arguments typedef is the list of argument (in a QtPrivate::List)
        - the Function typedef is an alias to the template parameter Func
        - the call<Args, R>(f,o,args) method is used to call that slot
            Args is the list of argument of the signal
            R is the return type of the signal
            f is the function pointer
            o is the receiver object
            and args is the array of pointer to arguments, as used in qt_metacall

       The Functor<Func,N> struct is the helper to call a functor of N argument.
       its call function is the same as the FunctionPointer::call function.
     */
    template<class T> using InvokeGenSeq = typename T::Type;

    template<int...> struct IndexesList { using Type = IndexesList; };

    template<int N, class S1, class S2> struct ConcatSeqImpl;

    template<int N, int... I1, int... I2>
    struct ConcatSeqImpl<N, IndexesList<I1...>, IndexesList<I2...>>
        : IndexesList<I1..., (N + I2)...>{};

    template<int N, class S1, class S2>
    using ConcatSeq = InvokeGenSeq<ConcatSeqImpl<N, S1, S2>>;

    template<int N> struct GenSeq;
    template<int N> using makeIndexSequence = InvokeGenSeq<GenSeq<N>>;

    template<int N>
    struct GenSeq : ConcatSeq<N/2, makeIndexSequence<N/2>, makeIndexSequence<N - N/2>>{};

    template<> struct GenSeq<0> : IndexesList<>{};
    template<> struct GenSeq<1> : IndexesList<0>{};

    template<int N>
    struct Indexes { using Value = makeIndexSequence<N>; };

    template<typename Func> struct FunctionPointer { enum {ArgumentCount = -1, IsPointerToMemberFunction = false}; };

    template<typename ObjPrivate> inline void assertObjectType(QObjectPrivate *d);
    template<typename Obj> inline void assertObjectType(QObject *o)
    {
        // ensure all three compile
        [[maybe_unused]] auto staticcast = [](QObject *obj) { return static_cast<Obj *>(obj); };
        [[maybe_unused]] auto qobjcast = [](QObject *obj) { return Obj::staticMetaObject.cast(obj); };
#ifdef __cpp_rtti
        [[maybe_unused]] auto dyncast = [](QObject *obj) { return dynamic_cast<Obj *>(obj); };
        auto cast = dyncast;
#else
        auto cast = qobjcast;
#endif
        Q_ASSERT_X(cast(o), Obj::staticMetaObject.className(),
                   "Called object is not of the correct type (class destructor may have already run)");
    }

    template <typename, typename, typename, typename> struct FunctorCall;
    template <int... II, typename... SignalArgs, typename R, typename Function>
    struct FunctorCall<IndexesList<II...>, List<SignalArgs...>, R, Function> {
        static void call(Function &f, void **arg) {
            f((*reinterpret_cast<typename RemoveRef<SignalArgs>::Type *>(arg[II+1]))...), ApplyReturnValue<R>(arg[0]);
        }
    };
    template <int... II, typename... SignalArgs, typename R, typename... SlotArgs, typename SlotRet, class Obj>
    struct FunctorCall<IndexesList<II...>, List<SignalArgs...>, R, SlotRet (Obj::*)(SlotArgs...)> {
        static void call(SlotRet (Obj::*f)(SlotArgs...), Obj *o, void **arg)
        {
            assertObjectType<Obj>(o);
            (o->*f)((*reinterpret_cast<typename RemoveRef<SignalArgs>::Type *>(arg[II+1]))...), ApplyReturnValue<R>(arg[0]);
        }
    };
    template <int... II, typename... SignalArgs, typename R, typename... SlotArgs, typename SlotRet, class Obj>
    struct FunctorCall<IndexesList<II...>, List<SignalArgs...>, R, SlotRet (Obj::*)(SlotArgs...) const> {
        static void call(SlotRet (Obj::*f)(SlotArgs...) const, Obj *o, void **arg)
        {
            assertObjectType<Obj>(o);
            (o->*f)((*reinterpret_cast<typename RemoveRef<SignalArgs>::Type *>(arg[II+1]))...), ApplyReturnValue<R>(arg[0]);
        }
    };
    template <int... II, typename... SignalArgs, typename R, typename... SlotArgs, typename SlotRet, class Obj>
    struct FunctorCall<IndexesList<II...>, List<SignalArgs...>, R, SlotRet (Obj::*)(SlotArgs...) noexcept> {
        static void call(SlotRet (Obj::*f)(SlotArgs...) noexcept, Obj *o, void **arg)
        {
            assertObjectType<Obj>(o);
            (o->*f)((*reinterpret_cast<typename RemoveRef<SignalArgs>::Type *>(arg[II+1]))...), ApplyReturnValue<R>(arg[0]);
        }
    };
    template <int... II, typename... SignalArgs, typename R, typename... SlotArgs, typename SlotRet, class Obj>
    struct FunctorCall<IndexesList<II...>, List<SignalArgs...>, R, SlotRet (Obj::*)(SlotArgs...) const noexcept> {
        static void call(SlotRet (Obj::*f)(SlotArgs...) const noexcept, Obj *o, void **arg)
        {
            assertObjectType<Obj>(o);
            (o->*f)((*reinterpret_cast<typename RemoveRef<SignalArgs>::Type *>(arg[II+1]))...), ApplyReturnValue<R>(arg[0]);
        }
    };

    template<class Obj, typename Ret, typename... Args> struct FunctionPointer<Ret (Obj::*) (Args...)>
    {
        typedef Obj Object;
        typedef List<Args...>  Arguments;
        typedef Ret ReturnType;
        typedef Ret (Obj::*Function) (Args...);
        enum {ArgumentCount = sizeof...(Args), IsPointerToMemberFunction = true};
        template <typename SignalArgs, typename R>
        static void call(Function f, Obj *o, void **arg) {
            FunctorCall<typename Indexes<ArgumentCount>::Value, SignalArgs, R, Function>::call(f, o, arg);
        }
    };
    template<class Obj, typename Ret, typename... Args> struct FunctionPointer<Ret (Obj::*) (Args...) const>
    {
        typedef Obj Object;
        typedef List<Args...>  Arguments;
        typedef Ret ReturnType;
        typedef Ret (Obj::*Function) (Args...) const;
        enum {ArgumentCount = sizeof...(Args), IsPointerToMemberFunction = true};
        template <typename SignalArgs, typename R>
        static void call(Function f, Obj *o, void **arg) {
            FunctorCall<typename Indexes<ArgumentCount>::Value, SignalArgs, R, Function>::call(f, o, arg);
        }
    };

    template<typename Ret, typename... Args> struct FunctionPointer<Ret (*) (Args...)>
    {
        typedef List<Args...> Arguments;
        typedef Ret ReturnType;
        typedef Ret (*Function) (Args...);
        enum {ArgumentCount = sizeof...(Args), IsPointerToMemberFunction = false};
        template <typename SignalArgs, typename R>
        static void call(Function f, void *, void **arg) {
            FunctorCall<typename Indexes<ArgumentCount>::Value, SignalArgs, R, Function>::call(f, arg);
        }
    };

    template<class Obj, typename Ret, typename... Args> struct FunctionPointer<Ret (Obj::*) (Args...) noexcept>
    {
        typedef Obj Object;
        typedef List<Args...>  Arguments;
        typedef Ret ReturnType;
        typedef Ret (Obj::*Function) (Args...) noexcept;
        enum {ArgumentCount = sizeof...(Args), IsPointerToMemberFunction = true};
        template <typename SignalArgs, typename R>
        static void call(Function f, Obj *o, void **arg) {
            FunctorCall<typename Indexes<ArgumentCount>::Value, SignalArgs, R, Function>::call(f, o, arg);
        }
    };
    template<class Obj, typename Ret, typename... Args> struct FunctionPointer<Ret (Obj::*) (Args...) const noexcept>
    {
        typedef Obj Object;
        typedef List<Args...>  Arguments;
        typedef Ret ReturnType;
        typedef Ret (Obj::*Function) (Args...) const noexcept;
        enum {ArgumentCount = sizeof...(Args), IsPointerToMemberFunction = true};
        template <typename SignalArgs, typename R>
        static void call(Function f, Obj *o, void **arg) {
            FunctorCall<typename Indexes<ArgumentCount>::Value, SignalArgs, R, Function>::call(f, o, arg);
        }
    };

    template<typename Ret, typename... Args> struct FunctionPointer<Ret (*) (Args...) noexcept>
    {
        typedef List<Args...> Arguments;
        typedef Ret ReturnType;
        typedef Ret (*Function) (Args...) noexcept;
        enum {ArgumentCount = sizeof...(Args), IsPointerToMemberFunction = false};
        template <typename SignalArgs, typename R>
        static void call(Function f, void *, void **arg) {
            FunctorCall<typename Indexes<ArgumentCount>::Value, SignalArgs, R, Function>::call(f, arg);
        }
    };

    template<typename Function, int N> struct Functor
    {
        template <typename SignalArgs, typename R>
        static void call(Function &f, void *, void **arg) {
            FunctorCall<typename Indexes<N>::Value, SignalArgs, R, Function>::call(f, arg);
        }
    };

    // Traits to detect if there is a conversion between two types,
    // and that conversion does not include a narrowing conversion.
    template <typename T>
    struct NarrowingDetector { T t[1]; }; // from P0608

    template <typename From, typename To, typename Enable = void>
    struct IsConvertibleWithoutNarrowing : std::false_type {};

    template <typename From, typename To>
    struct IsConvertibleWithoutNarrowing<From, To,
            std::void_t< decltype( NarrowingDetector<To>{ {std::declval<From>()} } ) >
        > : std::true_type {};

    // Check for the actual arguments. If they are exactly the same,
    // then don't bother checking for narrowing; as a by-product,
    // this solves the problem of incomplete types (which must be supported,
    // or they would error out in the trait above).
    template <typename From, typename To, typename Enable = void>
    struct AreArgumentsConvertibleWithoutNarrowingBase : std::false_type {};

    template <typename From, typename To>
    struct AreArgumentsConvertibleWithoutNarrowingBase<From, To,
        std::enable_if_t<
            std::disjunction_v<std::is_same<From, To>, IsConvertibleWithoutNarrowing<From, To>>
        >
    > : std::true_type {};

    /*
       Logic that check if the arguments of the slot matches the argument of the signal.
       To be used like this:
       static_assert(CheckCompatibleArguments<FunctionPointer<Signal>::Arguments, FunctionPointer<Slot>::Arguments>::value)
    */
    template<typename A1, typename A2> struct AreArgumentsCompatible {
        static int test(const typename RemoveRef<A2>::Type&);
        static char test(...);
        static const typename RemoveRef<A1>::Type &dummy();
        enum { value = sizeof(test(dummy())) == sizeof(int) };
#ifdef QT_NO_NARROWING_CONVERSIONS_IN_CONNECT
        using AreArgumentsConvertibleWithoutNarrowing = AreArgumentsConvertibleWithoutNarrowingBase<std::decay_t<A1>, std::decay_t<A2>>;
        static_assert(AreArgumentsConvertibleWithoutNarrowing::value, "Signal and slot arguments are not compatible (narrowing)");
#endif
    };
    template<typename A1, typename A2> struct AreArgumentsCompatible<A1, A2&> { enum { value = false }; };
    template<typename A> struct AreArgumentsCompatible<A&, A&> { enum { value = true }; };
    // void as a return value
    template<typename A> struct AreArgumentsCompatible<void, A> { enum { value = true }; };
    template<typename A> struct AreArgumentsCompatible<A, void> { enum { value = true }; };
    template<> struct AreArgumentsCompatible<void, void> { enum { value = true }; };

    template <typename List1, typename List2> struct CheckCompatibleArguments { enum { value = false }; };
    template <> struct CheckCompatibleArguments<List<>, List<>> { enum { value = true }; };
    template <typename List1> struct CheckCompatibleArguments<List1, List<>> { enum { value = true }; };
    template <typename Arg1, typename Arg2, typename... Tail1, typename... Tail2>
    struct CheckCompatibleArguments<List<Arg1, Tail1...>, List<Arg2, Tail2...>>
    {
        enum { value = AreArgumentsCompatible<typename RemoveConstRef<Arg1>::Type, typename RemoveConstRef<Arg2>::Type>::value
                    && CheckCompatibleArguments<List<Tail1...>, List<Tail2...>>::value };
    };

    /*
       Find the maximum number of arguments a functor object can take and be still compatible with
       the arguments from the signal.
       Value is the number of arguments, or -1 if nothing matches.
     */
    template <typename Functor, typename ArgList> struct ComputeFunctorArgumentCount;

    template <typename Functor, typename ArgList, bool Done> struct ComputeFunctorArgumentCountHelper
    { enum { Value = -1 }; };
    template <typename Functor, typename First, typename... ArgList>
    struct ComputeFunctorArgumentCountHelper<Functor, List<First, ArgList...>, false>
        : ComputeFunctorArgumentCount<Functor,
            typename List_Left<List<First, ArgList...>, sizeof...(ArgList)>::Value> {};

    template <typename Functor, typename... ArgList> struct ComputeFunctorArgumentCount<Functor, List<ArgList...>>
    {
        template <typename D> static D dummy();
        template <typename F> static auto test(F f) -> decltype(((f.operator()((dummy<ArgList>())...)), int()));
        static char test(...);
        enum {
            Ok = sizeof(test(dummy<Functor>())) == sizeof(int),
            Value = Ok ? int(sizeof...(ArgList)) : int(ComputeFunctorArgumentCountHelper<Functor, List<ArgList...>, Ok>::Value)
        };
    };

    /* get the return type of a functor, given the signal argument list  */
    template <typename Functor, typename ArgList> struct FunctorReturnType;
    template <typename Functor, typename ... ArgList> struct FunctorReturnType<Functor, List<ArgList...>> {
        template <typename D> static D dummy();
        typedef decltype(dummy<Functor>().operator()((dummy<ArgList>())...)) Value;
    };

    // internal base class (interface) containing functions required to call a slot managed by a pointer to function.
    class QSlotObjectBase {
        QAtomicInt m_ref;
        // don't use virtual functions here; we don't want the
        // compiler to create tons of per-polymorphic-class stuff that
        // we'll never need. We just use one function pointer, and the
        // Operations enum below to distinguish requests
        typedef void (*ImplFn)(int which, QSlotObjectBase* this_, QObject *receiver, void **args, bool *ret);
        const ImplFn m_impl;
    protected:
        // The operations that can be requested by calls to m_impl,
        // see the member functions that call m_impl below for details
        enum Operation {
            Destroy,
            Call,
            Compare,

            NumOperations
        };
    public:
        explicit QSlotObjectBase(ImplFn fn) : m_ref(1), m_impl(fn) {}

        inline int ref() noexcept { return m_ref.ref(); }
        inline void destroyIfLastRef() noexcept
        { if (!m_ref.deref()) m_impl(Destroy, this, nullptr, nullptr, nullptr); }

        inline bool compare(void **a) { bool ret = false; m_impl(Compare, this, nullptr, a, &ret); return ret; }
        inline void call(QObject *r, void **a)  { m_impl(Call,    this, r, a, nullptr); }
    protected:
        ~QSlotObjectBase() {}
    private:
        Q_DISABLE_COPY_MOVE(QSlotObjectBase)
    };

    // implementation of QSlotObjectBase for which the slot is a pointer to member function of a QObject
    // Args and R are the List of arguments and the return type of the signal to which the slot is connected.
    template<typename Func, typename Args, typename R> class QSlotObject : public QSlotObjectBase
    {
        typedef QtPrivate::FunctionPointer<Func> FuncType;
        Func function;
        static void impl(int which, QSlotObjectBase *this_, QObject *r, void **a, bool *ret)
        {
            switch (which) {
            case Destroy:
                delete static_cast<QSlotObject*>(this_);
                break;
            case Call:
                FuncType::template call<Args, R>(static_cast<QSlotObject*>(this_)->function, static_cast<typename FuncType::Object *>(r), a);
                break;
            case Compare:
                *ret = *reinterpret_cast<Func *>(a) == static_cast<QSlotObject*>(this_)->function;
                break;
            case NumOperations: ;
            }
        }
    public:
        explicit QSlotObject(Func f) : QSlotObjectBase(&impl), function(f) {}
    };
    // implementation of QSlotObjectBase for which the slot is a functor (or lambda)
    // N is the number of arguments
    // Args and R are the List of arguments and the return type of the signal to which the slot is connected.
    template<typename Func, int N, typename Args, typename R> class QFunctorSlotObject : public QSlotObjectBase
    {
        typedef QtPrivate::Functor<Func, N> FuncType;
        Func function;
        static void impl(int which, QSlotObjectBase *this_, QObject *r, void **a, bool *ret)
        {
            switch (which) {
            case Destroy:
                delete static_cast<QFunctorSlotObject*>(this_);
                break;
            case Call:
                FuncType::template call<Args, R>(static_cast<QFunctorSlotObject*>(this_)->function, r, a);
                break;
            case Compare: // not implemented
            case NumOperations:
                Q_UNUSED(ret);
            }
        }
    public:
        explicit QFunctorSlotObject(Func f) : QSlotObjectBase(&impl), function(std::move(f)) {}
    };

    // typedefs for readability for when there are no parameters
    template <typename Func>
    using QSlotObjectWithNoArgs = QSlotObject<Func,
                                              QtPrivate::List<>,
                                              typename QtPrivate::FunctionPointer<Func>::ReturnType>;

    template <typename Func, typename R>
    using QFunctorSlotObjectWithNoArgs = QFunctorSlotObject<Func, 0, QtPrivate::List<>, R>;

    template <typename Func>
    using QFunctorSlotObjectWithNoArgsImplicitReturn = QFunctorSlotObjectWithNoArgs<Func, typename QtPrivate::FunctionPointer<Func>::ReturnType>;
}

QT_END_NAMESPACE

