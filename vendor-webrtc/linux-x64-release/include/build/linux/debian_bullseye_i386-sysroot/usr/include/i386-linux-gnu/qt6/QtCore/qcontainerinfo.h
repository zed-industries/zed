// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCONTAINERINFO_H
#define QCONTAINERINFO_H

#include <QtCore/qglobal.h>
#include <type_traits>

QT_BEGIN_NAMESPACE

namespace QContainerInfo {

template<typename C>
using value_type = typename C::value_type;

template<typename C>
using key_type = typename C::key_type;

template<typename C>
using mapped_type = typename C::mapped_type;

template<typename C>
using iterator = typename C::iterator;

template<typename C>
using const_iterator = typename C::const_iterator;

// Some versions of Apple clang warn about the constexpr variables below being unused.
QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wunused-const-variable")

template<typename C, typename = void>
inline constexpr bool has_value_type_v = false;
template<typename C>
inline constexpr bool has_value_type_v<C, std::void_t<value_type<C>>> = true;

template<typename C, typename = void>
inline constexpr bool has_key_type_v = false;
template<typename C>
inline constexpr bool has_key_type_v<C, std::void_t<key_type<C>>> = true;

template<typename C, typename = void>
inline constexpr bool has_mapped_type_v = false;
template<typename C>
inline constexpr bool has_mapped_type_v<C, std::void_t<mapped_type<C>>> = true;

template<typename C, typename = void>
inline constexpr bool has_size_v = false;
template<typename C>
inline constexpr bool has_size_v<C, std::void_t<decltype(C().size())>> = true;

template<typename C, typename = void>
inline constexpr bool has_clear_v = false;
template<typename C>
inline constexpr bool has_clear_v<C, std::void_t<decltype(C().clear())>> = true;

template<typename, typename = void>
inline constexpr bool has_at_index_v = false;
template<typename C>
inline constexpr bool has_at_index_v<C, std::void_t<decltype(C().at(0))>> = true;

template<typename, typename = void>
inline constexpr bool has_at_key_v = false;
template<typename C>
inline constexpr bool has_at_key_v<C, std::void_t<decltype(C().at(key_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool can_get_at_index_v = false;
template<typename C>
inline constexpr bool can_get_at_index_v<C, std::void_t<value_type<C>(decltype(C()[0]))>> = true;

template<typename, typename = void>
inline constexpr bool can_set_at_index_v = false;
template<typename C>
inline constexpr bool can_set_at_index_v<C, std::void_t<decltype(C()[0] = value_type<C>())>> = true;

template<typename, typename = void>
inline constexpr bool has_push_front_v = false;
template<typename C>
inline constexpr bool has_push_front_v<C, std::void_t<decltype(C().push_front(value_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool has_push_back_v = false;
template<typename C>
inline constexpr bool has_push_back_v<C, std::void_t<decltype(C().push_back(value_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool has_insert_v = false;
template<typename C>
inline constexpr bool has_insert_v<C, std::void_t<decltype(C().insert(value_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool has_pop_front_v = false;
template<typename C>
inline constexpr bool has_pop_front_v<C, std::void_t<decltype(C().pop_front())>> = true;

template<typename, typename = void>
inline constexpr bool has_pop_back_v = false;
template<typename C>
inline constexpr bool has_pop_back_v<C, std::void_t<decltype(C().pop_back())>> = true;

template<typename, typename = void>
inline constexpr bool has_iterator_v = false;
template<typename C>
inline constexpr bool has_iterator_v<C, std::void_t<iterator<C>>> = true;

template<typename, typename = void>
inline constexpr bool has_const_iterator_v = false;
template<typename C>
inline constexpr bool has_const_iterator_v<C, std::void_t<const_iterator<C>>> = true;

template<typename, typename = void>
inline constexpr bool can_set_value_at_iterator_v = false;
template<typename C>
inline constexpr bool can_set_value_at_iterator_v<C, std::void_t<decltype(*C().begin() = value_type<C>())>> = true;

template<typename, typename = void>
inline constexpr bool can_set_mapped_at_iterator_v = false;
template<typename C>
inline constexpr bool can_set_mapped_at_iterator_v<C, std::void_t<decltype(*C().begin() = mapped_type<C>())>> = true;

template<typename, typename = void>
inline constexpr bool can_insert_value_at_iterator_v = false;
template<typename C>
inline constexpr bool can_insert_value_at_iterator_v<C, std::void_t<decltype(C().insert(C().begin(), value_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool can_erase_at_iterator_v = false;
template<typename C>
inline constexpr bool can_erase_at_iterator_v<C, std::void_t<decltype(C().erase(C().begin()))>> = true;

template<typename, typename = void>
inline constexpr bool can_erase_range_at_iterator_v = false;
template<typename C>
inline constexpr bool can_erase_range_at_iterator_v<C, std::void_t<decltype(C().erase(C().begin(), C().end()))>> = true;

template<typename, typename = void>
inline constexpr bool can_get_at_key_v = false;
template<typename C>
inline constexpr bool can_get_at_key_v<C, std::void_t<mapped_type<C>(decltype(C()[key_type<C>()]))>> = true;

template<typename, typename = void>
inline constexpr bool can_set_at_key_v = false;
template<typename C>
inline constexpr bool can_set_at_key_v<C, std::void_t<decltype(C()[key_type<C>()] = mapped_type<C>())>> = true;

template<typename, typename = void>
inline constexpr bool can_erase_at_key_v = false;
template<typename C>
inline constexpr bool can_erase_at_key_v<C, std::void_t<decltype(C().erase(key_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool can_remove_at_key_v = false;
template<typename C>
inline constexpr bool can_remove_at_key_v<C, std::void_t<decltype(C().remove(key_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool can_insert_key_v = false;
template<typename C>
inline constexpr bool can_insert_key_v<C, std::void_t<decltype(C().insert(key_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool can_insert_pair_v = false;
template<typename C>
inline constexpr bool can_insert_pair_v<C, std::void_t<decltype(C().insert({key_type<C>(), mapped_type<C>()}))>> = true;

template<typename, typename = void>
inline constexpr bool can_insert_key_mapped_v = false;
template<typename C>
inline constexpr bool can_insert_key_mapped_v<C, std::void_t<decltype(C().insert(key_type<C>(), mapped_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool has_contains_v = false;
template<typename C>
inline constexpr bool has_contains_v<C, std::void_t<decltype(bool(C().contains(key_type<C>())))>> = true;

template<typename, typename = void>
inline constexpr bool has_find_v = false;
template<typename C>
inline constexpr bool has_find_v<C, std::void_t<decltype(C().find(key_type<C>()))>> = true;

template<typename, typename = void>
inline constexpr bool iterator_dereferences_to_value_v = false;
template<typename C>
inline constexpr bool iterator_dereferences_to_value_v<C, std::void_t<decltype(value_type<C>(*C().begin()))>> = true;

template<typename, typename = void>
inline constexpr bool iterator_has_key_v = false;
template<typename C>
inline constexpr bool iterator_has_key_v<C, std::void_t<decltype(key_type<C>(C().begin().key()))>> = true;

template<typename, typename = void>
inline constexpr bool value_type_has_first_v = false;
template<typename C>
inline constexpr bool value_type_has_first_v<C, std::void_t<decltype(key_type<C>(value_type<C>().first))>> = true;

template<typename, typename = void>
inline constexpr bool iterator_dereferences_to_key_v = false;
template<typename C>
inline constexpr bool iterator_dereferences_to_key_v<C, std::void_t<decltype(key_type<C>(*C().begin()))>> = true;

template<typename, typename = void>
inline constexpr bool iterator_has_value_v = false;
template<typename C>
inline constexpr bool iterator_has_value_v<C, std::void_t<decltype(mapped_type<C>(C().begin().value()))>> = true;

template<typename, typename = void>
inline constexpr bool value_type_has_second_v = false;
template<typename C>
inline constexpr bool value_type_has_second_v<C, std::void_t<decltype(mapped_type<C>(value_type<C>().second))>> = true;

template<typename, typename = void>
inline constexpr bool iterator_dereferences_to_mapped_v = false;
template<typename C>
inline constexpr bool iterator_dereferences_to_mapped_v<C, std::void_t<decltype(mapped_type<C>(*C().begin()))>> = true;

QT_WARNING_POP

}

QT_END_NAMESPACE

#endif // QCONTAINERINFO_H
