// Copyright (C) 2022 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#ifndef QXPFUNCTIONAL_H
#define QXPFUNCTIONAL_H

#include <QtCore/qglobal.h>

//
//  W A R N I N G
//  -------------
//
// This file is not part of the Qt API. Types and functions defined in this
// file can reliably be replaced by their std counterparts, once available.
// You may use these definitions in your own code, but be aware that we
// will remove them once Qt depends on the C++ version that supports
// them in namespace std. There will be NO deprecation warning, the
// definitions will JUST go away.
//
// If you can't agree to these terms, don't use these definitions!
//
// We mean it.
//

#include <QtCore/q23functional.h>
#include <type_traits>
#include <utility>

QT_BEGIN_NAMESPACE

namespace qxp {
// like P0792r9's function_ref:

// [func.wrap.ref], non-owning wrapper
template<class... S> class function_ref;              // not defined

// template<class R, class... ArgTypes>
// class function_ref<R(ArgTypes...) cv noexcept(noex)>; // see below
//
// [func.wrap.ref.general]
// The header provides partial specializations of function_ref for each combination
// of the possible replacements of the placeholders cv and noex where:
// - cv is either const or empty.
// - noex is either true or false.

namespace detail {

template <typename T>
using if_function = std::enable_if_t<std::is_function_v<T>, bool>;
template <typename T>
using if_non_function = std::enable_if_t<!std::is_function_v<T>, bool>;

template <typename From, typename To>
using copy_const_t = std::conditional_t<
        std::is_const_v<From>,
        std::add_const_t<To>,
        To
    >;

template <class Const>
union BoundEntityType {
    template <typename F, if_function<F> = true>
    explicit constexpr BoundEntityType(F *f)
        : fun(reinterpret_cast<QFunctionPointer>(f)) {}
    template <typename T, if_non_function<T> = true>
    explicit constexpr BoundEntityType(T *t)
        : obj(static_cast<Const*>(t)) {}
    Const *obj;
    QFunctionPointer fun;
};

template <bool noex, class Const, class R, class... ArgTypes>
class function_ref_base
{
protected:
    ~function_ref_base() = default;

    using BoundEntityType = detail::BoundEntityType<Const>;

    template <typename... Ts>
    using is_invocable_using = std::conditional_t<
            noex,
            std::is_nothrow_invocable_r<R, Ts..., ArgTypes...>,
            std::is_invocable_r<R, Ts..., ArgTypes...>
        >;

    using ThunkPtr = R(*)(BoundEntityType, ArgTypes&&...) noexcept(noex);

    BoundEntityType m_bound_entity;
    ThunkPtr m_thunk_ptr;

public:
    template<
        class F,
        std::enable_if_t<std::conjunction_v<
            std::is_function<F>,
            is_invocable_using<F>
        >, bool> = true
    >
    Q_IMPLICIT function_ref_base(F* f) noexcept
        : m_bound_entity(f),
          m_thunk_ptr([](BoundEntityType ctx, ArgTypes&&... args) noexcept(noex) -> R {
                return q23::invoke_r<R>(reinterpret_cast<F*>(ctx.fun),
                                        std::forward<ArgTypes>(args)...);
            })
    {}

    template<
        class F,
        std::enable_if_t<std::conjunction_v<
            std::negation<std::is_same<q20::remove_cvref_t<F>, function_ref_base>>,
            std::negation<std::is_member_pointer<std::remove_reference_t<F>>>,
            is_invocable_using<copy_const_t<Const, std::remove_reference_t<F>>&>
        >, bool> = true
    >
    Q_IMPLICIT constexpr function_ref_base(F&& f) noexcept
        : m_bound_entity(std::addressof(f)),
          m_thunk_ptr([](BoundEntityType ctx, ArgTypes&&... args) noexcept(noex) -> R {
                using That = copy_const_t<Const, std::remove_reference_t<F>>;
                return q23::invoke_r<R>(*static_cast<That*>(ctx.obj),
                                        std::forward<ArgTypes>(args)...);
            })
    {}

protected:
    template <
        class T,
        std::enable_if_t<std::conjunction_v<
            std::negation<std::is_same<q20::remove_cvref_t<T>, function_ref_base>>,
            std::negation<std::is_pointer<T>>
        >, bool> = true
    >
    function_ref_base& operator=(T) = delete;

    // Invocation [func.wrap.ref.inv]
    R operator()(ArgTypes... args) const noexcept(noex)
    {
        return m_thunk_ptr(m_bound_entity, std::forward<ArgTypes>(args)...);
    }

};

} // namespace detail

#define QT_SPECIALIZE_FUNCTION_REF(cv, noex) \
    template<class R, class... ArgTypes> \
    class function_ref<R(ArgTypes...) cv noexcept( noex )> \
        : private detail::function_ref_base< noex , cv void, R, ArgTypes...> \
    { \
        using base = detail::function_ref_base< noex , cv void, R, ArgTypes...>; \
        \
    public: \
        using base::base; \
        using base::operator(); \
    } \
    /* end */

QT_SPECIALIZE_FUNCTION_REF(     , false);
QT_SPECIALIZE_FUNCTION_REF(const, false);
QT_SPECIALIZE_FUNCTION_REF(     , true );
QT_SPECIALIZE_FUNCTION_REF(const, true );

#undef QT_SPECIALIZE_FUNCTION_REF

// deduction guides [func.wrap.ref.deduct]

template <
    class F,
    std::enable_if_t<std::is_function_v<F>, bool> = true
>
function_ref(F*) -> function_ref<F>;

} // namespace qxp

QT_END_NAMESPACE

#endif /* QXPFUNCTIONAL_H */
